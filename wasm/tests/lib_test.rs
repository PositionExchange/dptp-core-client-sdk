#[cfg(test)]
mod tests {
    use super::*;
    use js_sys::Array;
    use wasm::OrderBookManager;
    use wasm_bindgen::JsValue;
    use wasm_bindgen_test::*;
    
    #[test]
    fn test_compute_dry() {
        let mut orderbook_manager = OrderBookManager::new();

        let asks: Vec<(String, String)> = vec![
            ("101".into(), "1".into()),
            ("102".into(), "1".into()),
            ("103".into(), "2".into()),
            ("104".into(), "2".into()),
            ("105".into(), "3".into()),
        ];

        let bids: Vec<(String, String)> = vec![
            ("100".into(), "1".into()),
            ("99".into(), "1".into()),
            ("98".into(), "2".into()),
            ("97".into(), "2".into()),
            ("96".into(), "3".into()),
        ];

        // Create a new `Array` with the same length as the `asks` vector.
        let mut asks_js = Array::new_with_length(asks.len() as u32);

        // Iterate over the `asks` vector and convert each tuple to a `String` array.
        for (i, (price, quantity)) in asks.iter().enumerate() {
            let mut tuple = Array::new();
            tuple.push(&JsValue::from(price.as_str()));
            tuple.push(&JsValue::from(quantity.as_str()));
            asks_js.set(i as u32, tuple.into());
        }

        // Create a new `Array` with the same length as the `bids` vector.
        let mut bids_js = Array::new_with_length(bids.len() as u32);

        // Iterate over the `bids` vector and convert each tuple to a `String` array.
        for (i, (price, quantity)) in bids.iter().enumerate() {
            let mut tuple = Array::new();
            tuple.push(&JsValue::from(price.as_str()));
            tuple.push(&JsValue::from(quantity.as_str()));
            bids_js.set(i as u32, tuple.into());
        }

        // `asks_js` and `bids_js` are now `Array` of `String` arrays.
        orderbook_manager.initialize_orders(asks_js, bids_js);

        let result = orderbook_manager.compute_dry("4".into(), false, true).unwrap();
        // let expected_result = (
        //     "103.2500000000".into(),
        //     "4.0000000000".into(),
        //     "2.1978021978".into(),
        // );
        println!("{:?}", result);
        // assert_eq!(result, expected_result);
    }
}
