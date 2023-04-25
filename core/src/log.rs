use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = console)]
    fn log(s: &str);
}

#[cfg(not(target_arch = "wasm32"))]
pub fn console_log(s: String) {
    unsafe {
        println!("{}", s);
    }
}
//
#[cfg(target_arch = "wasm32")]
pub fn console_log(s: String) {
    unsafe {
        log(format!("RUST:: (dptp-core) {}", s).as_str())
    }
}

#[macro_export]
macro_rules! clg {
    ($($t:tt)*) => (crate::log::console_log(format_args!($($t)*).to_string()))
}
