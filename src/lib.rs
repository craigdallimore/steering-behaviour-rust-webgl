use wasm_bindgen::prelude::*;
use wasm_bindgen::JsCast; // Appears to enable casting (e.g. for Element -> HTMLCanvasElement)

fn draw_grid(ctx: web_sys::CanvasRenderingContext2d) -> web_sys::CanvasRenderingContext2d {
    ctx.save();
    ctx.set_stroke_style(&"rgb(227, 242, 253)".into());

    for x in (10..800).step_by(10) {
        ctx.begin_path();
        ctx.move_to(x as f64, 0.0);
        ctx.line_to(x as f64, 800.0);
        ctx.stroke();
    }
    for z in (10..800).step_by(10) {
        ctx.begin_path();
        ctx.move_to(0.0, z as f64);
        ctx.line_to(800.0, z as f64);
        ctx.stroke();
    }

    ctx.restore();
    ctx
}

#[wasm_bindgen]
pub fn run() {
    web_sys::console::log_1(&"Running WASM".into());

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

    draw_grid(context);
}
