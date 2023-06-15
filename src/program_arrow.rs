use web_sys::{WebGl2RenderingContext, WebGlProgram};

pub fn setup_arrow_program(ctx: &WebGl2RenderingContext, arrow_program: &WebGlProgram) -> () {

  let position = ctx.get_attrib_location(&arrow_program, "position");
  ctx.enable_vertex_attrib_array(position as u32);
  ctx.vertex_attrib_pointer_with_i32(
    position as u32,               // location
    2,                             // size (components per iteration)
    WebGl2RenderingContext::FLOAT, // type to get from buffer
    false,                         // normalized
    0,                             // stride (bytes to advance per iteration)
    0,                             // offset (bytes from start of buffer)
    );

}

pub fn buffer_arrow_data(ctx: &WebGl2RenderingContext) -> () {

  // Define the vertices for two triangles that form a sort of arrow
  let vertices: [f32; 12] = [
    0.0, 0.0,
    0.0, 1.0,
    -1.0, 1.1,

    0.0, 0.0,
    0.0, 1.0,
    1.0, 1.1,
  ];


  unsafe {
    let vertex_array = js_sys::Float32Array::view(&vertices);
    ctx.buffer_data_with_array_buffer_view(
      WebGl2RenderingContext::ARRAY_BUFFER,
      &vertex_array,
      WebGl2RenderingContext::STATIC_DRAW,
      );
  }

}
