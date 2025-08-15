use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/public/hls.js")]
extern "C" {
    #[wasm_bindgen(js_name = attachHlsToAudio)]
    pub fn attach_hls_to_audio(audio_id: &str, stream_url: &str, volume: f64);

    #[wasm_bindgen(js_name = pauseRadio)]
    pub fn pause_radio() -> bool;

    #[wasm_bindgen(js_name = stopRadio)]
    pub fn stop_radio() -> bool;

    #[wasm_bindgen(js_name = setAudioVolume)]
    pub fn set_audio_volume(volume: f64);
}
