use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast; // Appears to enable casting (e.g. for Element -> HTMLCanvasElement)

mod draw;
mod raf;

use crate::draw::*;
use crate::raf::*;

// ---------------------------------------------------------------------

struct App {
    ctx: web_sys::CanvasRenderingContext2d,
    handle: AnimationFrame,
    prevtime: f64,
}

static mut APP: Option<App> = None;

#[allow(illegal_floating_point_literal_pattern)]
fn on_animation_frame(nexttime: f64) {
    let app = unsafe { APP.as_mut().unwrap() };
    let delta = match app.prevtime {
        0.0 => 1.0,
        x => nexttime - x,
    } as f32;

    app.prevtime = nexttime;

    web_sys::console::log_1(&format!("nexttime: {}, delta: {}", nexttime, delta).into());
    draw_grid(&app.ctx);

    app.handle = request_animation_frame(on_animation_frame);
}

// ---------------------------------------------------------------------

#[wasm_bindgen(start)]
pub fn main() -> Result<(), JsValue> {
    web_sys::console::log_1(&"Running WASM_".into());

    let document = web_sys::window().unwrap().document().unwrap();
    let canvas = document.get_element_by_id("canvas-main").unwrap();
    let canvas: web_sys::HtmlCanvasElement = canvas
        .dyn_into::<web_sys::HtmlCanvasElement>()
        .map_err(|_| ())
        .unwrap();

    let context = canvas
        .get_context("2d")
        .unwrap()
        .unwrap()
        .dyn_into::<web_sys::CanvasRenderingContext2d>()
        .unwrap();

    unsafe {
        APP = Some(App {
            ctx: context,
            handle: request_animation_frame(on_animation_frame),
            prevtime: 0.0,
        });
    }

    Ok(())
}
