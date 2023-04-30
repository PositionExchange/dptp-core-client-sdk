use core::{compute::order::{self, FuturesOrder}, orderbook::OrderBook};
use std::{collections::HashMap, sync::{Arc, Mutex}, future::Future, cell::RefCell, rc::Rc};

use rust_decimal::Decimal;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

type OrderCalculatationLockable = Rc<RefCell<order::FuturesOrderCalculation>>;//Arc<Mutex<order::FuturesOrderCalculation>>;

#[derive(Clone, Debug)]
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

    pub fn new_pair_order_compute(&mut self,
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
        log(format!("RUST:: new pair {}", pair_symbol.clone()).as_str());
        self.pair_order_compute.insert(
            pair_symbol.clone(),
            Rc::new(
            RefCell::new(
                order::FuturesOrderCalculation::new(
                    collateral_long_token,
                    collateral_short_token,
                    leverage,
                    max_notional,
                    min_quantity_base,
                    margin_ratio,
                    taker_fee,
                    maker_fee,
                    base_token_precision,
                )
            ))
        );
        log(format!("RUST:: new pair {} DONE 1", pair_symbol.clone()).as_str());
        self.active_pair_symbol = pair_symbol.clone();
        log(format!("RUST:: new pair {} DONE 2", pair_symbol.clone()).as_str());
    }

    pub fn update_balance(&mut self, token: String, balance: String) {
        self.user_balance.insert(token, Decimal::from_str_exact(&balance).unwrap());
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
    ) -> Result<JsValue, String> {

        log(format!("RUST:: Compute open order: {:?}, is_quote {}, is_buy {}", quantity, is_quote, is_buy).as_str());
        let order_type: order::OrderType;
        let mut price: Option<Decimal> = None;
        match limit_price {
            Some(expr) => {
                price = Some(Decimal::from_str_exact(&expr).unwrap());
                order_type = order::OrderType::Limit;
                log("RUST:: Limit price");
            },
            None => {
                order_type = order::OrderType::Market;
                log("RUST:: Market price");
            },
        }
        log(format!("RUST:: order type: {}", price.unwrap_or_else(||Decimal::ZERO)).as_str());
        let result = self.get_active_order_compute().borrow_mut().compute_open_order(
            order_type,
            orderbook,
            *self.user_balance.get(&pay_token).clone().unwrap_or_else(|| &Decimal::ZERO),
            order::string_to_decimal(&pay_amount, "Invlaid pay amount"),
            order::string_to_decimal(&quantity, "Invalid quantity"),
            price,
            is_quote,
            is_buy,
            use_percentage
        );
        let final_result = match result {
            Ok(result) => result,
            Err(..) => {
                log("RUST:: compute error");
                FuturesOrder::empty()
            },
        };
        log(format!("RUST:: raw result {:?}", &final_result).as_str());

         Ok(serde_wasm_bindgen::to_value(&final_result).unwrap())
    }

    pub fn change_leverage(
        &self,
        new_leverage: String,
        max_notional: String,
    ) {
        log(format!("change leverage to {}, max notional {}", new_leverage, max_notional).as_str());
        self.get_active_order_compute().borrow_mut()
            .change_leverage(Decimal::from_str_exact(&new_leverage).unwrap(), max_notional);
        let leverage_after = self.get_active_order_compute().borrow().leverage;
        log(format!("change leverage to {} after", leverage_after).as_str());
    }

    pub fn get_active_order_compute(&self) -> OrderCalculatationLockable {
        log(format!("RUST:: active pair {}", self.active_pair_symbol.clone()).as_str());
        return self.pair_order_compute.get(&self.active_pair_symbol.clone()).expect("Not initialized. Make sure you have set active pair, and it's configuration.").clone();
    }
}

