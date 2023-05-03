use core::fmt;
use std::collections::HashMap;
use rust_decimal::{Decimal, RoundingStrategy};
use rust_decimal_macros::dec;
use crate::{orderbook::OrderBook, log};
use serde::{Serialize, Deserialize};
use crate::clg;

/// Calculate max quantity, min quantity, entry_price, liquidation_price, fees and slippage for a given order

#[derive(Serialize, Deserialize, Clone)]
pub enum OrderType {
    Limit,
    Market,
}

impl fmt::Debug for OrderType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            OrderType::Limit => write!(f, "Limit"),
            OrderType::Market => write!(f, "Market"),
        }
    }
}


#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct FuturesOrder {
    pub entry_price: Decimal,
    pub liquidation_price: Decimal,
    pub max_quantity_base: Decimal,
    pub min_quantity_base: Decimal,
    pub max_quantity_quote: Decimal,
    pub min_quantity_quote: Decimal,
    pub fees: Decimal,
    pub swap_fee: Decimal,
    pub slippage: Decimal,
    pub cost_long: Decimal,
    pub cost_long_base: Decimal,
    pub cost_short: Decimal,
    pub open_quantity: Decimal,
    pub open_notional: Decimal,
}

impl FuturesOrder {
    pub fn empty() -> Self {
        Self {
            entry_price: Decimal::ZERO,
            liquidation_price: Decimal::ZERO,
            max_quantity_base: Decimal::ZERO,
            min_quantity_base: Decimal::ZERO,
            max_quantity_quote: Decimal::ZERO,
            min_quantity_quote: Decimal::ZERO,
            fees: Decimal::ZERO,
            swap_fee: Decimal::ZERO,
            slippage: Decimal::ZERO,
            cost_long: Decimal::ZERO,
            cost_long_base: Decimal::ZERO,
            cost_short: Decimal::ZERO,
            open_quantity: Decimal::ZERO,
            open_notional: Decimal::ZERO,
        }
    }
    pub fn empty2(entry_price: Decimal) -> Self {
        Self {
            entry_price,
            liquidation_price: Decimal::ZERO,
            max_quantity_base: Decimal::ZERO,
            min_quantity_base: Decimal::ZERO,
            max_quantity_quote: Decimal::ZERO,
            min_quantity_quote: Decimal::ZERO,
            fees: Decimal::ZERO,
            swap_fee: Decimal::ZERO,
            slippage: Decimal::ZERO,
            cost_long: Decimal::ZERO,
            cost_long_base: Decimal::ZERO,
            cost_short: Decimal::ZERO,
            open_quantity: Decimal::ZERO,
            open_notional: Decimal::ZERO,
        }
    }
}

#[derive(Debug, Clone, Default)]
pub struct FuturesOrderCalculation {
    pub leverage: Decimal,
    // configuration
    pub collateral_long_token: String,
    pub collateral_short_token: String,

    pub max_notional: Decimal,
    pub min_quantity_base: Decimal,
    pub margin_ratio: Decimal,
    pub taker_fee: Decimal,
    pub maker_fee: Decimal,
    pub base_token_precision: u32,
}

pub fn string_to_decimal(s: &str, expect_msg: &str) -> Decimal {
    Decimal::from_str_exact(s).expect(expect_msg)
}

impl FuturesOrderCalculation {
    pub fn new(
        collateral_long_token: String,
        collateral_short_token: String,
        leverage: String,
        max_notional: String,
        min_quantity_base: String,
        margin_ratio: String,
        taker_fee: String,
        maker_fee: String,
        base_token_precision: u32,
    ) -> Self {
        Self {
            leverage: string_to_decimal(&leverage, "Invalid leverage"),
            collateral_long_token,
            collateral_short_token,
            max_notional: string_to_decimal(&max_notional, "Invalid max notional"),
            min_quantity_base: string_to_decimal(&min_quantity_base, "Invalid min quantity base"),
            margin_ratio: string_to_decimal(&margin_ratio, "Invalid margin ratio"),
            taker_fee: string_to_decimal(&taker_fee, "Invalid taker fee"),
            maker_fee: string_to_decimal(&maker_fee, "Invalid maker fee"),
            base_token_precision,
        }
    }

    /// Compute open order details
    /// If pay_amount = 0, quantity > 0 => use quantity to calculate
    /// if pay_amount > 0, quantity = 0 => use pay_amount to calculate
    /// If pay_amount > 0, quantity > 0 => prefer quantity over pay_amount
    /// If both = 0, invalid
    /// Note pay_amount should be in USD
    pub fn compute_open_order(
        &self,
        order_type: OrderType,
        order_book: &OrderBook,
        balance: Decimal,
        pay_amount: Decimal,
        quantity: Decimal,
        limit_price: Option<Decimal>,
        is_quote: bool,
        is_buy: bool,
        use_percentage: bool,
    ) -> anyhow::Result<FuturesOrder> {
        assert!(self.leverage != Decimal::ZERO, "Leverage not set. Must init new pare first");
        assert!(self.max_notional != Decimal::ZERO, "Max notional not set. Must init new pare first");
        assert!(!String::is_empty(&self.collateral_long_token), "Long collateral token not set. Must init new pare first");
        assert!(!String::is_empty(&self.collateral_short_token), "Short collateral token not set. Must init new pare first");
        crate::clg!("pay amount {:?}, quantity {}", pay_amount, quantity);
        assert(pay_amount > Decimal::ZERO || quantity > Decimal::ZERO, "Must have positive pay_amount or quantity".to_string()).unwrap();

        let zero = Decimal::ZERO;
        // let balance = self.account_balance.get(&pay_token).unwrap_or(&zero);
        // TODO Convert balance to quote balance
        let quote_balance = balance;
        let mut quantity = quantity;
        let mut is_quote = is_quote;
        if quantity == zero && pay_amount > zero {
            // todo use pay_amount to calculate the quantity
            quantity = pay_amount * self.leverage;
            // should auto be quote
            is_quote = true;
        }

        // Convert the percentage quantity to an absolute value if necessary
        quantity = if use_percentage {
            (balance * self.leverage) * quantity
        } else {
            quantity
        };

        let (entry_price, total_base_filled, slippage) = match order_type {
            OrderType::Market => order_book.compute_dry(quantity, is_quote, is_buy),
            OrderType::Limit => {
                // For limit orders, use the provided quantity and slippage
                (limit_price.unwrap(), if is_quote {quantity / limit_price.unwrap()} else {quantity}, dec!(0))
            }
        };
        let total_base_filled = total_base_filled.round_dp_with_strategy(self.base_token_precision, RoundingStrategy::ToZero);

        if entry_price.is_zero() || total_base_filled.is_zero() {
            clg!("Entry price or total base filled is zero entry_price: {}, total_base_filled: {}", entry_price, total_base_filled);
            return Ok(
                FuturesOrder::empty2(entry_price)
            );
        }

        let open_notional = total_base_filled * entry_price;

        let open_fees_rate = match order_type {
            OrderType::Limit => self.maker_fee,
            OrderType::Market => self.taker_fee,
        };

        let open_fee = open_fees_rate * open_notional;


        let swap_fee = dec!(0); // TODO: Calculate swap fee

        let max_balance = quote_balance * (dec!(1) - open_fees_rate);

        let initial_margin = self.compute_margin(total_base_filled, entry_price);
        let maintenance_margin = initial_margin * self.margin_ratio;
        clg!("initial_margin: {}, maintenance_margin: {}, quantity: {}, total_base_filled: {}", initial_margin, maintenance_margin, quantity, total_base_filled);
        let liquidation_price = match is_buy {
            true => (maintenance_margin - initial_margin + open_notional) / total_base_filled,
            false => (open_notional - maintenance_margin + initial_margin) / total_base_filled,
        };

        // Calculate min, max
        // Max = min (max_notional / entry_price, max_balance * leverage / entry_price)
        let max_quantity_base = if max_balance > zero {
            (self.max_notional / entry_price).min(max_balance * self.leverage / entry_price)
        }else {
            self.max_notional / entry_price
        };

        let max_quantity_base = max_quantity_base.round_dp_with_strategy(self.base_token_precision, RoundingStrategy::ToZero);

        // Min = min_quantity
        let min_quantity_base = self.min_quantity_base;

        let max_quantity_quote = self.max_notional;
        let min_quantity_quote = (min_quantity_base * entry_price).round_dp_with_strategy(self.base_token_precision, RoundingStrategy::ToZero);

        let fees = open_fee + swap_fee;
        let cost_long = initial_margin;
        let cost_long_base = initial_margin/entry_price;
        let cost_short = initial_margin;

        Ok(
        FuturesOrder {
            entry_price,
            liquidation_price,
            max_quantity_base,
            min_quantity_base,
            max_quantity_quote,
            min_quantity_quote,
            fees,
            swap_fee,
            slippage,
            cost_long,
            cost_long_base,
            cost_short,
            open_notional,
            open_quantity: total_base_filled
        }
        )
    }

    pub fn compute_margin(&self, quantity: Decimal, entry_price: Decimal) -> Decimal {
        quantity * entry_price / self.leverage
    }

    // pub fn update_account_balance(&mut self, balance: Decimal) {
    //     self.account_balance.insert(self.collateral_long_token.clone(), balance);
    // }

    pub fn change_leverage(&mut self, leverage: Decimal, max_notional: String) {
        self.leverage = leverage;
        self.max_notional = string_to_decimal(&max_notional, "Invalid max notional");
    }
}

fn assert(condition: bool, message: String) -> anyhow::Result<()> {
    if !condition {
        anyhow::bail!(message)
    }
    Ok(())
}


#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;
    use std::collections::HashMap;

    fn setup_account_balance() -> HashMap<String, Decimal> {
        let mut account_balance = HashMap::new();
        account_balance.insert("USDT".to_string(), Decimal::new(1000, 0));
        account_balance
    }

    fn setup_order_book() -> OrderBook {
        let mut ob = OrderBook::new();
        let asks = vec![
            (dec!(10000), dec!(1)),
            (dec!(10100), dec!(1)),
            (dec!(10200), dec!(1)),
            (dec!(10300), dec!(1)),
            (dec!(10400), dec!(1)),
        ];

        let bids = vec![
            (dec!(9900), dec!(1)),
            (dec!(9800), dec!(1)),
            (dec!(9700), dec!(1)),
            (dec!(9600), dec!(1)),
            (dec!(9500), dec!(1)),
        ];
        ob.initialize(asks, bids);
        ob
    }

    fn setup_futures_order_calculation() -> FuturesOrderCalculation {
        FuturesOrderCalculation {
            leverage: dec!(10),
            collateral_long_token: "USDT".to_string(),
            collateral_short_token: "USDT".to_string(),
            max_notional: dec!(50000),
            min_quantity_base: dec!(0.001),
            margin_ratio: dec!(0.03),
            taker_fee: dec!(0.001),
            maker_fee: dec!(0.0005),
        }
    }

    #[test]
    fn should_calculate_fine_from_pay_amount() {
        let order_book = setup_order_book();
        let mut futures_order_calculation = setup_futures_order_calculation();
        let account_balance = setup_account_balance();

        let result = futures_order_calculation.compute_open_order(
            OrderType::Market,
            &order_book,
            account_balance.get("USDT").unwrap().clone(),
            dec!(100),
            dec!(0),
            None,
            false,
            true,
            false,
        );
        assert_eq!(result.entry_price, dec!(10000));
        assert_eq!(result.liquidation_price, dec!(9030));
        assert_eq!(result.max_quantity_quote, dec!(9990.0));
        assert_eq!(result.max_quantity_base.round_dp(4), dec!(0.999));
        assert_eq!(result.min_quantity_base, dec!(0.001));
        assert_eq!(result.min_quantity_quote, dec!(10));
        assert_eq!(result.fees, dec!(1));
        assert_eq!(result.slippage, dec!(0));
        assert_eq!(result.cost_long, dec!(100));
        assert_eq!(result.cost_short, dec!(100));

    }

    #[test]
    fn test_market_order_buy() {
        let order_book = setup_order_book();
        let mut futures_order_calculation = setup_futures_order_calculation();
        let account_balance = setup_account_balance();

        let result = futures_order_calculation.compute_open_order(
            OrderType::Market,
            &order_book,
            account_balance.get("USDT").unwrap().clone(),
            dec!(0),
            dec!(0.1),
            None,
            false,
            true,
            false,
        );

        assert_eq!(result.entry_price, dec!(10000));
        assert_eq!(result.liquidation_price, dec!(9030));
        assert_eq!(result.max_quantity_quote, dec!(9990.0));
        assert_eq!(result.max_quantity_base.round_dp(4), dec!(0.999));
        assert_eq!(result.min_quantity_base, dec!(0.001));
        assert_eq!(result.min_quantity_quote, dec!(10));
        assert_eq!(result.fees, dec!(1));
        assert_eq!(result.slippage, dec!(0));
        assert_eq!(result.cost_long, dec!(100));
        assert_eq!(result.cost_short, dec!(100));
        futures_order_calculation.change_leverage(dec!(20));
        let result = futures_order_calculation.compute_open_order(
            OrderType::Market,
            &order_book,
            account_balance.get("USDT").unwrap().clone(),
            dec!(0),
            dec!(0.1),
            None,
            false,
            true,
            false,
        );

        assert_eq!(result.cost_long, dec!(50));
        assert_eq!(result.cost_short, dec!(50));
        assert_eq!(futures_order_calculation.leverage, dec!(20));
    }

    #[test]
    fn test_market_order_sell() {
        let order_calculation = setup_futures_order_calculation();
        let order_book = setup_order_book();

        let account_balance = setup_account_balance();
        let result = order_calculation.compute_open_order(
            OrderType::Market,
            &order_book,
            account_balance.get("USDT").unwrap().clone(),
            dec!(0),
            dec!(0.1),
            None,
            false,
            false,
            false,
        );

        assert_eq!(result.entry_price, dec!(9900));
        assert_eq!(result.liquidation_price, dec!(10860.3));
        assert_eq!(result.max_quantity_base, dec!(1.01010101));
        assert_eq!(result.min_quantity_base, dec!(0.001));
        assert_eq!(result.max_quantity_quote, dec!(9990.0));
        assert_eq!(result.min_quantity_quote, dec!(9.9));
        assert_eq!(result.fees, dec!(99.99));
        assert_eq!(result.slippage, dec!(0));
        assert_eq!(result.cost_long, dec!(99));
        assert_eq!(result.cost_short, dec!(0));
    }

    #[test]
    fn test_limit_order_buy() {
        let order_calculation = setup_futures_order_calculation();
        let order_book = setup_order_book();

        let account_balance = setup_account_balance();
        let result = order_calculation.compute_open_order(
            OrderType::Limit,
            &order_book,
            account_balance.get("USDT").unwrap().clone(),
            dec!(0),
            dec!(0.1),
            Some(dec!(9500)),
            false,
            true,
            false,
        );

        assert_eq!(result.entry_price, dec!(9500));
        assert_eq!(result.liquidation_price, dec!(9000));
        assert_eq!(result.max_quantity_base, dec!(1.0526315789473684));
        assert_eq!(result.min_quantity_base, dec!(0.001));
        assert_eq!(result.max_quantity_quote, dec!(10000));
        assert_eq!(result.min_quantity_quote, dec!(9.5));
        assert_eq!(result.fees, dec!(0.95));
        assert_eq!(result.slippage, dec!(0));
        assert_eq!(result.cost_long, dec!(9.5));
        assert_eq!(result.cost_short, dec!(9.5));
    }
    #[test]
fn test_limit_order_sell() {
    let order_calculation = setup_futures_order_calculation();
    let order_book = setup_order_book();

        let account_balance = setup_account_balance();
    let result = order_calculation.compute_open_order(
        OrderType::Limit,
        &order_book,
        account_balance.get("USDT").unwrap().clone(),
        dec!(0),
        dec!(0.1),
        Some(dec!(10500)),
        false,
        false,
        false,
    );

    assert_eq!(result.entry_price, dec!(10500));
    assert_eq!(result.liquidation_price, dec!(12600));
    assert_eq!(result.max_quantity_base, dec!(0.9523809523809523));
    assert_eq!(result.min_quantity_base, dec!(0.001));
    assert_eq!(result.max_quantity_quote, dec!(10000));
    assert_eq!(result.min_quantity_quote, dec!(10.5));
    assert_eq!(result.fees, dec!(1.05));
    assert_eq!(result.slippage, dec!(0));
    assert_eq!(result.cost_long, dec!(10.5));
    assert_eq!(result.cost_short, dec!(10.5));
}

#[test]
fn test_market_order_buy_quote() {
    let order_calculation = setup_futures_order_calculation();
    let order_book = setup_order_book();
        let account_balance = setup_account_balance();

    let result = order_calculation.compute_open_order(
        OrderType::Market,
        &order_book,
        account_balance.get("USDT").unwrap().clone(),
        dec!(0),
        dec!(1000),
        None,
        true,
        true,
        false,
    );

    assert_eq!(result.entry_price, dec!(10000));
    assert_eq!(result.liquidation_price, dec!(9030));
    assert_eq!(result.max_quantity_base, dec!(0.999));
    assert_eq!(result.min_quantity_base, dec!(0.001));
    assert_eq!(result.max_quantity_quote, dec!(9990));
    assert_eq!(result.min_quantity_quote, dec!(10));
    assert_eq!(result.fees, dec!(1));
    assert_eq!(result.slippage, dec!(0));
    assert_eq!(result.cost_long, dec!(100));
    assert_eq!(result.cost_short, dec!(100));
}

#[test]
fn test_market_order_sell_quote() {
    let order_calculation = setup_futures_order_calculation();
    let order_book = setup_order_book();
        let account_balance = setup_account_balance();

    let result = order_calculation.compute_open_order(
        OrderType::Market,
        &order_book,
        account_balance.get("USDT").unwrap().clone(),
        dec!(0),
        dec!(0.1),
        None,
        true,
        false,
        false,
    );

    assert_eq!(result.entry_price, dec!(10000));
    assert_eq!(result.liquidation_price, dec!(12000));
    assert_eq!(result.max_quantity_base, dec!(1));
    assert_eq!(result.min_quantity_base, dec!(0.001));
    assert_eq!(result.max_quantity_quote, dec!(10000));
    assert_eq!(result.min_quantity_quote, dec!(10));
    assert_eq!(result.fees, dec!(1));
    assert_eq!(result.slippage, dec!(0));
    assert_eq!(result.cost_long, dec!(10));
    assert_eq!(result.cost_short, dec!(10));
    }

    #[test]
fn test_limit_order_buy_quote() {
    let order_calculation = setup_futures_order_calculation();
    let order_book = setup_order_book();

        let account_balance = setup_account_balance();
    let result = order_calculation.compute_open_order(
        OrderType::Limit,
        &order_book,
        account_balance.get("USDT").unwrap().clone(),
        dec!(0),
        dec!(0.5),
        Some(dec!(10000)),
        true,
        true,
        false,
    );

    assert_eq!(result.entry_price, dec!(10000));
    assert_eq!(result.liquidation_price, dec!(8333.333333333333333333333333));
    assert_eq!(result.max_quantity_base, dec!(1));
    assert_eq!(result.min_quantity_base, dec!(0.001));
    assert_eq!(result.max_quantity_quote, dec!(10000));
    assert_eq!(result.min_quantity_quote, dec!(10));
    assert_eq!(result.fees, dec!(0.1));
    assert_eq!(result.slippage, dec!(0));
    assert_eq!(result.cost_long, dec!(10));
    assert_eq!(result.cost_short, dec!(10));
}

#[test]
fn test_limit_order_sell_quote() {
    let order_calculation = setup_futures_order_calculation();
    let order_book = setup_order_book();
        let account_balance = setup_account_balance();
    let result = order_calculation.compute_open_order(
        OrderType::Limit,
        &order_book,
        account_balance.get("USDT").unwrap().clone(),
        dec!(0),
        dec!(0.5),
        Some(dec!(10000)),
        true,
        false,
        false,
    );

    assert_eq!(result.entry_price, dec!(10000));
    assert_eq!(result.liquidation_price, dec!(12500));
    assert_eq!(result.max_quantity_base, dec!(1));
    assert_eq!(result.min_quantity_base, dec!(0.001));
    assert_eq!(result.max_quantity_quote, dec!(10000));
    assert_eq!(result.min_quantity_quote, dec!(10));
    assert_eq!(result.fees, dec!(0.1));
    assert_eq!(result.slippage, dec!(0));
    assert_eq!(result.cost_long, dec!(10));
    assert_eq!(result.cost_short, dec!(10));
}


}

