use reqwest::{
    header::{ACCEPT, ACCEPT_LANGUAGE, CONTENT_TYPE, USER_AGENT},
    Client,
};
use serde_json;
use tauri::http::{HeaderMap, HeaderValue};
use tauri_plugin_log::{Target, TargetKind};
use tauri_plugin_store::StoreExt;

fn common_headers() -> HeaderMap {
    let mut headers = HeaderMap::new();

    headers.insert(
        USER_AGENT,
        HeaderValue::from_static(
            "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:140.0) Gecko/20100101 Firefox/140.0",
        ),
    );
    headers.insert(
        ACCEPT,
        HeaderValue::from_static("application/json, text/javascript, */*; q=0.01"),
    );
    headers.insert(ACCEPT_LANGUAGE, HeaderValue::from_static("en-US,en;q=0.5"));
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_static("application/x-www-form-urlencoded; charset=UTF-8"),
    );
    headers.insert("Sec-GPC", HeaderValue::from_static("1"));
    headers.insert("Sec-Fetch-Dest", HeaderValue::from_static("empty"));
    headers.insert("Sec-Fetch-Mode", HeaderValue::from_static("cors"));
    headers.insert("Sec-Fetch-Site", HeaderValue::from_static("same-site"));
    headers.insert("Pragma", HeaderValue::from_static("no-cache"));
    headers.insert("Cache-Control", HeaderValue::from_static("no-cache"));
    headers.insert(
        "Referer",
        HeaderValue::from_static("https://www.radyofenomen.com/"),
    );

    headers
}

#[tauri::command]
async fn fetch_radios() -> Result<serde_json::Value, String> {
    let client = Client::new();

    // Prepare form data as a HashMap
    let mut form = std::collections::HashMap::new();
    form.insert("client", "web");
    form.insert("lang", "tr");
    form.insert("version", "8");
    form.insert("token", "");
    form.insert("siteID", "20");
    form.insert("devicePlatform", "1");
    form.insert("deviceType", "0");

    let res = client
        .post("https://api.radyofenomen.com/v2/Route/get?url=/radyolar")
        .headers(common_headers())
        .form(&form)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let text = res.text().await.map_err(|e| e.to_string())?;
    let value: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    Ok(value)
}

#[tauri::command]
async fn get_radio(radio_name: &str) -> Result<serde_json::Value, String> {
    log::info!("get_radio: {}", radio_name);
    let client = Client::new();
    let mut form = std::collections::HashMap::new();
    form.insert("client", "web");
    form.insert("lang", "tr");
    form.insert("version", "8");
    form.insert("token", "");
    form.insert("siteID", "20");
    form.insert("devicePlatform", "1");
    form.insert("deviceType", "0");

    let res = client
        .post(format!(
            "https://api.radyofenomen.com/v2/Route/get?url=/radyolar/{}",
            radio_name
        ))
        .headers(common_headers())
        .form(&form)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let text = res.text().await.map_err(|e| e.to_string())?;
    let value: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    Ok(value)
}

#[tauri::command]
async fn get_radio_stream(radio_name: &str, hd: bool) -> Result<serde_json::Value, String> {
    // Send request to https://api.radyofenomen.com/v2/Channels/getSecureLink
    let client = Client::new();
    let mut form = std::collections::HashMap::new();
    form.insert("client", "web");
    form.insert("lang", "tr");
    form.insert("version", "8");
    form.insert("token", "");
    form.insert("siteID", "20");
    form.insert("devicePlatform", "1");
    form.insert("deviceType", "0");
    form.insert("URL", radio_name);
    form.insert("qualityIndex", if hd { "1" } else { "0" });

    let res = client
        .post(format!(
            "https://api.radyofenomen.com/v2/Channels/getSecureLink?url=/radyolar/{}",
            radio_name
        ))
        .headers(common_headers())
        .form(&form)
        .send()
        .await
        .map_err(|e| e.to_string())?;

    let text = res.text().await.map_err(|e| e.to_string())?;
    log::info!("get_radio_stream: {}", text);
    let value: serde_json::Value = serde_json::from_str(&text).map_err(|e| e.to_string())?;

    Ok(value)
}

#[tauri::command]
async fn save_volume(app_handle: tauri::AppHandle, volume: f64) -> Result<(), String> {
    let store = app_handle
        .store("settings.json")
        .map_err(|e| format!("Failed to get store: {}", e))?;
    store.set("volume", volume);
    store
        .save()
        .map_err(|e| format!("Failed to save volume: {}", e))?;
    Ok(())
}

#[tauri::command]
async fn load_volume(app_handle: tauri::AppHandle) -> Result<f64, String> {
    let store = app_handle
        .store("settings.json")
        .map_err(|e| format!("Failed to get store: {}", e))?;
    if let Some(value) = store.get("volume") {
        if let Some(vol) = value.as_f64() {
            return Ok(vol);
        }
    }

    //Spawn new thread to save volume to 1.0 if it is the first time loading the app
    std::thread::spawn(move || {
        if let Ok(store) = app_handle.store("settings.json") {
            store.set("volume", 1.0);
            if let Err(e) = store.save() {
                log::warn!("Failed to save default volume: {}", e);
            }
        }
    });

    Ok(1.0)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .plugin(
            tauri_plugin_log::Builder::new()
                .target(Target::new(TargetKind::Stdout))
                .build(),
        )
        .invoke_handler(tauri::generate_handler![
            fetch_radios,
            get_radio,
            get_radio_stream,
            save_volume,
            load_volume
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
