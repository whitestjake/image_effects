
use std::io::Cursor;
use wasm_bindgen::prelude::wasm_bindgen;
use web_sys::console::log_1 as log;
use base64::{Engine as _, engine::{general_purpose}};
use image::{load_from_memory, imageops, ImageOutputFormat::Png};





#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String {

    log(&"Grayscale called".into());

    let base64_to_vector = general_purpose::STANDARD.decode(encoded_file).unwrap();
    log(&"Image decoded".into());

    let img = load_from_memory(&base64_to_vector).unwrap();
    log(&"Image Loaded".into());
    
    let _gray_img = imageops::grayscale(&img);
    log(&"Grayscale effect applied".into());

    let mut buffer = vec![];
    img.write_to(&mut Cursor::new(&mut buffer), Png).unwrap();
    log(&"New Image Written".into());

    let encoded_img = general_purpose::STANDARD.encode(&buffer);
    let data_url = format!(
        "data:image/png;base64,{}",
        encoded_img
    );

    data_url

}


