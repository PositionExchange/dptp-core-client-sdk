#[derive(Debug)]

pub enum Side {
    Bid,
    Ask,

}


#[derive(Debug)]
pub enum OrderStatus {
    Uninitialized,
    Created,
    Filled,
    PartiallyFilled,
}

