use serde_wasm_bindgen::from_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use yew::prelude::*;

use futures::future::join_all;
use gloo_timers::callback::Timeout;
use web_sys::InputEvent;
use yew_icons::{Icon, IconId};

use crate::api::{get_radio_details, get_radio_stream_url, invoke};
use crate::audio::{attach_hls_to_audio, pause_radio, set_audio_volume, stop_radio};
use crate::styles::{play_btn_classes, radio_btn_classes, radio_card_classes};
use crate::types::{ApiResponse, Radio, RadioData, RadioDetailsData};
use crate::utils::{get_button_text, get_currently_playing_song_image, get_logo_url};

#[function_component(App)]
pub fn app() -> Html {
    let radios = use_state(|| None as Option<Vec<Radio>>);
    let selected_radio = use_state(|| None as Option<usize>);
    let fetch_error = use_state(|| None as Option<String>);
    let audio_ref = use_node_ref();
    let is_playing = use_state(|| true);
    let volume = use_state(|| 1.0f32);
    let radios_data = use_state(|| None as Option<Vec<RadioData>>);
    let is_manually_stopped = use_state(|| false);
    let radio_details = use_state(|| None as Option<RadioDetailsData>);
    let polling_timeout = use_mut_ref(|| None as Option<Timeout>);
    // Removed is_dark and toggle_dark

    // Removed dark mode logic

    // Load saved volume on app startup
    {
        let volume = volume.clone();
        use_effect_with((), move |_| {
            let volume = volume.clone();
            spawn_local(async move {
                let js_val = invoke("load_volume", JsValue::NULL).await;
                match from_value::<f64>(js_val) {
                    Ok(saved_volume) => {
                        volume.set(saved_volume as f32);
                        // Apply the loaded volume to the audio element immediately
                        set_audio_volume(saved_volume);
                    }
                    Err(_) => {
                        // Volume loading failed, using default
                    }
                }
            });
            || ()
        });
    }

    {
        let radios = radios.clone();
        let fetch_error = fetch_error.clone();
        let radios_data = radios_data.clone();
        use_effect_with((), move |_| {
            let radios = radios.clone();
            let fetch_error = fetch_error.clone();
            let radios_data = radios_data.clone();
            spawn_local(async move {
                let js_val = invoke("fetch_radios", JsValue::NULL).await;
                match from_value::<ApiResponse>(js_val.clone()) {
                    Ok(response) => {
                        radios.set(Some(response.data.list.clone()));

                        let stream_url_futures = response
                            .data
                            .list
                            .iter()
                            .map(|radio| get_radio_stream_url(&radio.URL, true))
                            .collect::<Vec<_>>();

                        // Run all in parallel to get all stream URLs at once
                        let stream_urls = join_all(stream_url_futures).await;

                        let radios_data_temp = response
                            .data
                            .list
                            .iter()
                            .zip(stream_urls.into_iter())
                            .map(|(radio, stream_url)| RadioData {
                                stream_url,
                                logo_url: get_logo_url(radio, "400", Some("400")),
                                id: radio.URL.clone(),
                                name: radio.name.clone(),
                                title: radio.title.clone().unwrap_or_default(),
                                description: radio.description.clone().unwrap_or_default(),
                            })
                            .collect::<Vec<_>>();

                        radios_data.set(Some(radios_data_temp));
                    }
                    Err(e) => {
                        fetch_error.set(Some(format!("Veri alınırken hata oluştu: {}", e)));
                    }
                }
            });
            || ()
        });
    }

    let on_select_radio = {
        let selected_radio = selected_radio.clone();
        let is_playing = is_playing.clone();
        let is_manually_stopped = is_manually_stopped.clone();
        let radios_data = radios_data.clone();
        let volume = volume.clone();
        let radio_details = radio_details.clone();
        Callback::from(move |idx: usize| {
            stop_radio();
            selected_radio.set(Some(idx));
            is_playing.set(true);
            is_manually_stopped.set(false);
            let radios_data = radios_data.clone();
            let volume = volume.clone();
            let radio_details = radio_details.clone();
            spawn_local(async move {
                if let Some(data) = radios_data.as_ref().and_then(|d| d.get(idx)) {
                    // Fetch radio details and currently playing song
                    let details = get_radio_details(&data.id).await;
                    radio_details.set(details);
                    if !data.stream_url.ends_with(".m3u8") {
                        set_audio_volume(*volume as f64);
                    }
                }
            });
        })
    };

    let on_audio_end = {
        let selected_radio = selected_radio.clone();
        let is_playing = is_playing.clone();
        let is_manually_stopped = is_manually_stopped.clone();
        Callback::from(move |_| {
            stop_radio();
            selected_radio.set(None);
            is_playing.set(false);
            is_manually_stopped.set(false);
        })
    };

    let on_play_pause = {
        let is_playing = is_playing.clone();
        let is_manually_stopped = is_manually_stopped.clone();
        let selected_radio = selected_radio.clone();
        let radios_data = radios_data.clone();
        let volume = volume.clone();
        // log selected radio
        Callback::from(move |_| {
            if *is_playing {
                pause_radio();
                is_manually_stopped.set(true);
            } else {
                if let Some(idx) = *selected_radio {
                    if let Some(data) = radios_data.as_ref().and_then(|d| d.get(idx)) {
                        if data.stream_url.ends_with(".m3u8") && *is_manually_stopped {
                            attach_hls_to_audio("main-audio", &data.stream_url, *volume as f64);
                        }
                    }
                }
                set_audio_volume(*volume as f64);
                is_manually_stopped.set(false);
            }
            is_playing.set(!*is_playing);
        })
    };

    let on_volume_change = {
        let volume = volume.clone();
        Callback::from(move |e: InputEvent| {
            if let Some(input) = e.target_dyn_into::<web_sys::HtmlInputElement>() {
                let val = input.value().parse::<f32>().unwrap_or(1.0);
                if (val - *volume).abs() > 0.01 {
                    set_audio_volume(val as f64);
                    volume.set(val);

                    // Save volume to configuration
                    let volume_to_save = val;
                    spawn_local(async move {
                        let obj = js_sys::Object::new();
                        if let Err(_) = js_sys::Reflect::set(
                            &obj,
                            &JsValue::from_str("volume"),
                            &JsValue::from_f64(volume_to_save as f64),
                        ) {
                            return;
                        }
                        let js_val = invoke("save_volume", JsValue::from(obj)).await;
                        match from_value::<()>(js_val) {
                            Ok(_) => {
                                // Volume saved successfully
                            }
                            Err(_) => {
                                // Volume save failed
                            }
                        }
                    });
                }
            }
        })
    };

    {
        let selected_radio = selected_radio.clone();
        let radios_data = radios_data.clone();
        let is_manually_stopped = is_manually_stopped.clone();
        let is_playing = is_playing.clone();
        let volume = volume.clone();
        use_effect_with(
            (*selected_radio, *is_manually_stopped, *is_playing),
            move |(selected_radio, is_manually_stopped, is_playing)| {
                if let Some(idx) = *selected_radio {
                    if let Some(data) = radios_data.as_ref().and_then(|d| d.get(idx)) {
                        if data.stream_url.ends_with(".m3u8")
                            && !*is_manually_stopped
                            && *is_playing
                        {
                            attach_hls_to_audio("main-audio", &data.stream_url, *volume as f64);
                        }
                    }
                }
                || ()
            },
        );
    }

    {
        let selected_radio = selected_radio.clone();
        let radios_data = radios_data.clone();
        let is_playing = is_playing.clone();
        let radio_details = radio_details.clone();
        let polling_timeout = polling_timeout.clone();
        use_effect_with(
            (*selected_radio, *is_playing),
            move |(selected_radio, is_playing)| {
                if let Some(timeout) = polling_timeout.borrow_mut().take() {
                    timeout.cancel();
                }
                if let (Some(idx), true) = (selected_radio, *is_playing) {
                    if let Some(data) = radios_data.as_ref().and_then(|d| d.get(*idx)) {
                        let radio_id = data.id.clone();
                        let radio_details = radio_details.clone();
                        let polling_timeout_ref = polling_timeout.clone();
                        spawn_local(async move {
                            let details = get_radio_details(&radio_id).await;
                            let remaining = details
                                .as_ref()
                                .and_then(|d| d.timeline.as_ref())
                                .and_then(|t| t.first())
                                .and_then(|song| song.remainingSeconds)
                                .unwrap_or(10);
                            radio_details.set(details);

                            let timeout = Timeout::new(remaining * 1000, move || {
                                let radio_id = radio_id.clone();
                                let radio_details = radio_details.clone();
                                spawn_local(async move {
                                    let details = get_radio_details(&radio_id).await;
                                    radio_details.set(details);
                                });
                            });
                            *polling_timeout_ref.borrow_mut() = Some(timeout);
                        });
                    }
                }
                move || {
                    if let Some(timeout) = polling_timeout.borrow_mut().take() {
                        timeout.cancel();
                    }
                }
            },
        );
    }

    html! {
        <>
        <main class={classes!(
            "container", "mx-auto", "max-w-5xl", "px-4", "py-10", "font-sans", "min-h-screen", "transition-colors", "duration-300",
            "text-gray-900",
            if selected_radio.is_some() { "pb-32" } else { "" }
        )}>
            <div class="flex justify-between items-center mb-10">
                <h1 class="text-4xl font-extrabold text-center text-red-600 tracking-tight drop-shadow-sm flex-1">{"Radyo Fenomen"}</h1>
            </div>
            if let Some(error) = &*fetch_error {
                <p class="text-red-600 text-center font-semibold mb-4 text-lg bg-red-50 border border-red-200 rounded-lg py-2">{ error }</p>
            } else if let Some(radios_data_ref) = radios_data.as_ref() {
                <div class="grid grid-cols-1 sm:grid-cols-2 md:grid-cols-3 gap-8">
                    { for radios_data_ref.iter().enumerate().map(|(idx, radio)| {
                        let is_selected = Some(idx) == *selected_radio;
                        html! {
                            <div
                                class={classes!(radio_card_classes(is_selected), "transition-transform", "duration-200", "ease-in-out", "group")}
                                style="--radio-color: #F2050F"
                                onclick={
                                    let on_select_radio = on_select_radio.clone();
                                    move |_| on_select_radio.emit(idx)
                                }
                                tabindex="0"
                            >
                                <img src={radio.logo_url.clone()} alt={radio.name.clone()} class="w-24 h-24 rounded-2xl object-cover mb-4 border-4 border-white shadow-md group-hover:scale-105 transition-transform duration-200" />
                                <div class="text-xl font-bold text-gray-900 mb-1 truncate w-full text-center">{ &radio.name }</div>
                                <div class="text-gray-600 text-base mb-3 text-center min-h-[2em] truncate w-full">
                                    {
                                        if !radio.title.is_empty() {
                                            radio.title.clone()
                                        } else if !radio.description.is_empty() && radio.description != "test" {
                                            radio.description.clone()
                                        } else {
                                            radio.name.clone()
                                        }
                                    }
                                </div>
                                <button class={classes!(radio_btn_classes(is_selected, *is_playing), "w-full", "py-2", "text-base", "shadow-sm", "transition-all", "duration-150")}
                                    >
                                    { get_button_text(is_selected, *is_playing, *selected_radio, idx) }
                                </button>
                                { if is_selected {
                                    html! {
                                        <span class="mt-3 px-4 py-1 text-xs rounded-full bg-red-100 text-red-600 font-semibold shadow">{"Seçili"}</span>
                                    }
                                } else { html!{} } }
                            </div>
                        }
                    }) }
                </div>
            } else {
                <p class="text-gray-500 text-center mt-12 text-lg animate-pulse">{"Yükleniyor..."}</p>
            }
        </main>
        { selected_radio.and_then(|idx| radios_data.as_ref().and_then(|data| data.get(idx))).map({
            let volume = volume.clone();
            let radio_details = radio_details.clone();
            move |radio| {
                let volume_clone = volume.clone();
                let details = (*radio_details).clone();
                let song_info = details.as_ref().and_then(|d| d.timeline.as_ref().and_then(|t| t.first()));
                let song_image_url = song_info.and_then(|song| song.image.clone().map(|img| get_currently_playing_song_image(img, "300", Some("300")))).unwrap_or_else(|| radio.logo_url.clone());
                html! {
                    <div class={classes!(
                        "fixed", "bottom-0", "left-0", "w-full", "backdrop-blur-md", "shadow-2xl", "flex", "flex-col", "md:flex-row", "justify-between", "items-center", "px-6", "py-4", "z-50", "border-t", "transition-colors", "duration-300",
                        "bg-white/80", "border-gray-200", "text-gray-900"
                    )}>
                        // Left: Song details
                        <div class="flex flex-row items-center min-w-0 max-w-xs w-full md:w-auto mb-2 md:mb-0">
                            <img src={song_image_url} alt="Şarkı görseli" class="w-14 h-14 rounded-xl object-cover border-2 border-gray-200 mr-4 shadow" />
                            <div class="flex flex-col items-start min-w-0">
                                { if let Some(song) = song_info {
                                    html! {
                                        <span class="font-medium text-gray-800 max-w-[200px] text-base">
                                            { format!("{} - {}", song.artistTitle.clone().unwrap_or_default(), song.songTitle.clone().unwrap_or_default()) }
                                        </span>
                                    }
                                } else { html!{} } }
                            </div>
                        </div>
                        // Center: Radio image, name, description/title
                        <div class="flex flex-row items-center justify-center flex-1 min-w-0 w-full md:w-auto">
                            <img src={radio.logo_url.clone()} alt={radio.name.clone()} class="w-14 h-14 rounded-full object-cover mr-4 shadow-xl" />
                            <div class="flex flex-col items-start min-w-0">
                                <span class="font-semibold text-gray-900 truncate text-lg">{ &radio.name }</span>
                                <span class="text-gray-600 text-sm truncate">{ if !radio.title.is_empty() { radio.title.clone() } else { radio.description.clone() } }</span>
                            </div>
                        </div>
                        // Right: Controls
                        <div class="flex items-center ml-auto w-full md:w-auto justify-end">
                            <button class={classes!(play_btn_classes(*is_playing),
                                "hover:bg-red-100", "focus:ring-red-400",
                                "transition-colors", "duration-150", "focus:outline-none", "focus:ring-2"
                            )}
                                onclick={on_play_pause.clone()} aria-label={if *is_playing { "Durdur" } else { "Çal" }}>
                                { if *is_playing {
                                    html! {
                                        <Icon icon_id={IconId::HeroiconsSolidPause} class="text-red-600" />
                                    }
                                } else {
                                    html! {
                                        <Icon icon_id={IconId::HeroiconsSolidPlay} class="text-red-600" />
                                    }
                                } }
                            </button>
                            <input type="range" min="0" max="1" step="0.01" value={volume_clone.to_string()} oninput={on_volume_change} class={classes!(
                                "ml-6", "w-36", "h-2", "rounded-lg", "appearance-none", "cursor-pointer", "transition-all",
                                "bg-red-100", "focus:ring-red-400"
                            )} style="accent-color: #ef4444;" aria-label="Ses" />
                        </div>
                        <audio
                            key={radio.id.clone()}
                            id="main-audio"
                            ref={audio_ref.clone()}
                            src={radio.stream_url.clone()}
                            onended={on_audio_end.clone()}
                            controls={false}
                            style="display: none;"
                        />
                    </div>
                }
            }
        }) }
        </>
    }
}
