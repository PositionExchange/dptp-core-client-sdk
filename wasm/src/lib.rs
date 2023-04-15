use serde::{ser::SerializeTuple, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use core::orderbook::OrderBook;
use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use console_error_panic_hook::set_once;
use js_sys::Array;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[wasm_bindgen(start)]
pub fn initialize() {
    // set up the console error hook
    set_once();
}

type PriceLevel = (Decimal, Decimal);

#[wasm_bindgen]
pub struct OrderBookManager {
    orderbook: OrderBook,
}

#[wasm_bindgen]
impl OrderBookManager {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            orderbook: OrderBook::new(),
        }
    }

    #[wasm_bindgen]
    pub fn initialize_orders(&mut self, asks: Array, bids: Array) {
        let asks: Vec<PriceLevel> = to_price_level_vec(&asks)
            .into_iter()
            .map(|(price, quantity)| (Decimal::from_str_exact(&price).unwrap(), Decimal::from_str_exact(&quantity).unwrap()))
            .collect();

        let bids: Vec<PriceLevel> = to_price_level_vec(&bids)
            .into_iter()
            .map(|(price, quantity)| (Decimal::from_str_exact(&price).unwrap(), Decimal::from_str_exact(&quantity).unwrap()))
            .collect();

        self.orderbook.initialize(asks, bids);
    }

    #[wasm_bindgen]
    pub fn update_orders(&mut self, is_ask: bool, updates: Array) {
        let updates: Vec<PriceLevel> = to_price_level_vec(&updates)
            .into_iter()
            .map(|(price, quantity)| (Decimal::from_str_exact(&price).unwrap(), Decimal::from_str_exact(&quantity).unwrap()))
            .collect();

        self.orderbook.update_order(is_ask, updates);
    }

    
    #[wasm_bindgen]
    pub fn compute_dry(
        &self,
        fill_amount: String,
        fill_by_quote: bool,
        is_buy: bool,
    ) -> Result<JsValue, JsValue> {
        let fill_amount_decimal =
            Decimal::from_str_exact(&fill_amount).map_err(|e| JsValue::from_str(&e.to_string())).unwrap();

        let (avg_price, total_base, slippage) =
            self.orderbook.compute_dry(fill_amount_decimal, fill_by_quote, is_buy);

        // let result = ComputeDryResult {
        //     avg_price: avg_price.to_string(),
        //     total_base: total_base.to_string(),
        //     slippage: slippage.to_string(),
        // };
        let result = (avg_price.to_string(), total_base.to_string(), slippage.to_string());

        to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    #[wasm_bindgen]
    pub fn get_depth(&self) -> Result<JsValue, JsValue> {
        let (asks, bids) = self.orderbook.get_depth();
        let asks_js: Vec<(String, String)> = asks
            .into_iter()
            .map(|(price, quantity)| (price.to_string(), quantity.to_string()))
            .collect();
        let bids_js: Vec<(String, String)> = bids
            .into_iter()
            .map(|(price, quantity)| (price.to_string(), quantity.to_string()))
            .collect();

        // (JsValue::from_serde(&asks_js).unwrap(), JsValue::from_serde(&bids_js).unwrap())
        let result = (asks_js, bids_js);
        to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))
    }

    // #[wasm_bindgen]
    // pub fn get_best_ask_bid(&self) -> (Option<String>, Option<String>) {
    //     let (best_ask, best_bid) = self.orderbook.get_best_ask_bid();
    //     (best_ask.map(|x| x.to_string()), best_bid.map(|x| x.to_string()))
    // }

    #[wasm_bindgen]
    pub fn group_prices(&self, grouping_size: String) -> Result<JsValue, JsValue> {
        let grouping_size_decimal = Decimal::from_str_exact(&grouping_size).map_err(|e| JsValue::from_str(&e.to_string()))?;

        let (grouped_asks, grouped_bids) = self.orderbook.group_prices(grouping_size_decimal);
        
        let asks_js: Vec<_> = grouped_asks
            .into_iter()
            .map(|(price, quantity)| {
                 (price.to_string(), quantity.to_string());
                // JsValue::from_serde(&tuple).unwrap()
            })
            .collect();
        let bids_js: Vec<_> = grouped_bids
            .into_iter()
            .map(|(price, quantity)| {
                (price.to_string(), quantity.to_string());
                // JsValue::from_serde(&tuple).unwrap()
            })
            .collect();

        // ((JsValue::from_serde(&asks_js).unwrap(), JsValue::from_serde(&bids_js).unwrap()))
        let result = (asks_js, bids_js);
        to_value(&result).map_err(|e| JsValue::from_str(&e.to_string()))
    }

}

// #[wasm_bindgen]
// #[derive(serde::Serialize)]
// pub struct ComputeDryResult {
//     pub avg_price: String,
//     pub total_base: String,
//     pub slippage: String,
// }


fn to_price_level_vec(arr: &Array) -> Vec<(String, String)> {
    let mut result = Vec::new();
    for i in 0..arr.length() {
        let tuple = arr.get(i);
        if let Some(tuple) = tuple.dyn_into::<Array>().ok() {
            let price = tuple.get(0).as_string().unwrap();
            let quantity = tuple.get(1).as_string().unwrap();
            result.push((price, quantity));
        }
    }
    result
}


fn to_vec_tuple(arr: &Array) -> Vec<(String, String)> {
    let mut result = Vec::new();
    for i in 0..arr.length() {
        let tuple = arr.get(i);
        if let Some(tuple) = tuple.dyn_into::<Array>().ok() {
            let price = tuple.get(0).as_string().unwrap();
            let quantity = tuple.get(1).as_string().unwrap();
            result.push((price, quantity));
        }
    }
    result
}
    

