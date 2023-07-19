use wasm_bindgen::{prelude::Closure, JsCast, JsValue};
use web_sys::{console, WebGl2RenderingContext, WebGlProgram, WebGlShader};

// https://rustwasm.github.io/wasm-bindgen/examples/webgl.html
pub fn compile_shader(
    ctx: &WebGl2RenderingContext,
    shader_type: u32,
    source: &str,
) -> Result<WebGlShader, String> {
    let shader = ctx
        .create_shader(shader_type)
        .ok_or_else(|| String::from("Unable to create shader object"))?;

    ctx.shader_source(&shader, source);
    ctx.compile_shader(&shader);

    let compile_status = ctx
        .get_shader_parameter(&shader, WebGl2RenderingContext::COMPILE_STATUS)
        .as_bool()
        .unwrap_or(false);

    if compile_status {
        Ok(shader)
    } else {
        let msg = ctx
            .get_shader_info_log(&shader)
            .unwrap_or_else(|| String::from("Unknown error creating shader"));

        Err(msg)
    }
}

pub fn link_program(
    ctx: &WebGl2RenderingContext,
    vert_shader: &WebGlShader,
    frag_shader: &WebGlShader,
) -> Result<WebGlProgram, String> {
    let program = ctx
        .create_program()
        .ok_or_else(|| String::from("Unable to create program"))?;

    ctx.attach_shader(&program, vert_shader);
    ctx.attach_shader(&program, frag_shader);
    ctx.link_program(&program);

    let link_status = ctx
        .get_program_parameter(&program, WebGl2RenderingContext::LINK_STATUS)
        .as_bool()
        .unwrap_or(false);

    if link_status {
        Ok(program)
    } else {
        let msg = ctx
            .get_program_info_log(&program)
            .unwrap_or_else(|| String::from("Unknown error linking program"));

        Err(msg)
    }
}

pub fn make_program(
    ctx: &WebGl2RenderingContext,
    vs_id: &str,
    fs_id: &str,
) -> Result<WebGlProgram, String> {
    let vert_shader_content = get_shader_string_by_id(String::from(vs_id));

    let vert_shader = compile_shader(
        &ctx,
        WebGl2RenderingContext::VERTEX_SHADER,
        &vert_shader_content,
    )?;

    let frag_shader_content = get_shader_string_by_id(String::from(fs_id));

    let frag_shader = compile_shader(
        &ctx,
        WebGl2RenderingContext::FRAGMENT_SHADER,
        &frag_shader_content,
    )?;

    let program = link_program(&ctx, &vert_shader, &frag_shader)?;

    Ok(program)
}

pub fn get_shader_string_by_id(id: String) -> String {
    let document = web_sys::window().unwrap().document().unwrap();
    let element = document
        .get_element_by_id(&id)
        .unwrap()
        .dyn_into::<web_sys::Element>()
        .unwrap();
    element.text_content().unwrap()
}

pub fn get_context() -> Result<(WebGl2RenderingContext, f32, f32), JsValue> {
    let window = web_sys::window().unwrap();
    let document = window.document().unwrap();

    let canvas = document.get_element_by_id("canvas-main").unwrap();
    let c = canvas.clone();

    let parent = canvas.parent_element().unwrap();

    let cb = move || {
        let p = parent.get_bounding_client_rect();
        let width = p.width();
        let height = p.height();

        c.set_attribute("width", &width.to_string()).unwrap();
        c.set_attribute("height", &height.to_string()).unwrap();
    };

    cb();

    let on_resize = Closure::<dyn FnMut()>::new(cb);

    window.add_event_listener_with_callback("resize", on_resize.as_ref().unchecked_ref())?;

    on_resize.forget();

    let canvas: web_sys::HtmlCanvasElement = canvas.dyn_into::<web_sys::HtmlCanvasElement>()?;
    let ctx = canvas
        .get_context("webgl2")?
        .unwrap()
        .dyn_into::<WebGl2RenderingContext>()?;

    Ok((ctx, canvas.width() as f32, canvas.height() as f32))
}
