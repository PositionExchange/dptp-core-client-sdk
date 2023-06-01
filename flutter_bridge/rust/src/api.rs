use core::{compute::order::FuturesOrder, orderbook::OrderBook};
use futures::executor::block_on;
use rust_decimal::prelude::FromPrimitive;
use rust_decimal::Decimal;
use serde_derive::Serialize;
use serde_json::*;
use static_init::dynamic;
use std::borrow::BorrowMut;
use std::collections::HashMap;
use tokio::io::AsyncWriteExt;

use crate::orders::*;

type PriceLevel = (Decimal, Decimal);

#[derive(Debug, Clone, Serialize)]
pub struct OrderBookManager {
    orderbook: OrderBook,
    order_manager: OrderManager,
}

impl OrderBookManager {
    pub fn new() -> Self {
        let orderbook = OrderBook::new();
        let order_manager = OrderManager::new();
        OrderBookManager {
            orderbook,
            order_manager,
        }
    }

    pub fn initialize_orders(&mut self, asks: Vec<Vec<f32>>, bids: Vec<Vec<f32>>) {
        let asks: Vec<PriceLevel> = asks
            .iter()
            .map(|item| {
                (
                    Decimal::from_f32(item[0]).unwrap(),
                    Decimal::from_f32(item[1]).unwrap(),
                )
            })
            .collect();

        let bids: Vec<PriceLevel> = bids
            .iter()
            .map(|item| {
                (
                    Decimal::from_f32(item[0]).unwrap(),
                    Decimal::from_f32(item[1]).unwrap(),
                )
            })
            .collect();

        self.orderbook.borrow_mut().initialize(asks, bids);
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
        self.order_manager.new_pair_order_compute(
            pair_symbol,
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
    }

    pub fn update_balance(&mut self, token: String, balance: String) {
        self.order_manager.update_balance(token, balance);
    }

    pub fn change_leverage(&mut self, new_leverage: String, max_notional: String) {
        self.order_manager.change_leverage(new_leverage, max_notional);
    }

    pub fn check_pair_exists(&mut self, new_active_pair: String) -> bool {
        self.order_manager.check_pair_exists(new_active_pair)
    }

    pub fn get_active_pair(&self) -> String {
        self.order_manager.active_pair_symbol.clone()
    }

    pub fn change_active_pair(&mut self, new_active_pair: String) {
        self.order_manager.change_active_pair(new_active_pair);
    }

    pub fn compute_open_order(
        &self,
        pay_token: String,
        pay_amount: String,
        limit_price: Option<String>,
        quantity: String,
        is_quote: bool,
        is_buy: bool,
        use_percentage: bool,
    ) -> FuturesOrder {
        self.order_manager.compute_open_order(
            &self.orderbook,
            pay_token,
            pay_amount,
            limit_price,
            quantity,
            is_quote,
            is_buy,
            use_percentage,
        )
    }

    pub fn get_order_book_manager(&self) -> OrderBookManager {
        self.clone()
    }
}

fn to_price_level_vec(arr: &Vec<Vec<String>>) -> Vec<(String, String)> {
    let mut result = Vec::new();
    for i in 0..arr.len() {
        let tuple = arr.get(i);

        if let Some(tuple) = tuple {
            if tuple.len() == 0 {
                continue;
            }
            let price = tuple.get(0).unwrap().to_owned();
            let quantity = tuple.get(1).unwrap().to_owned();
            result.push((price, quantity));
        }
    }
    result
}

#[dynamic]
static mut ORDER_BOOK_MANAGER: OrderBookManager = OrderBookManager::new();

pub fn initialize_orders(asks: String, bids: String) {
    let asks: Vec<Vec<f32>> = serde_json::from_str(&format!(r#"{}"#, asks)).unwrap();
    let bids: Vec<Vec<f32>> = serde_json::from_str(&format!(r#"{}"#, bids)).unwrap();
    // println!("asks: {:?}", asks);
    // println!("bids: {:?}", bids);
    ORDER_BOOK_MANAGER.write().initialize_orders(asks, bids);
}

pub fn new_pair_order_compute(
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
    ORDER_BOOK_MANAGER.write().new_pair_order_compute(
        pair_symbol,
        collateral_long_token,
        collateral_short_token,
        leverage,
        max_notional,
        min_quantity_base,
        margin_ratio,
        taker_fee,
        maker_fee,
        base_token_precision,
    );
}

pub fn change_leverage(new_leverage: String, max_notional: String) {
    ORDER_BOOK_MANAGER.write().change_leverage(
        new_leverage,
        max_notional,
    );
}

pub fn get_active_pair() -> String {
    ORDER_BOOK_MANAGER.write().get_active_pair()
}

pub fn change_active_pair(new_active_pair: String) {
    ORDER_BOOK_MANAGER.write().change_active_pair(new_active_pair);
}

pub fn check_pair_exists(new_active_pair: String) {
    ORDER_BOOK_MANAGER.write().check_pair_exists(new_active_pair);
}

pub fn update_balance(token: String, balance: String) {
    ORDER_BOOK_MANAGER.write().update_balance(
        token,
        balance,
    );
}

pub fn compute_open_order(
    pay_token: String,
    pay_amount: String,
    limit_price: Option<String>,
    quantity: String,
    is_quote: bool,
    is_buy: bool,
    use_percentage: bool,
) -> String {
    let res = ORDER_BOOK_MANAGER.write().compute_open_order(
        pay_token,
        pay_amount,
        limit_price,
        quantity,
        is_quote,
        is_buy,
        use_percentage,
    );
    let serialized = to_string(&res).unwrap();
    serialized
}

pub fn get_order_book_manager() -> String {
    let res = ORDER_BOOK_MANAGER.write().get_order_book_manager();
    let serialized = to_string(&res).unwrap();
    serialized
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        initialize_orders("[[28241.6, 4.879], [28241.5, 4.718], [28241.4, 3.31], [28241.3, 5.082], [28241.2, 3.61], [28241.1, 1.974], [28241.0, 2.66], [28240.9, 2.221], [28240.8, 2.573], [28240.7, 4.779], [28240.6, 4.445], [28240.5, 3.88], [28240.4, 3.737], [28227.2, 4.336], [28227.1, 3.485], [28227.0, 2.843], [28226.9, 3.611], [28226.8, 1.833], [28226.7, 3.02], [28226.6, 4.658], [28226.5, 2.58], [28226.4, 3.784], [28226.3, 5.177], [28226.2, 1.995], [28226.1, 4.351], [28226.0, 2.482], [28225.9, 5.163], [28225.8, 2.002], [28225.7, 5.174], [28225.6, 3.95], [28225.5, 3.326], [28225.4, 3.193], [28225.3, 3.359], [28216.0, 5.97], [28215.9, 3.02], [28215.8, 5.678], [28215.7, 3.759], [28215.6, 3.042], [28215.5, 3.833], [28215.4, 1.885], [28215.3, 2.875], [28215.2, 3.869], [28215.1, 5.418], [28215.0, 2.719], [28214.9, 5.798], [28214.8, 4.067], [28214.7, 4.764], [28214.6, 2.019], [28214.5, 5.79], [28214.4, 3.277], [28214.3, 3.862], [28214.2, 3.566], [28214.1, 3.385], [28193.9, 4.261], [28193.8, 3.806], [28193.7, 5.898], [28193.6, 2.836], [28193.5, 3.411], [28193.4, 5.298], [28193.3, 3.041], [28193.2, 2.308], [28193.1, 5.39], [28193.0, 4.187], [28192.9, 2.306], [28192.8, 5.806], [28192.7, 5.637], [28192.6, 4.765], [28192.5, 4.316], [28192.4, 3.708], [28192.3, 2.331], [28192.2, 2.396], [28192.1, 2.472], [28192.0, 2.763], [28190.3, 5.406], [28190.2, 3.163], [28190.1, 4.521], [28190.0, 2.939], [28189.9, 3.066], [28189.8, 4.36], [28189.7, 2.896], [28189.6, 2.154], [28189.5, 3.314], [28189.4, 5.283], [28189.3, 4.179], [28189.2, 2.398], [28189.1, 4.013], [28189.0, 4.176], [28188.9, 2.913], [28188.8, 2.63], [28188.7, 3.517], [28188.6, 4.821], [28188.5, 3.525], [28188.4, 4.344], [28181.3, 3.35], [28181.2, 3.405], [28181.1, 5.423], [28181.0, 4.664], [28180.9, 2.015], [28180.8, 3.099], [28180.7, 1.43]]".to_string(), "[[28744.5, 5.277], [28744.4, 5.425], [28744.3, 2.081], [28744.2, 2.824], [28744.1, 3.347], [28744.0, 1.977], [28743.9, 3.18], [28743.8, 2.607], [28743.7, 5.751], [28743.6, 4.794], [28743.5, 3.44], [28743.4, 3.97], [28743.3, 5.508], [28743.2, 4.197], [28743.1, 2.875], [28743.0, 3.125], [28742.9, 5.582], [28742.8, 5.298], [28742.7, 3.003], [28742.6, 3.45], [28742.5, 2.496], [28706.0, 3.934], [28705.9, 4.49], [28705.8, 5.223], [28705.7, 2.502], [28705.6, 3.118], [28705.5, 3.862], [28705.4, 1.796], [28705.3, 4.039], [28705.2, 3.022], [28705.1, 4.753], [28705.0, 1.854], [28704.9, 4.467], [28704.8, 4.265], [28704.7, 4.756], [28704.6, 4.928], [28704.5, 4.402], [28704.4, 3.698], [28704.3, 3.895], [28704.2, 3.146], [28704.1, 3.035], [28703.1, 4.944], [28703.0, 5.657], [28702.9, 3.771], [28702.8, 2.607], [28702.7, 1.986], [28702.6, 2.12], [28702.5, 5.126], [28702.4, 4.961], [28702.3, 3.538], [28702.2, 3.838], [28702.1, 1.796], [28702.0, 2.426], [28701.9, 3.524], [28701.8, 2.87], [28701.7, 2.072], [28701.6, 3.58], [28701.5, 3.123], [28701.4, 2.222], [28701.3, 4.686], [28701.2, 2.056], [28685.3, 2.485], [28685.2, 4.761], [28685.1, 3.917], [28685.0, 5.751], [28684.9, 2.721], [28684.8, 4.799], [28684.7, 4.852], [28684.6, 5.066], [28684.5, 5.316], [28684.4, 5.671], [28684.3, 2.863], [28684.2, 2.371], [28684.1, 5.802], [28684.0, 3.469], [28683.9, 3.368], [28683.8, 5.913], [28683.7, 5.09], [28683.6, 5.575], [28683.5, 2.259], [28683.4, 3.993], [28100.0, 0.1], [28049.5, 3.628], [28049.4, 1.989], [28049.3, 4.341], [28049.2, 3.185], [28049.1, 2.609], [28049.0, 2.119], [28048.9, 4.043], [28048.8, 3.77], [28048.7, 4.889], [28048.6, 3.841], [28048.5, 4.132], [28048.4, 4.41], [28048.3, 2.183], [28048.2, 2.158], [28048.1, 4.723], [28048.0, 5.8], [28032.8, 3.967], [28032.7, 4.173]]".to_string());
        new_pair_order_compute(
            "ETHBUSD".to_owned(),
            "usd".to_owned(),
            "usd".to_owned(),
            "10".to_owned(),
            "500000000".to_owned(),
            "0.001".to_owned(),
            "0.03".to_owned(),
            "0.01".to_owned(),
            "0.01".to_owned(),
            5,
        );
        new_pair_order_compute(
            "BTCBUSD".to_owned(),
            "usd".to_owned(),
            "usd".to_owned(),
            "10".to_owned(),
            "500000000".to_owned(),
            "0.001".to_owned(),
            "0.03".to_owned(),
            "0.01".to_owned(),
            "0.01".to_owned(),
            5,
        );
        let output = compute_open_order(
            "USDT".to_owned(),
            "0".to_owned(),
            None,
            "1".to_owned(),
            true,
            true,
            false,
        );
        let get_order_book_manager1 = get_order_book_manager();
        // println!("get_order_book_manager: {:?}", get_order_book_manager1);
        println!("active pair {}", get_active_pair());
        change_active_pair("ETHBUSD".to_owned());
        println!("active pair {}", get_active_pair());
        // new_pair_order_compute(
        //     "BTCBUSD".to_owned(),
        //     "usd".to_owned(),
        //     "usd".to_owned(),
        //     "100".to_owned(),
        //     "500000000".to_owned(),
        //     "0.001".to_owned(),
        //     "0.03".to_owned(),
        //     "0.01".to_owned(),
        //     "0.01".to_owned(),
        //     5,
        // );
        // change_leverage("100".to_string(), "100000".to_string());
        // let get_order_book_manager = get_order_book_manager();
        // println!("get_order_book_manager: {:?}", get_order_book_manager);
    }
}
