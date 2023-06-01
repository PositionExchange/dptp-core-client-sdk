#![allow(
    non_camel_case_types,
    unused,
    clippy::redundant_closure,
    clippy::useless_conversion,
    clippy::unit_arg,
    clippy::double_parens,
    non_snake_case
)]
// AUTO GENERATED FILE, DO NOT EDIT.
// Generated by `flutter_rust_bridge`.

use crate::api::*;
use flutter_rust_bridge::*;

// Section: imports

// Section: wire functions

#[no_mangle]
pub extern "C" fn wire_initialize_orders(
    port_: i64,
    asks: *mut wire_uint_8_list,
    bids: *mut wire_uint_8_list,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "initialize_orders",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_asks = asks.wire2api();
            let api_bids = bids.wire2api();
            move |task_callback| Ok(initialize_orders(api_asks, api_bids))
        },
    )
}

#[no_mangle]
pub extern "C" fn wire_new_pair_order_compute(
    port_: i64,
    pair_symbol: *mut wire_uint_8_list,
    collateral_long_token: *mut wire_uint_8_list,
    collateral_short_token: *mut wire_uint_8_list,
    leverage: *mut wire_uint_8_list,
    max_notional: *mut wire_uint_8_list,
    min_quantity_base: *mut wire_uint_8_list,
    margin_ratio: *mut wire_uint_8_list,
    taker_fee: *mut wire_uint_8_list,
    maker_fee: *mut wire_uint_8_list,
    base_token_precision: u32,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "new_pair_order_compute",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_pair_symbol = pair_symbol.wire2api();
            let api_collateral_long_token = collateral_long_token.wire2api();
            let api_collateral_short_token = collateral_short_token.wire2api();
            let api_leverage = leverage.wire2api();
            let api_max_notional = max_notional.wire2api();
            let api_min_quantity_base = min_quantity_base.wire2api();
            let api_margin_ratio = margin_ratio.wire2api();
            let api_taker_fee = taker_fee.wire2api();
            let api_maker_fee = maker_fee.wire2api();
            let api_base_token_precision = base_token_precision.wire2api();
            move |task_callback| {
                Ok(new_pair_order_compute(
                    api_pair_symbol,
                    api_collateral_long_token,
                    api_collateral_short_token,
                    api_leverage,
                    api_max_notional,
                    api_min_quantity_base,
                    api_margin_ratio,
                    api_taker_fee,
                    api_maker_fee,
                    api_base_token_precision,
                ))
            }
        },
    )
}

#[no_mangle]
pub extern "C" fn wire_change_leverage(
    port_: i64,
    new_leverage: *mut wire_uint_8_list,
    max_notional: *mut wire_uint_8_list,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "change_leverage",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_new_leverage = new_leverage.wire2api();
            let api_max_notional = max_notional.wire2api();
            move |task_callback| Ok(change_leverage(api_new_leverage, api_max_notional))
        },
    )
}

#[no_mangle]
pub extern "C" fn wire_get_active_pair(port_: i64) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "get_active_pair",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Ok(get_active_pair()),
    )
}

#[no_mangle]
pub extern "C" fn wire_change_active_pair(port_: i64, new_active_pair: *mut wire_uint_8_list) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "change_active_pair",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_new_active_pair = new_active_pair.wire2api();
            move |task_callback| Ok(change_active_pair(api_new_active_pair))
        },
    )
}

#[no_mangle]
pub extern "C" fn wire_check_pair_exists(port_: i64, new_active_pair: *mut wire_uint_8_list) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "check_pair_exists",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_new_active_pair = new_active_pair.wire2api();
            move |task_callback| Ok(check_pair_exists(api_new_active_pair))
        },
    )
}

#[no_mangle]
pub extern "C" fn wire_update_balance(
    port_: i64,
    token: *mut wire_uint_8_list,
    balance: *mut wire_uint_8_list,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "update_balance",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_token = token.wire2api();
            let api_balance = balance.wire2api();
            move |task_callback| Ok(update_balance(api_token, api_balance))
        },
    )
}

#[no_mangle]
pub extern "C" fn wire_compute_open_order(
    port_: i64,
    pay_token: *mut wire_uint_8_list,
    pay_amount: *mut wire_uint_8_list,
    limit_price: *mut wire_uint_8_list,
    quantity: *mut wire_uint_8_list,
    is_quote: bool,
    is_buy: bool,
    use_percentage: bool,
) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "compute_open_order",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || {
            let api_pay_token = pay_token.wire2api();
            let api_pay_amount = pay_amount.wire2api();
            let api_limit_price = limit_price.wire2api();
            let api_quantity = quantity.wire2api();
            let api_is_quote = is_quote.wire2api();
            let api_is_buy = is_buy.wire2api();
            let api_use_percentage = use_percentage.wire2api();
            move |task_callback| {
                Ok(compute_open_order(
                    api_pay_token,
                    api_pay_amount,
                    api_limit_price,
                    api_quantity,
                    api_is_quote,
                    api_is_buy,
                    api_use_percentage,
                ))
            }
        },
    )
}

#[no_mangle]
pub extern "C" fn wire_get_order_book_manager(port_: i64) {
    FLUTTER_RUST_BRIDGE_HANDLER.wrap(
        WrapInfo {
            debug_name: "get_order_book_manager",
            port: Some(port_),
            mode: FfiCallMode::Normal,
        },
        move || move |task_callback| Ok(get_order_book_manager()),
    )
}

// Section: wire structs

#[repr(C)]
#[derive(Clone)]
pub struct wire_uint_8_list {
    ptr: *mut u8,
    len: i32,
}

// Section: wrapper structs

// Section: static checks

// Section: allocate functions

#[no_mangle]
pub extern "C" fn new_uint_8_list(len: i32) -> *mut wire_uint_8_list {
    let ans = wire_uint_8_list {
        ptr: support::new_leak_vec_ptr(Default::default(), len),
        len,
    };
    support::new_leak_box_ptr(ans)
}

// Section: impl Wire2Api

pub trait Wire2Api<T> {
    fn wire2api(self) -> T;
}

impl<T, S> Wire2Api<Option<T>> for *mut S
where
    *mut S: Wire2Api<T>,
{
    fn wire2api(self) -> Option<T> {
        if self.is_null() {
            None
        } else {
            Some(self.wire2api())
        }
    }
}

impl Wire2Api<String> for *mut wire_uint_8_list {
    fn wire2api(self) -> String {
        let vec: Vec<u8> = self.wire2api();
        String::from_utf8_lossy(&vec).into_owned()
    }
}

impl Wire2Api<bool> for bool {
    fn wire2api(self) -> bool {
        self
    }
}

impl Wire2Api<u32> for u32 {
    fn wire2api(self) -> u32 {
        self
    }
}

impl Wire2Api<u8> for u8 {
    fn wire2api(self) -> u8 {
        self
    }
}

impl Wire2Api<Vec<u8>> for *mut wire_uint_8_list {
    fn wire2api(self) -> Vec<u8> {
        unsafe {
            let wrap = support::box_from_leak_ptr(self);
            support::vec_from_leak_ptr(wrap.ptr, wrap.len)
        }
    }
}

// Section: impl NewWithNullPtr

pub trait NewWithNullPtr {
    fn new_with_null_ptr() -> Self;
}

impl<T> NewWithNullPtr for *mut T {
    fn new_with_null_ptr() -> Self {
        std::ptr::null_mut()
    }
}

// Section: impl IntoDart

// Section: executor

support::lazy_static! {
    pub static ref FLUTTER_RUST_BRIDGE_HANDLER: support::DefaultHandler = Default::default();
}

// Section: sync execution mode utility

#[no_mangle]
pub extern "C" fn free_WireSyncReturnStruct(val: support::WireSyncReturnStruct) {
    unsafe {
        let _ = support::vec_from_leak_ptr(val.ptr, val.len);
    }
}