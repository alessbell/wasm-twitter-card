mod utils;
// use footile::{FillRule, Plotter, Raster, Rgba8};
use image::{DynamicImage, Rgba};
use rusttype::{point, Font, Scale};
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
    let font_data = include_bytes!("../fonts/GT-Pressura-Mono-Bold.ttf");
    // This only succeeds if collection consists of one font
    let font = Font::from_bytes(font_data as &[u8]).expect("Error constructing Font");

    // Title font size to use
    let title_scale = Scale::uniform(title_font_size as f32);
    let author_scale = Scale::uniform(subtitle_font_size as f32);

    let v_title_metrics = font.v_metrics(title_scale);
    let v_author_metrics = font.v_metrics(author_scale);

    // layout the glyphs in a line with 10 pixels padding
    let title_glyphs: Vec<_> = font
        .layout(title, title_scale, point(0.0, 0.0 + v_title_metrics.ascent))
        .collect();
    let author_glyphs: Vec<_> = font
        .layout(
            author,
            author_scale,
            point(0.0, 0.0 + v_author_metrics.ascent),
        )
        .collect();

    // work out the layout size
    // let glyphs_height = (v_title_metrics.ascent - v_title_metrics.descent).ceil() as u32;
    // let glyphs_width = {
    //     let min_x = title_glyphs
    //         .first()
    //         .map(|g| g.pixel_bounding_box().unwrap().min.x)
    //         .unwrap();
    //     let max_x = title_glyphs
    //         .last()
    //         .map(|g| g.pixel_bounding_box().unwrap().max.x)
    //         .unwrap();
    //     (max_x - min_x) as u32
    // };

    // log!("height: {}", glyphs_height);
    // log!("width: {}", glyphs_width);

    // Create a new rgba image with some padding
    let mut image = DynamicImage::new_rgba8(WIDTH as u32, HEIGHT as u32).to_rgba();

    // Loop through the glyphs in the text, positing each one on a line
    for glyph in title_glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            // Draw the glyph into the image per-pixel by using the draw closure
            log!("{:?}", glyph);
            glyph.draw(|x, y, v| {
                // log!("position x: {}", glyph.position().x);
                if x + bounding_box.min.x as u32 + PADDING as u32 >= WIDTH as u32 {
                    image.put_pixel(
                        // Offset the position by the glyph bounding box
                        x + bounding_box.min.x as u32 + PADDING as u32 - WIDTH as u32,
                        y + bounding_box.min.y as u32 + glyph.position().y as u32 + PADDING as u32,
                        // Turn the coverage into an alpha value
                        Rgba([rgb_value.0, rgb_value.1, rgb_value.2, (v * 255.0) as u8]),
                    )
                } else {
                    image.put_pixel(
                        // Offset the position by the glyph bounding box
                        x + bounding_box.min.x as u32 + PADDING as u32,
                        y + bounding_box.min.y as u32 + PADDING as u32,
                        // Turn the coverage into an alpha value
                        Rgba([rgb_value.0, rgb_value.1, rgb_value.2, (v * 255.0) as u8]),
                    )
                }
            });
        }
    }

    for glyph in author_glyphs {
        if let Some(bounding_box) = glyph.pixel_bounding_box() {
            // Draw the glyph into the image per-pixel by using the draw closure
            glyph.draw(|x, y, v| {
                image.put_pixel(
                    // Offset the position by the glyph bounding box
                    x + bounding_box.min.x as u32 + PADDING as u32,
                    y + bounding_box.min.y as u32 + HEIGHT as u32 - PADDING as u32 - 60,
                    // Turn the coverage into an alpha value
                    Rgba([rgb_value.0, rgb_value.1, rgb_value.2, (v * 255.0) as u8]),
                )
            });
        }
    }

    return image.to_vec();

    //////
    ////
    //
    ////
    /////

    // // Init font, and paths
    // let font = if has_serif_font {
    //     fonterator::normal_font()
    // } else {
    //     fonterator::monospace_font()
    // };

    // // Init rendering
    // let mut plotter = Plotter::new(WIDTH as u32, HEIGHT as u32);
    // let mut raster = Raster::new(plotter.width(), plotter.height());

    // // Render title left aligned with line wrapping
    // let mut begin = 0;
    // let mut line = 0;
    // loop {
    //     let (path, l) = font.render(
    //         &title[begin..],
    //         (
    //             PADDING,
    //             line as f32 * title_font_size as f32 + PADDING,
    //             WIDTH - (PADDING * 2.0),
    //             HEIGHT,
    //         ),
    //         (title_font_size as f32, title_font_size as f32),
    //         fonterator::TextAlign::Left,
    //     );
    //     let path: Vec<footile::PathOp> = path.collect();
    //     raster.over(
    //         plotter.fill(&path, FillRule::NonZero),
    //         Rgba8::rgb(rgb_value.0, rgb_value.1, rgb_value.2),
    //     );
    //     begin += l;
    //     line += 1;
    //     if l == 0 {
    //         break;
    //     }
    // }

    // // Return a Vec<T>
    // let x = raster.as_u8_slice().to_vec();
    // return x;
}
