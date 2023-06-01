use core::{
    compute::order::{self, FuturesOrder, FuturesOrderCalculation},
    orderbook::OrderBook,
};
use serde_derive::Serialize;
use rust_decimal::Decimal;
use std::borrow::BorrowMut;
use std::collections::HashMap;

type OrderCalculatationLockable = FuturesOrderCalculation; //Arc<Mutex<order::FuturesOrderCalculation>>;

#[derive(Debug, Clone, Serialize)]
pub struct OrderManager {
    user_balance: HashMap<String, Decimal>,
    pair_order_compute: HashMap<String, OrderCalculatationLockable>,
    pub active_pair_symbol: String,
}

impl OrderManager {
    pub fn new() -> Self {
        Self {
            user_balance: HashMap::new(),
            pair_order_compute: HashMap::new(),
            active_pair_symbol: "".to_string(),
        }
    }

    pub fn new_pair_order_compute(
        &mut self,
        pair_symbol: String,
        collateral_long_token: String,
        collateral_short_token: String,
        leverage: String,
        max_notional: String,
        min_quantity_base: String,
        margin_ratio: String,
        taker_fee: String,
        maker_fee: String,
        base_token_precision: u32,
    ) {
        // log(format!("RUST:: new pair {}", pair_symbol.clone()).as_str());
        self.pair_order_compute.insert(
            pair_symbol.clone(),
            FuturesOrderCalculation::new(
                collateral_long_token,
                collateral_short_token,
                leverage,
                max_notional,
                min_quantity_base,
                margin_ratio,
                taker_fee,
                maker_fee,
                base_token_precision,
            ),
        );
        // log(format!("RUST:: new pair {} DONE 1", pair_symbol.clone()).as_str());
        self.active_pair_symbol = pair_symbol.clone();
        // log(format!("RUST:: new pair {} DONE 2", pair_symbol.clone()).as_str());
    }

    pub fn update_balance(&mut self, token: String, balance: String) {
        println!("Balance before {:?}", self.user_balance.get("BTC"));
        self.user_balance
            .insert(token, Decimal::from_str_exact(&balance).unwrap());
        println!("Balance after {:?}", self.user_balance.get("BTC"));
    }

    pub fn compute_open_order(
        &self,
        orderbook: &OrderBook,
        pay_token: String,
        pay_amount: String,
        limit_price: Option<String>,
        quantity: String,
        is_quote: bool,
        is_buy: bool,
        use_percentage: bool,
    ) -> FuturesOrder {
        // log(format!(
        //     "RUST:: Compute open order: {:?}, is_quote {}, is_buy {}",
        //     quantity, is_quote, is_buy
        // )
        // .as_str());
        let order_type: order::OrderType;
        let mut price: Option<Decimal> = None;
        match limit_price {
            Some(expr) => {
                price = Some(Decimal::from_str_exact(&expr).unwrap());
                order_type = order::OrderType::Limit;
                // log("RUST:: Limit price");
            }
            None => {
                order_type = order::OrderType::Market;
                // log("RUST:: Market price");
            }
        }
        // log(format!(
        //     "RUST:: order type: {}",
        //     price.unwrap_or_else(|| Decimal::ZERO)
        // )
        // .as_str());
        let result = self
            .get_active_order_compute()
            .borrow_mut()
            .compute_open_order(
                order_type,
                orderbook,
                *self
                    .user_balance
                    .get(&pay_token)
                    .clone()
                    .unwrap_or_else(|| &Decimal::ZERO),
                order::string_to_decimal(&pay_amount, "Invlaid pay amount"),
                order::string_to_decimal(&quantity, "Invalid quantity"),
                price,
                is_quote,
                is_buy,
                use_percentage,
            );
        let final_result = match result {
            Ok(result) => result,
            Err(..) => {
                // log("RUST:: compute error");
                FuturesOrder::empty()
            }
        };
        // log(format!("RUST:: raw result {:?}", &final_result).as_str());

        final_result
    }

    pub fn check_pair_exists(&mut self, new_active_pair: String) -> bool {
        match self.pair_order_compute.get(&new_active_pair) {
            None => false,
            Some(_) => true
        }
    }

    pub fn change_active_pair(&mut self, new_active_pair: String) {
        if self.check_pair_exists(new_active_pair.clone()) {
            self.active_pair_symbol = new_active_pair;
        } else {
            println!("{} not initialized", new_active_pair);
        }
    }

    pub fn change_leverage(&mut self, new_leverage: String, max_notional: String) {
        println!("change leverage to {}, max notional {} ", new_leverage, max_notional);
        let active_pair = self.get_active_order_compute();
        self.new_pair_order_compute(
            self.active_pair_symbol.clone(),
            active_pair.collateral_long_token,
            active_pair.collateral_short_token,
            new_leverage,
            max_notional,
            active_pair.min_quantity_base.to_string(),
            active_pair.margin_ratio.to_string(),
            active_pair.taker_fee.to_string(),
            active_pair.maker_fee.to_string(),
            active_pair.base_token_precision,
        );
        // self.get_active_order_compute()
        //     .borrow_mut()
        //     .change_leverage(
        //         Decimal::from_str_exact(&new_leverage).unwrap(),
        //         max_notional,
        //     );
        let leverage_after = self.get_active_order_compute().leverage;
        println!("change leverage to {} after", leverage_after);
    }

    pub fn get_active_order_compute(&self) -> OrderCalculatationLockable {
        println!("RUST:: active pair {:?}", self.pair_order_compute);
        return self
            .pair_order_compute
            .get(&self.active_pair_symbol.clone())
            .expect("Not initialized. Make sure you have set active pair, and it's configuration.")
            .clone();
    }
}