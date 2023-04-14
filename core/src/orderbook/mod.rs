use rust_decimal::Decimal;
use rust_decimal_macros::dec;
use std::collections::BTreeMap;

type PriceLevel = (Decimal, Decimal);

#[derive(Debug, PartialEq)]
pub struct OrderBook {
    asks: BTreeMap<Decimal, Decimal>,
    bids: BTreeMap<Decimal, Decimal>,
}

impl OrderBook {
    pub fn new() -> Self {
        OrderBook {
            asks: BTreeMap::new(),
            bids: BTreeMap::new(),
        }
    }

    pub fn initialize(&mut self, asks: Vec<PriceLevel>, bids: Vec<PriceLevel>) {
        // Clear the existing orderbook
        self.asks.clear();
        self.bids.clear();
        for (price, quantity) in asks {
            self.asks.insert(price, quantity);
        }
        for (price, quantity) in bids {
            self.bids.insert(price, quantity);
        }
    }


    pub fn update_order(&mut self, is_ask: bool, updates: Vec<PriceLevel>) {
        let book = if is_ask { &mut self.asks } else { &mut self.bids };
        for (price, quantity) in updates {
            if quantity.is_zero() {
                book.remove(&price);
            } else {
                if let Some(existing_quantity) = book.get_mut(&price) {
                    *existing_quantity = quantity;
                } else {
                    book.insert(price, quantity);
                }
            }
        }
    }


    pub fn compute_dry(&self, fill_amount: Decimal, fill_by_quote: bool, is_buy: bool) -> (Decimal, Decimal, Decimal) {
        // Select asks or bids based on the is_buy flag
        let order_map = if is_buy { &self.bids } else { &self.asks };
        let mut remaining_amount = fill_amount;
        let mut total_quote = Decimal::new(0, 0);
        let mut total_base = Decimal::new(0, 0);

        for (price, quantity) in order_map.iter() {
            let available_amount = if fill_by_quote { *quantity * *price } else { *quantity };
            if remaining_amount >= available_amount {
                remaining_amount -= available_amount;
                total_quote += *quantity * *price;
                total_base += *quantity;
            } else {
                let remaining_quantity = if fill_by_quote {
                    remaining_amount / *price
                } else {
                    remaining_amount
                };
                total_quote += *quantity * *price;
                total_base += remaining_quantity;
                break;
            }
            if remaining_amount.is_zero() {
                break;
            }
        }

        if !remaining_amount.is_zero() {
            return (Decimal::new(0, 0), Decimal::new(0, 0), Decimal::new(0, 0));
        }

        let avg_price = total_quote / total_base;
        let (best_ask, best_bid) = self.get_best_ask_bid();
        let slippage = if is_buy {
            (avg_price - best_bid.unwrap()) / best_bid.unwrap() * Decimal::new(100, 0)
        } else {
            (best_ask.unwrap() - avg_price) / avg_price * Decimal::new(100, 0)
        };

        (avg_price, total_base, slippage)
    }

     pub fn get_depth(&self) -> (Vec<PriceLevel>, Vec<PriceLevel>) {
        let asks: Vec<PriceLevel> = self.asks.iter().map(|(price, quantity)| (*price, *quantity)).collect();
        let mut bids: Vec<PriceLevel> = self.bids.iter().map(|(price, quantity)| (*price, *quantity)).collect();
        bids.reverse();
        (asks, bids)
    }
    pub fn get_best_ask_bid(&self) -> (Option<Decimal>, Option<Decimal>) {
        let best_ask = self.asks.iter().next().map(|(price, _)| *price);
        let best_bid = self.bids.iter().next_back().map(|(price, _)| *price);
        (best_ask, best_bid)
    }

    pub fn group_prices(&self, grouping_size: Decimal) -> (Vec<PriceLevel>, Vec<PriceLevel>) {
        let group = |prices: &BTreeMap<Decimal, Decimal>| -> Vec<PriceLevel> {
            let mut grouped_prices = BTreeMap::new();
            for (price, quantity) in prices {
                let grouped_price = (price / grouping_size).floor() * grouping_size;
                let grouped_quantity = grouped_prices.entry(grouped_price).or_insert(dec!(0.0));
                *grouped_quantity += quantity;
            }
            grouped_prices.into_iter().collect()
        };

        let grouped_asks = group(&self.asks);
        let mut grouped_bids = group(&self.bids);
        grouped_bids.reverse();

        (grouped_asks, grouped_bids)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rust_decimal_macros::dec;

    #[test]
    fn test_order_book_new() {
        let order_book = OrderBook::new();
        assert_eq!(order_book.asks.len(), 0);
        assert_eq!(order_book.bids.len(), 0);
    }

    #[test]
    fn test_order_book_initialize() {
        let mut order_book = OrderBook::new();
        let asks = vec![
            (Decimal::from(100), Decimal::from(10)),
            (Decimal::from(101), Decimal::from(5)),
        ];
        let bids = vec![
            (Decimal::from(99), Decimal::from(8)),
            (Decimal::from(98), Decimal::from(20)),
        ];
        order_book.initialize(asks.clone(), bids.clone());

        assert_eq!(order_book.asks.len(), 2);
        assert_eq!(order_book.bids.len(), 2);
    }

    #[test]
    fn test_order_book_update_order() {
        let mut order_book = OrderBook::new();
        let asks = vec![
            (Decimal::from(100), Decimal::from(10)),
            (Decimal::from(101), Decimal::from(5)),
        ];
        let bids = vec![
            (Decimal::from(99), Decimal::from(8)),
            (Decimal::from(98), Decimal::from(20)),
        ];
        order_book.initialize(asks.clone(), bids.clone());

        order_book.update_order(true, vec![(Decimal::from(100), Decimal::from(15))]);
        assert_eq!(order_book.asks[&Decimal::from(100)], Decimal::from(15));

        order_book.update_order(false, vec![(Decimal::from(98), Decimal::from(0))]);
        assert!(!order_book.bids.contains_key(&Decimal::from(98)));
    }

    #[test]
    fn test_order_book_get_depth() {
        let mut order_book = OrderBook::new();
        let asks = vec![
            (Decimal::from(100), Decimal::from(10)),
            (Decimal::from(101), Decimal::from(5)),
        ];
        let bids = vec![
            (Decimal::from(99), Decimal::from(8)),
            (Decimal::from(98), Decimal::from(20)),
        ];
        order_book.initialize(asks.clone(), bids.clone());

        let (depth_asks, depth_bids) = order_book.get_depth();
        assert_eq!(depth_asks, vec![
                    (Decimal::from(100), Decimal::from(10)),
                    (Decimal::from(101), Decimal::from(5)),
                ]);
        assert_eq!(depth_bids, 
        vec![
            (Decimal::from(99), Decimal::from(8)),
            (Decimal::from(98), Decimal::from(20)),
        ]);
    }
    
    #[test]
    fn test_get_best_ask_bid() {
        let mut orderbook = OrderBook::new();
        orderbook.initialize(
            vec![(dec!(1.0), dec!(1)), (dec!(1.1), dec!(1)), (dec!(1.2), dec!(1)), (dec!(1.3), dec!(1))],
            vec![(dec!(0.9), dec!(1)), (dec!(0.8), dec!(1)), (dec!(0.7), dec!(1)), (dec!(0.6), dec!(1))],
        );

        let (best_ask, best_bid) = orderbook.get_best_ask_bid();
        assert_eq!(best_ask, Some(dec!(1.0)));
        assert_eq!(best_bid, Some(dec!(0.9)));
    }

    #[test]
    fn test_group_prices() {
        let mut orderbook = OrderBook::new();
        orderbook.initialize(
            vec![
                (dec!(1.011), dec!(1.0)),
                (dec!(1.012), dec!(1.0)),
                (dec!(1.013), dec!(1.0)),
                (dec!(1.02), dec!(2.0)),
            ],
            vec![],
        );

        let (grouped_asks, grouped_bids) = orderbook.group_prices(dec!(0.01));

        assert_eq!(
            grouped_asks,
            vec![(dec!(1.01), dec!(3.0)), (dec!(1.02), dec!(2.0))]
        );
        assert_eq!(grouped_bids, vec![]);
    }
       #[test]
    fn test_group_prices2() {
        let asks = vec![
            (dec!(100.001), dec!(1.0)),
            (dec!(100.002), dec!(1.0)),
            (dec!(110.011), dec!(1.0)),
            (dec!(110.012), dec!(1.0)),
            (dec!(110.013), dec!(1.0)),
            (dec!(120.020), dec!(2.0)),
            (dec!(120.110), dec!(1.0)),
        ];

        let bids = vec![
            (dec!(99.999), dec!(1.0)),
            (dec!(99.998), dec!(1.0)),
            (dec!(89.997), dec!(1.0)),
            (dec!(89.996), dec!(1.0)),
            (dec!(79.990), dec!(2.0)),
            (dec!(70.000), dec!(1.0)),
        ];
        let mut order_book = OrderBook::new();
        order_book.initialize(asks.clone(), bids.clone());
        
        // let (grouped_asks_0_001, grouped_bids_0_001) = order_book.group_prices(dec!(0.001));
        // assert_eq!(grouped_asks_0_001, asks);
        // assert_eq!(grouped_bids_0_001, bids);

        let (grouped_asks_0_1, grouped_bids_0_1) = order_book.group_prices(dec!(0.1));
        let expected_asks_0_1 = vec![
            (dec!(100.0), dec!(2.0)),
            (dec!(110.0), dec!(3.0)),
            (dec!(120.0), dec!(2.0)),
            (dec!(120.1), dec!(1.0)),
        ];
        let expected_bids_0_1 = vec![
            (dec!(99.9), dec!(2.0)),
            (dec!(89.9), dec!(2.0)),
            (dec!(79.9), dec!(2.0)),
            (dec!(70.0), dec!(1.0)),
        ];
        assert_eq!(grouped_asks_0_1, expected_asks_0_1);
        assert_eq!(grouped_bids_0_1, expected_bids_0_1);

        let (grouped_asks_1, grouped_bids_1) = order_book.group_prices(dec!(1.0));
        let expected_asks_1 = vec![
            (dec!(100.0), dec!(2.0)),
            (dec!(110.0), dec!(3.0)),
            (dec!(120.0), dec!(3.0)),
        ];
        let expected_bids_1 = vec![
            (dec!(99.0), dec!(2.0)),
            (dec!(89.0), dec!(2.0)),
            (dec!(79.0), dec!(2.0)),
            (dec!(70.0), dec!(1.0)),
        ];
        assert_eq!(grouped_asks_1, expected_asks_1);
        assert_eq!(grouped_bids_1, expected_bids_1);

        let (grouped_asks_10, grouped_bids_10) = order_book.group_prices(dec!(10.0));
        let expected_asks_10 = vec![
            (dec!(100.0), dec!(2.0)),
            (dec!(110.0), dec!(3.0)),
            (dec!(120.0), dec!(3.0)),
        ];
        let expected_bids_10 = vec![
            (dec!(90.0), dec!(2.0)),
            (dec!(80.0), dec!(2.0)),
            (dec!(70.0), dec!(3.0)),
        ];
        assert_eq!(grouped_asks_10, expected_asks_10);
        assert_eq!(grouped_bids_10, expected_bids_10);
    }

}


#[cfg(test)]
mod tests_dry_compute {
    use super::{*};
    use rust_decimal_macros::dec;

    #[test]
    fn test_compute_dry_buy_base_full_filled() {
        let mut orderbook = OrderBook::new();
        orderbook.initialize(
            vec![(dec!(1.0), dec!(1)), (dec!(1.1), dec!(1)), (dec!(1.2), dec!(1)), (dec!(1.3), dec!(1))],
            vec![],
        );

        let (avg_price, filled_qty, slippage) = orderbook.compute_dry(dec!(4), false, true);
        assert_eq!(avg_price, dec!(1.15));
        assert_eq!(filled_qty, dec!(4.0));
        assert_eq!(slippage, dec!(0.0375));
    }

    #[test]
    fn test_compute_dry_buy_base_partial_filled() {
        let mut orderbook = OrderBook::new();
        orderbook.initialize(
            vec![(dec!(1.0), dec!(1)), (dec!(1.1), dec!(1)), (dec!(1.2), dec!(1)), (dec!(1.3), dec!(1))],
            vec![],
        );

        let (avg_price, filled_qty, slippage) = orderbook.compute_dry(dec!(3), false, true);
        assert_eq!(avg_price, dec!(1.1));
        assert_eq!(filled_qty, dec!(3.0));
        assert_eq!(slippage, dec!(0.02777777));
    }

    #[test]
    fn test_compute_dry_buy_quote_fully_filled() {
        let mut orderbook = OrderBook::new();
        orderbook.initialize(
            vec![(dec!(1.0), dec!(1)), (dec!(1.1), dec!(1)), (dec!(1.2), dec!(1)), (dec!(1.3), dec!(1))],
            vec![],
        );

        let (avg_price, filled_qty, slippage) = orderbook.compute_dry(dec!(3), true, false);
        assert_eq!(avg_price, dec!(1.033333));
        assert_eq!(filled_qty, dec!(2.9));
        assert_eq!(slippage, dec!(0.01111111));
    }

    #[test]
    fn test_compute_dry_buy_quote_partial_filled() {
        let mut orderbook = OrderBook::new();
        orderbook.initialize(
            vec![(dec!(1.0), dec!(1)), (dec!(1.1), dec!(1)), (dec!(1.2), dec!(1)), (dec!(1.3), dec!(1))],
            vec![],
        );

        let (avg_price, filled_qty, slippage) = orderbook.compute_dry(dec!(1.5), true, false);
        assert_eq!(avg_price, dec!(1.0));
        assert_eq!(filled_qty, dec!(1.5));
        assert_eq!(slippage, dec!(0.0));
    }

    #[test]
    fn test_compute_dry_sell_base_fully_filled() {
        let mut orderbook = OrderBook::new();
        orderbook.initialize(
            vec![],
            vec![(dec!(0.9), dec!(1)), (dec!(0.8), dec!(1)), (dec!(0.7), dec!(1)), (dec!(0.6), dec!(1))],
        );

        let (avg_price, filled_qty, slippage) = orderbook.compute_dry(dec!(4), false, true);
        assert_eq!(avg_price, dec!(0.75));
        assert_eq!(filled_qty, dec!(4.0));
        assert_eq!(slippage, dec!(0.0375));
    }

    #[test]
    fn test_compute_dry_sell_base_partial_filled() {
        let mut orderbook = OrderBook::new();
        orderbook.initialize(
            vec![],
            vec![(dec!(0.9), dec!(1)), (dec!(0.8), dec!(1)), (dec!(0.7), dec!(1)), (dec!(0.6), dec!(1))],
        );

        let (avg_price, filled_qty, slippage) = orderbook.compute_dry(dec!(2), false, true);
        assert_eq!(avg_price, dec!(0.85));
        assert_eq!(filled_qty, dec!(2.0));
        assert_eq!(slippage, dec!(0.02777777));
    }

    #[test]
    fn test_compute_dry_sell_quote_fully_filled() {
        let mut orderbook = OrderBook::new();
        orderbook.initialize(
            vec![],
            vec![(dec!(0.9), dec!(1)), (dec!(0.8), dec!(1)), (dec!(0.7), dec!(1)), (dec!(0.6), dec!(1))],
        );

        let (avg_price, filled_qty, slippage) = orderbook.compute_dry(dec!(3), true, false);
        assert_eq!(avg_price, dec!(0.75));
        assert_eq!(filled_qty, dec!(4.0));
        assert_eq!(slippage, dec!(0.0375));
    }

    #[test]
    fn test_compute_dry_sell_quote_partial_filled() {
        let mut orderbook = OrderBook::new();
        orderbook.initialize(
            vec![],
            vec![(dec!(0.9), dec!(1)), (dec!(0.8), dec!(1)), (dec!(0.7), dec!(1)), (dec!(0.6), dec!(1))],
        );

        let (avg_price, filled_qty, slippage) = orderbook.compute_dry(dec!(1.5), true, false);
        assert_eq!(avg_price, dec!(0.85));
        assert_eq!(filled_qty, dec!(2.0));
        assert_eq!(slippage, dec!(0.02777777));
    }
}
