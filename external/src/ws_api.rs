use rust_decimal::Decimal;
use reqwest::Url;
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message, WebSocketStream, MaybeTlsStream};
use futures_util::{StreamExt, SinkExt};
use serde_json::Value;
use std::sync::Arc;
use tokio::sync::Mutex;
use core_pkg::orderbook::OrderBook;
use warp::http::Request;

use crate::{entities::OrderbookEntity, types::LockedOrderBook};

type PriceLevel = (Decimal, Decimal);
// pub type LockedOrderBook = Arc<Mutex<OrderBook>>;

type LockWsStream = Arc<Mutex<WebSocketStream<MaybeTlsStream<tokio::net::TcpStream>>>>;

pub struct OrderBookWebSocket {
    pub url: String,
    pub symbol: String,
    pub orderbook: LockedOrderBook,
    ws_stream: Option<LockWsStream>,
}

/*
* NOTE: This websocket is_terminated is not currently working yet.
* Please wait for the support of socket.io client for wasm. 
*/

impl OrderBookWebSocket {
    pub fn new(url: String, symbol: String, orderbook: LockedOrderBook) -> Self {
        let mut url = url;
        if !url.contains("socket.io") {
            url = format!("{}/socket.io/?EIO=4&transport=websocket", url);
        }
        OrderBookWebSocket {
            url,
            symbol,
            orderbook,
            ws_stream: None,
        }
    }

    pub async fn connect_and_subscribe(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.connect().await.expect("Connect Socket Error");
        // let (mut write, mut read) = self.ws_stream.split();
        // let join_msg = format!(r#"["join", "orderBook@{}"]"#, self.symbol);
        // write.send(Message::Text(join_msg)).await?;
        self.subscribe(self.symbol.clone()).await?;

        self.receive_update_orderbook_messages().await?;

        Ok(())
    }

    pub async fn switch_pair(&mut self, symbol: String) -> Result<(), Box<dyn std::error::Error>> {
        self.unsubscribe().await?;
        self.subscribe(symbol.clone()).await?;
        self.symbol = symbol;
        Ok(())
    }

    pub async fn connect(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("connecting to {}", &self.url);
        let ws_url = "wss://ws.position.exchange/socket.io/?EIO=4&transport=websocket";
        // let ws_url = connect_to_socket_io_and_get_url(raw_ws_url, "/socket.io/").await?;
        // println!("connecting to ws url {}", &ws_url);
        // let request = Request::builder()
        //     .uri(&ws_url)
        //     .header("Origin", "https://position.exchange")
        //     .header("Sec-WebSocket-Key", "C5QvKmsapnViUj/9B0KyWQ==")
        //     .header("Sec-WebSocket-Version", "13")
        //     .header("Upgrade", "websocket")
        //     .header("User-Agent", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/111.0.0.0 Safari/537.36")
        //     .body(())?;
        //
       let (ws_stream, _) = tokio_tungstenite::connect_async(ws_url).await.expect("Failed to connect socket");
        self.ws_stream = Some(Arc::new(Mutex::new(ws_stream)));
        Ok(())
    }

    pub async fn subscribe(&self, symbol: String) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ws_stream) = &self.ws_stream {
            let event = format!(r#"["join", "orderBook@{}"]"#, symbol);
            let mut ws_stream = ws_stream.lock().await;
            ws_stream.send(Message::text(event)).await?;
            Ok(())
        } else {
            Err("WebSocket stream is not connected".into())
        }
    }

   pub async fn unsubscribe(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ws_stream) = &self.ws_stream {
            let event = format!(r#"["leave", "orderBook@{}"]"#, self.symbol);
            let mut ws_stream = ws_stream.lock().await;
            ws_stream.send(Message::text(event)).await?;
            Ok(())
        } else {
            Err("WebSocket stream is not connected".into())
        }
    }


    pub async fn handle_reconnect(&mut self) {
        loop {
            // Check if the WebSocket is still connected
            if let Some(ws_stream) = &self.ws_stream {
                // if ws_stream.is_terminated() {
                //     // If disconnected, reconnect and subscribe again
                //     if let Err(e) = self.connect().await {
                //         println!("Error reconnecting: {}", e);
                //     } else if let Err(e) = self.subscribe(self.symbol).await {
                //         println!("Error resubscribing: {}", e);
                //     }
                // }
            } else {
                // If the WebSocket is not connected, try to connect and subscribe
                if let Err(e) = self.connect().await {
                    println!("Error connecting: {}", e);
                } else if let Err(e) = self.subscribe(self.symbol.clone()).await {
                    println!("Error subscribing: {}", e);
                }
            }

            // Wait before checking the connection again
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
    }

    pub async fn receive_update_orderbook_messages(&self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(ws_stream) = &self.ws_stream {
            // let (_, mut read) = ws_stream.clone().lock().await.split();
            let mut read = ws_stream.lock().await;
            
            while let Some(msg) = read.next().await {
                let msg = msg?;
                if let Message::Text(text) = msg {
                    if let Some(updated_data) = self.parse_orderbook_update(&text) {
                        self.update_orderbook(updated_data).await;
                    }
                }
            }
        } else {
            return Err("WebSocket stream is not connected".into());
        }
        Ok(())
    }

    fn parse_orderbook_update(&self, text: &str) -> Option<OrderbookEntity> {
        // check message is an orderbook update
        if !text.contains("orderBookUpdated") {
            return None;
        }

        let start_index = text.find('[')?;
        let end_index = text.rfind(']')?;
        let payload = &text[start_index..=end_index];
        let json: Value = serde_json::from_str(payload).ok()?;
        let orderbook_data = json.get(1)?.get("data")?;

        let asks = orderbook_data["asks"]
            .as_array()?
            .iter()
            .map(|arr| {
                let price = arr[0].as_str().expect("ask price").parse::<Decimal>().ok().unwrap();
                let quantity = arr[1].as_str().expect("asks must have a quantity").parse::<Decimal>().ok().unwrap();
                Ok::<(rust_decimal::Decimal, rust_decimal::Decimal), String>((price, quantity))
            })
            .collect::<Result<Vec<PriceLevel>, _>>()
            .ok()?;

        let bids = orderbook_data["bids"]
            .as_array()?
            .iter()
            .map(|arr| {
                let price = arr[0].as_str().expect("bid price").parse::<Decimal>().ok().unwrap();
                let quantity = arr[1].as_str().expect("bid price").parse::<Decimal>().ok().unwrap();
                Ok::<(rust_decimal::Decimal, rust_decimal::Decimal), String>((price, quantity))
            })
            .collect::<Result<Vec<PriceLevel>, _>>()
            .ok()?;

        Some(OrderbookEntity { asks, bids })
    }


    async fn update_orderbook(&self, updated_data: OrderbookEntity) {
        let mut orderbook = self.orderbook.lock().await;
        orderbook.update_order(true, updated_data.asks);
        orderbook.update_order(false, updated_data.bids);
    }
}


async fn connect_to_socket_io_and_get_url(url: &str, path: &str) -> Result<String, Box<dyn std::error::Error>> {
    let url = Url::parse(url)?;
    let query = [("EIO", "4"), ("transport", "polling")];
    let mut url = url.join(path)?.into_string();
    url.push_str("?");

    for (i, (key, value)) in query.iter().enumerate() {
        if i > 0 {
            url.push('&');
        }
        url.push_str(&format!("{}={}", key, value));
    }
    println!("url: {}", url);

    let client = reqwest::Client::new();
    let res = client.get(&url).send().await.expect("Failed to send request");
    let text = res.text().await.expect("Failed to get text");
    println!("text: {}", text);
    let text = text.strip_prefix("0").unwrap_or(&text);
    let json: Value = serde_json::from_str(text)?;

    let sid = json["sid"].as_str().ok_or("No sid found in response")?;

    let ws_url = format!(
        "wss://ws.position.exchange/socket.io/?EIO=4&transport=websocket&sid={}",
        // if url.starts_with("https") { "wss" } else { "ws" },
        // url.strip_prefix("http://").unwrap_or(&url).strip_prefix("https://").unwrap_or(&url),
        sid
    );
    // let (socket, response) = connect_async(&ws_url).await?;

    Ok(ws_url)
}


#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal::Decimal;
    use rust_decimal_macros::dec;
    use tokio_tungstenite::tungstenite::protocol::Message;
    use warp::{Filter, ws::WebSocket};
    async fn order_book_update_ws(ws: WebSocket) {
        let (mut ws_tx, _) = ws.split();
        let update_data = r#"42["orderBookUpdated",{"room":"orderBook@fBTCBUSD","event":"orderBookUpdated","data":{"symbol":"fBTCBUSD","asks":[["100.1","0.5"]],"bids":[["99.9","1.2"]],"price":"","liquidity":[]}}]"#;
        ws_tx.send(warp::ws::Message::text(update_data)).await.unwrap();
    }

    #[tokio::test]
    async fn test_update_orderbook() {
        // Prepare the initial order book
        // let initial_orderbook = OrderbookEntity {
        //     asks: vec![(Decimal::new(100, 1), Decimal::new(1, 0))],
        //     bids: vec![(Decimal::new(99, 1), Decimal::new(1, 0))],
        // };
        let mut orderbook = OrderBook::new();
        orderbook.initialize(vec![(dec!(100.0), dec!(1))], vec![(dec!(99.0), dec!(1.0))]);
        let orderbook_websocket = OrderBookWebSocket::new(String::from(""), "Hello".to_string(), Arc::new(Mutex::new(orderbook)));

        // Update the order book
        let updated_data = OrderbookEntity {
            asks: vec![((dec!(101), dec!(1)))],
            bids: vec![((dec!(99), dec!(0)))],
        };
        orderbook_websocket.update_orderbook(updated_data.clone()).await;

        // Check if the order book is updated
        let updated_orderbook = orderbook_websocket.orderbook.lock().await;
        println!("updated ob:::: {:?}", updated_orderbook);
        let (depth_asks, depth_bids): (Vec<PriceLevel>, Vec<PriceLevel>) = updated_orderbook.get_depth();
        println!("updated depth:::: {:?}", depth_asks);
        assert_eq!(depth_asks, vec![
            (dec!(100), dec!(1)),
            (dec!(101), dec!(1)),
        ]);
        assert_eq!(depth_bids, vec![
        ]);
        // assert_eq!(updated_orderbook.bids, updated_data.bids);
    }

    #[tokio::test]
    async fn test_websocket() {
        // Prepare the mock WebSocket server
        let ws_route = warp::path("socket.io")
            .and(warp::ws())
            .map(|ws: warp::ws::Ws| ws.on_upgrade(order_book_update_ws));
        let server = warp::serve(ws_route).bind_with_graceful_shutdown(([127, 0, 0, 1], 3030), async {
            tokio::time::sleep(tokio::time::Duration::from_secs(1)).await;
        });
        tokio::spawn(server.1);

        // Prepare the initial order book
        let initial_orderbook = OrderbookEntity {
            asks: vec![(Decimal::new(100, 1), Decimal::new(1, 0))],
            bids: vec![(Decimal::new(99, 1), Decimal::new(1, 0))],
        };
        let mut orderbook = OrderBook::new();
        orderbook.initialize(initial_orderbook.asks, initial_orderbook.bids);

        // Initialize the OrderBookWebSocket with the mock server URL
        let url = "ws://127.0.0.1:3030/socket.io/?EIO=4&transport=websocket";
        let mut orderbook_websocket = OrderBookWebSocket::new(String::from(url), "Hello".to_string(), Arc::new(Mutex::new(orderbook)));

        // Subscribe to the BTCBUSD pair
        orderbook_websocket.connect_and_subscribe().await;        // Wait for the order book update
        tokio::time::sleep(tokio::time::Duration::from_secs(2)).await;

        // Check if the order book is updated
        let updated_orderbook = orderbook_websocket.orderbook.lock().await;
        let depth = updated_orderbook.get_depth();
        println!("updated depth:::: {:?}", depth);
        assert_eq!(
            depth.0,
            vec![(Decimal::new(100, 1), Decimal::new(1, 0)), (dec!(100.1), dec!(0.5))]
        );
        assert_eq!(
            depth.1,
            vec![
                (dec!(99.9), dec!(1.2)),
                (Decimal::new(99, 1), Decimal::new(1, 0)),
            ]
        );
    }


}
