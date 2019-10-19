mod utils;
use footile::{FillRule, Plotter, Raster, Rgba8};
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
    let rgb_value: (u8, u8, u8) = serde_wasm_bindgen::from_value(rgb).unwrap();

    const WIDTH: f32 = 1200.0;
    const HEIGHT: f32 = 630.0;
    const PADDING: f32 = 50.0;

    // Init font, and paths
    let font = if has_serif_font {
        fonterator::normal_font()
    } else {
        fonterator::monospace_font()
    };

    // Init rendering
    let mut plotter = Plotter::new(WIDTH as u32, HEIGHT as u32);
    let mut raster = Raster::new(plotter.width(), plotter.height());

    // Render title left aligned with line wrapping
    let mut begin = 0;
    let mut line = 0;
    loop {
        let (path, l) = font.render(
            &title[begin..],
            (
                PADDING,
                line as f32 * title_font_size as f32 + PADDING,
                WIDTH - (PADDING * 2.0),
                HEIGHT,
            ),
            (title_font_size as f32, title_font_size as f32),
            fonterator::TextAlign::Left,
        );
        let path: Vec<footile::PathOp> = path.collect();
        raster.over(
            plotter.fill(&path, FillRule::NonZero),
            Rgba8::rgb(rgb_value.0, rgb_value.1, rgb_value.2),
        );
        begin += l;
        line += 1;
        if l == 0 {
            break;
        }
    }

    // Render author left aligned.
    let path = font
        .render(
            author,
            (
                PADDING,
                HEIGHT - subtitle_font_size as f32 - PADDING,
                WIDTH - (PADDING * 2.0),
                HEIGHT,
            ),
            (subtitle_font_size as f32, subtitle_font_size as f32),
            fonterator::TextAlign::Left,
        )
        .0;
    let path: Vec<fonterator::PathOp> = path.collect();
    raster.over(
        plotter.fill(&path, FillRule::NonZero),
        Rgba8::rgb(rgb_value.0, rgb_value.1, rgb_value.2),
    );

    // Return a Vec<T>
    let x = raster.as_u8_slice().to_vec();
    return x;
}
