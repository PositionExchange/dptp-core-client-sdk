use tokio::sync::{Mutex};
use std::sync::{Arc};
use core_pkg::orderbook::OrderBook;
pub type LockedOrderBook = Arc<Mutex<OrderBook>>;
pub type Ob = OrderBook;
