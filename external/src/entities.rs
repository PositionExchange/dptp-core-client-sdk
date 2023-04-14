use rust_decimal::Decimal;

type PriceLevel = (Decimal, Decimal);

#[derive(Debug, Clone)]
pub struct OrderbookEntity {
    pub asks: Vec<PriceLevel>,
    pub bids: Vec<PriceLevel>,
}

impl OrderbookEntity {
    pub fn new(asks: Vec<PriceLevel>, bids: Vec<PriceLevel>) -> Self {
        Self {
            asks,
            bids
        }
    }
}

