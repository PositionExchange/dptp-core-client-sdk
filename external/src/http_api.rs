
use std::str::FromStr;
use reqwest::Client;
use rust_decimal::Decimal;
use async_trait::async_trait;


use crate::entities::OrderbookEntity;

// Define ApiResponse
#[derive(serde::Deserialize, Debug)]
struct ApiResponse {
    data: OrderbookData,
}

#[derive(serde::Deserialize, Debug)]
struct OrderbookData {
    asks: Vec<(String, String)>,
    bids: Vec<(String, String)>,
}

#[async_trait]
pub trait OrderBookApi {
    async fn fetch_order_book(&self) -> Result<OrderbookEntity, Box<dyn std::error::Error>>;
    fn switch_symbol(&mut self, symbol: &str);
}

pub struct HttpApi {
    base_url: String,
    symbol: String,
}

impl HttpApi {
    pub fn new(base_url: &str, symbol: &str) -> Self {
        HttpApi {
            base_url: base_url.to_string(),
            symbol: symbol.to_string(),
        }
    }
}

#[async_trait]
impl OrderBookApi for HttpApi {
    async fn fetch_order_book(&self) -> Result<OrderbookEntity, Box<dyn std::error::Error>> {
        let client = Client::new();
        let url = format!("{}/order-book/v1/books?depth=10&symbol={}", self.base_url, format!("f{}", self.symbol));
        let response = client.get(&url)
            .send().await?.json::<ApiResponse>().await?;

        let asks = response.data.asks.into_iter().map(|(price, quantity)| {
            (
                Decimal::from_str(&price).unwrap(),
                Decimal::from_str(&quantity).unwrap()
            )
        }).collect();

        let bids = response.data.bids.into_iter().map(|(price, quantity)| {
            (
                Decimal::from_str(&price).unwrap(),
                Decimal::from_str(&quantity).unwrap()
            )
        }).collect();

        Ok(OrderbookEntity { asks, bids })
    }

    fn switch_symbol(&mut self, symbol: &str) {
        self.symbol = symbol.clone().to_string();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use mockito::{mock, server_url};

    #[tokio::test]
    async fn test_fetch_order_book() {
        let mocked_api_response = r#"
        {
            "meta": {
                "code": 200,
                "message": "Successful"
            },
            "data": {
                "asks": [
                    ["100.001", "1.0"],
                    ["100.002", "2.0"]
                ],
                "bids": [
                    ["99.999", "1.5"],
                    ["99.998", "0.5"]
                ]
            }
        }
        "#;

        let _m = mock("GET", "/order-book/v1/books?depth=10&symbol=fBTCBUSD")
            .with_status(200)
            .with_header("content-type", "application/json")
            .with_body(mocked_api_response)
            .create();

        let api = HttpApi::new(&server_url(), "fBTCBUSD");
        let orderbook = api.fetch_order_book().await.unwrap();
        println!("orderbook {:?}", orderbook);

        // Check if the fetched asks and bids are correct
        assert_eq!(orderbook.asks.len(), 2);
        assert_eq!(orderbook.bids.len(), 2);

        assert_eq!(orderbook.asks[0], (Decimal::from_str("100.001").unwrap(), Decimal::from_str("1.0").unwrap()));
        assert_eq!(orderbook.asks[1], (Decimal::from_str("100.002").unwrap(), Decimal::from_str("2.0").unwrap()));
        assert_eq!(orderbook.bids[0], (Decimal::from_str("99.999").unwrap(), Decimal::from_str("1.5").unwrap()));
        assert_eq!(orderbook.bids[1], (Decimal::from_str("99.998").unwrap(), Decimal::from_str("0.5").unwrap()));
    }
}
