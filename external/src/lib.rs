mod types;
mod entities;
mod http_api;
mod ws_api;

use std::sync::Arc;

use tokio::sync::{ Mutex};
use types::LockedOrderBook;
use crate::{http_api::*, ws_api::*};
use core_pkg::orderbook::OrderBook;

use crate::{entities::OrderbookEntity};

pub struct OrderBookManager {
    http_api: HttpApi,
    ws_api: Option<OrderBookWebSocket>,
    orderbook: LockedOrderBook,
}

impl OrderBookManager {
    // Pass empty ws_url to disable use Rust socket, which is not yet ready to use
    pub async fn new(api_url: &str, ws_url: &str, symbol: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let http_api = HttpApi::new(api_url, symbol);
        let orderbook = Arc::new(Mutex::new(OrderBook::new()));
        let mut ws_api = None;

        if ws_url.to_string().len() > 0 {
            ws_api = Some(OrderBookWebSocket::new(ws_url.to_string(), symbol.to_string(), Arc::clone(&orderbook)));
        }
        let mut manager = OrderBookManager {
            http_api,
            ws_api,
            orderbook,
        };
        manager.fetch_and_fill_orderbook().await?;
        if let Some(ws_api) = manager.ws_api.as_mut() {
            ws_api.connect_and_subscribe().await?;
        }
        Ok(manager)
    }

    async fn fetch_and_fill_orderbook(&self) -> Result<(), Box<dyn std::error::Error>> {
        let orderbook_data = self.http_api.fetch_order_book().await.expect("Fetch REST orderbook failed");
        let mut orderbook = self.orderbook.lock().await;
        orderbook.initialize(orderbook_data.asks, orderbook_data.bids);
        Ok(())
    }

    pub async fn switch_pair(&mut self, new_symbol: &str) -> Result<(), Box<dyn std::error::Error>> {
        self.http_api.switch_symbol(new_symbol);
        self.fetch_and_fill_orderbook().await?;
        if let Some(ws_api) = &mut self.ws_api {
            ws_api.switch_pair(new_symbol.to_string()).await?;
        }
        Ok(())
    }

    // If you disable socket connection, you should connect to the enpoint outside
    // then use this method to update the orderbook data
    pub async fn update_orderbook(&mut self, updated_data: OrderbookEntity) {
        let mut orderbook = self.orderbook.lock().await;
        orderbook.update_order(true, updated_data.asks);
        orderbook.update_order(false, updated_data.bids);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;
    #[tokio::test]
    async fn test_initialization_with_out_websocket() {
        let mut order_book_manager = OrderBookManager::new("https://apex.position.exchange", "", "BTCBUSD").await.unwrap();
        let depth = order_book_manager.orderbook.lock().await.get_depth();
        println!("depth {:?}", depth);
        assert!(!depth.0.is_empty());
        assert!(!depth.1.is_empty());
        order_book_manager.update_orderbook(OrderbookEntity::new(vec![
            (dec!(10), dec!(0.1)),
            (dec!(11), dec!(0.1)),
        ], vec![
            (dec!(9), dec!(0.1)),
            (dec!(8), dec!(0.1)),
        ])).await;
        let depth = order_book_manager.orderbook.lock().await.get_depth();
        println!("depth {:?}", depth);
    }
    

    #[tokio::test]
    async fn test_switch_pair() {
        // let order_book_manager = OrderBookManager::new("http://api.example.com", "wss://ws.example.com", "BTCUSD").await.unwrap();
        // order_book_manager.switch_pair("ETHUSD").await.unwrap();
        //
        // assert_eq!(order_book_manager.ws.symbol, "ETHUSD");
        // assert!(!order_book_manager.order_book.lock().await.bids.is_empty());
        // assert!(!order_book_manager.order_book.lock().await.asks.is_empty());
    }

    #[tokio::test]
    async fn test_error_scenarios() {
        // Test invalid API URL
        let result = OrderBookManager::new("http://invalid-api.example.com", "wss://ws.example.com", "BTCUSD").await;
        assert!(result.is_err());

        // Test invalid WebSocket URL
        let result = OrderBookManager::new("http://api.example.com", "wss://invalid-ws.example.com", "BTCUSD").await;
        assert!(result.is_err());

        // Test invalid symbol
        let result = OrderBookManager::new("http://api.example.com", "wss://ws.example.com", "INVALID").await;
        assert!(result.is_err());
    }

}
