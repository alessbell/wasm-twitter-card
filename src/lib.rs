mod utils;
use glyph_brush_layout::{
    rusttype::{point, Font, Scale},
    *,
};
use image::{DynamicImage, Rgba};
use serde_wasm_bindgen;
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

// A macro to provide `println!(..)`-style syntax for `console.log` logging.
macro_rules! log {
    ( $( $t:tt )* ) => {
        web_sys::console::log_1(&format!( $( $t )* ).into());
    }
}

#[wasm_bindgen]
pub fn generate_text(
    title: &str,
    author: &str,
    title_font_size: i32,
    subtitle_font_size: i32,
    rgb: JsValue,
    has_serif_font: bool,
) -> Vec<u8> {
    utils::set_panic_hook();
    const WIDTH: f32 = 1200.0;
    const HEIGHT: f32 = 630.0;
    const PADDING: f32 = 50.0;

    let rgb_value: (u8, u8, u8) = serde_wasm_bindgen::from_value(rgb).unwrap();

    // Load the font
    // let font_data = include_bytes!("../fonts/GT-Pressura-Mono-Bold.ttf");
    // This only succeeds if collection consists of one font
    let font = Font::from_bytes(&include_bytes!("../fonts/GT-Pressura-Mono-Bold.ttf")[..])
        .expect("Error constructing Font");
    // // Load the font
    // let font_data = include_bytes!("../fonts/GT-Pressura-Mono-Bold.ttf");
    // // This only succeeds if collection consists of one font
    // let font = Font::from_bytes(font_data as &[u8]).expect("Error constructing Font");

    // let dejavu =
    //     Font::from_bytes(&include_bytes!("../fonts/GT-Pressura-Mono-Bold.ttf")[..]).unwrap();
    // let garamond =
    //     Font::from_bytes(&include_bytes!("../fonts/GT-Pressura-Mono-Bold.ttf")[..]).unwrap();
    let fonts = vec![font];

    let title_glyphs: Vec<_> = Layout::default().calculate_glyphs(
        &fonts,
        &SectionGeometry {
            screen_position: (0.0, 0.0),
            bounds: (WIDTH, HEIGHT),
        },
        &[SectionText {
            text: title,
            scale: Scale::uniform(title_font_size as f32),
            font_id: FontId(0),
            color: [0.0, 1.0, 0.0, 1.0],
        }],
    );

    let author_glyphs: Vec<_> = Layout::default().calculate_glyphs(
        &fonts,
        &SectionGeometry {
            screen_position: (0.0, 0.0),
            bounds: (WIDTH, HEIGHT),
        },
        &[SectionText {
            text: author,
            scale: Scale::uniform(subtitle_font_size as f32),
            font_id: FontId(0),
            color: [0.0, 1.0, 0.0, 1.0],
        }],
    );

    // Create a new rgba image with some padding
    let mut image = DynamicImage::new_rgba8(WIDTH as u32, HEIGHT as u32).to_rgba();

    for glyph in title_glyphs {
        if let Some(bounding_box) = glyph.0.pixel_bounding_box() {
            // Draw the glyph into the image per-pixel by using the draw closure
            // log!("{:?}", glyph);
            glyph.0.draw(|x, y, v| {
                image.put_pixel(
                    // Offset the position by the glyph bounding box
                    x + bounding_box.min.x as u32,
                    y + bounding_box.min.y as u32,
                    // Turn the coverage into an alpha value
                    Rgba([rgb_value.0, rgb_value.1, rgb_value.2, (v * 255.0) as u8]),
                )
            });
        }
    }

    for glyph in author_glyphs {
        if let Some(bounding_box) = glyph.0.pixel_bounding_box() {
            // Draw the glyph into the image per-pixel by using the draw closure
            glyph.0.draw(|x, y, v| {
                image.put_pixel(
                    // Offset the position by the glyph bounding box
                    x + bounding_box.min.x as u32,
                    (HEIGHT as u32 - bounding_box.max.y as u32) + (y + bounding_box.min.y as u32)
                        - PADDING as u32,
                    // y + bounding_box.min.y as u32 + HEIGHT as u32 - PADDING as u32 - 60,
                    // Turn the coverage into an alpha value
                    Rgba([rgb_value.0, rgb_value.1, rgb_value.2, (v * 255.0) as u8]),
                )
            });
        }
    }

    return image.to_vec();
}
