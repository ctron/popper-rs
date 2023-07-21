/// Log a debug message to the browser console
#[allow(unused_macros)]
macro_rules! debug_active {
    ($s1:expr) => {
        web_sys::console::debug_1(&wasm_bindgen::JsValue::from($s1));
    };
    ($s1:expr, $s2:expr) => {
        web_sys::console::debug_2(
            &wasm_bindgen::JsValue::from($s1),
            &wasm_bindgen::JsValue::from($s2),
        );
    };
    ($s1:expr, $s2:expr, $s3:expr) => {
        web_sys::console::debug_3(
            &wasm_bindgen::JsValue::from($s1),
            &wasm_bindgen::JsValue::from($s2),
            &wasm_bindgen::JsValue::from($s3),
        );
    };
    ($s1:expr, $s2:expr, $s3:expr, $s4:expr) => {
        web_sys::console::debug_4(
            &wasm_bindgen::JsValue::from($s1),
            &wasm_bindgen::JsValue::from($s2),
            &wasm_bindgen::JsValue::from($s3),
            &wasm_bindgen::JsValue::from($s4),
        );
    };
}

#[allow(unused_macros)]
macro_rules! debug_inactive {
    ($s1:expr) => {};
    ($s1:expr, $s2:expr) => {};
    ($s1:expr, $s2:expr, $s3:expr) => {};
    ($s1:expr, $s2:expr, $s3:expr, $s4:expr) => {};
}

#[cfg(feature = "debug")]
pub(crate) use debug_active as debug;

#[cfg(not(feature = "debug"))]
pub(crate) use debug_inactive as debug;
