use crate::types::{Radio, RadioImage};

pub fn get_currently_playing_song_image(
    radio_image: RadioImage,
    size: &str,
    size2: Option<&str>,
) -> String {
    let size2 = size2.unwrap_or(size);
    let image_url = format!(
        "{}{}x{}{}",
        radio_image.prefix, size, size2, radio_image.suffix
    );

    image_url
}

pub fn get_logo_url(radio: &Radio, size: &str, size2: Option<&str>) -> String {
    let size2 = size2.unwrap_or(size);
    format!("{}{}", radio.image.prefix, radio.image.suffix)
        .replace("/u", &format!("/{}x{}/u", size, size2))
}

pub fn get_button_text(
    is_selected: bool,
    is_playing: bool,
    selected_radio: Option<usize>,
    current_idx: usize,
) -> &'static str {
    if is_selected && is_playing {
        "Çalıyor"
    } else if !is_playing && selected_radio == Some(current_idx) {
        "Durduruldu"
    } else {
        "Seç & Dinle"
    }
}
