use core::{compute::order, orderbook::OrderBook};
use std::{collections::HashMap};

use rust_decimal::Decimal;
use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

pub struct OrderManager {
    user_balance: HashMap<String, Decimal>,
    pair_order_compute: HashMap<String, order::FuturesOrderCalculation>,
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
            maker_fee: String
    ) {
        self.pair_order_compute.insert(pair_symbol.clone(), order::FuturesOrderCalculation::new(
            collateral_long_token,
            collateral_short_token,
            leverage,
            max_notional,
            min_quantity_base,
            margin_ratio,
            taker_fee,
            maker_fee
        ));
        self.active_pair_symbol = pair_symbol;
    }

    pub fn update_balance(&mut self, token: String, balance: String) {
        self.user_balance.insert(token, Decimal::from_str_exact(&balance).unwrap());
    }

    pub fn compute_open_order(
        &self,
        orderbook: &OrderBook,
        pay_token: String,
        limit_price: Option<String>,
        quantity: String,
        is_quote: bool,
        is_buy: bool,
        use_percentage: bool,
    ) -> JsValue {
        log(format!("Compute open order: {:?}, is_quote {}, is_buy {}", quantity, is_quote, is_buy).as_str());
        let order_type;
        let mut price: Option<Decimal> = None;
        match limit_price {
            Some(expr) => {
                price = Some(Decimal::from_str_exact(&expr).unwrap());
                order_type = order::OrderType::Limit;
            },
            None => {
                order_type = order::OrderType::Market;
            },
        }
        let result = self.get_active_order_compute().compute_open_order(
            order_type,
            orderbook,
            *self.user_balance.get(&pay_token).clone().unwrap_or_else(|| &Decimal::ZERO),
            pay_token,
            order::string_to_decimal(&quantity, "Invalid quantity"),
            price,
            is_quote,
            is_buy,
            use_percentage
        );
        log(format!("raw result {:?}", result).as_str());

         serde_wasm_bindgen::to_value(&result).unwrap()
    }

    fn get_active_order_compute(&self) -> order::FuturesOrderCalculation {
        return self.pair_order_compute.get(&self.active_pair_symbol).expect("Not initialized. Make sure you have set active pair, and it's configuration.").clone();
    }


}
