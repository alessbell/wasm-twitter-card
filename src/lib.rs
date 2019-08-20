mod utils;

use fonterator as font; // For parsing font file.
use footile::{FillRule, Plotter, Raster, Rgba8};
use wasm_bindgen::prelude::*;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
pub fn generate_text(title: &str, author: &str) -> Vec<u8> {
    utils::set_panic_hook();
    const TITLE_FONT_SIZE: f32 = 96.0;
    const SUBTITLE_FONT_SIZE: f32 = 60.0;
    const WIDTH: f32 = 1200.0;
    const HEIGHT: f32 = 630.0;
    const PADDING: f32 = 50.0;

    // Init font, and paths.
    let font = font::monospace_font();

    // Init rendering.
    let mut p = Plotter::new(WIDTH as u32, HEIGHT as u32);
    let mut r = Raster::new(p.width(), p.height());

    // Render title left aligned with line wrapping
    let mut begin = 0;
    let mut line = 0;
    loop {
        let (path, l) = font.render(
            &title[begin..],
            (
                PADDING,
                line as f32 * TITLE_FONT_SIZE + PADDING,
                WIDTH - (PADDING * 2.0),
                HEIGHT,
            ),
            (TITLE_FONT_SIZE, TITLE_FONT_SIZE),
            fonterator::TextAlign::Left,
        );
        let path: Vec<footile::PathOp> = path.collect();
        r.over(p.fill(&path, FillRule::NonZero), Rgba8::rgb(255, 255, 255));
        begin += l;
        line += 1;
        if l == 0 {
            break;
        }
    }

    // let path = font.render(
    //     title,
    //     (0.0, 0.0, WIDTH, HEIGHT),
    //     (TITLE_FONT_SIZE, TITLE_FONT_SIZE),
    //     font::TextAlign::Left
    // ).0;
    // let path: Vec<font::PathOp> = path.collect();
    // r.over(p.fill(&path, FillRule::NonZero), Rgba8::rgb(255, 255, 255));

    // Render author left aligned.
    let path = font
        .render(
            author,
            (
                PADDING,
                HEIGHT - SUBTITLE_FONT_SIZE - PADDING,
                WIDTH - (PADDING * 2.0),
                HEIGHT,
            ),
            (SUBTITLE_FONT_SIZE, SUBTITLE_FONT_SIZE),
            font::TextAlign::Left,
        )
        .0;
    let path: Vec<font::PathOp> = path.collect();
    r.over(p.fill(&path, FillRule::NonZero), Rgba8::rgb(255, 255, 255));

    let x = r.as_u8_slice().to_vec();
    return x;
}
