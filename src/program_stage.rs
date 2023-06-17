use web_sys::{WebGl2RenderingContext, WebGlProgram};

pub fn setup_stage_program(
  ctx: &WebGl2RenderingContext,
  stage_program: &WebGlProgram,
) -> () {

  let position = ctx.get_attrib_location(&stage_program, "a_position");
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

pub fn buffer_stage_data(ctx: &WebGl2RenderingContext) -> () {

  // Define the vertices for two triangles that form a square
  let vertices: [f32; 12] = [
    -1.0, -1.0,
    1.0, -1.0,
    -1.0, 1.0,
    -1.0, 1.0,
    1.0, -1.0,
    1.0, 1.0,
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
