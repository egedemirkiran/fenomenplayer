use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;

use crate::types::{RadioDetailsData, RadioDetailsResponse, RadioStreamData};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "core"])]
    pub async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

pub async fn get_radio_stream_url(radio_name: &str, hd: bool) -> String {
    let obj = js_sys::Object::new();
    if let Err(_) = js_sys::Reflect::set(
        &obj,
        &JsValue::from_str("radioName"),
        &JsValue::from_str(radio_name),
    ) {
        return String::new();
    }
    if let Err(_) = js_sys::Reflect::set(&obj, &JsValue::from_str("hd"), &JsValue::from_bool(hd)) {
        return String::new();
    }
    let js_val = invoke("get_radio_stream", JsValue::from(obj)).await;

    from_value::<RadioStreamData>(js_val)
        .map(|result| result.data.URL)
        .unwrap_or_default()
}

pub async fn get_radio_details(radio_url: &str) -> Option<RadioDetailsData> {
    let obj = js_sys::Object::new();
    if let Err(_) = js_sys::Reflect::set(
        &obj,
        &JsValue::from_str("radioName"),
        &JsValue::from_str(radio_url),
    ) {
        return None;
    }
    let js_val = invoke("get_radio", JsValue::from(obj)).await;
    from_value::<RadioDetailsResponse>(js_val)
        .ok()
        .map(|r| r.data)
}
