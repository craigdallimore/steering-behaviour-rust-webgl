use web_sys::{WebGl2RenderingContext, WebGlProgram};

use crate::vector::Vector;

pub fn setup_arrow_program(
  ctx: &WebGl2RenderingContext,
  arrow_program: &WebGlProgram,
  dimensions: &Vector
) -> () {

  let position = ctx.get_attrib_location(&arrow_program, "a_position");
  ctx.enable_vertex_attrib_array(position as u32);
  ctx.vertex_attrib_pointer_with_i32(
    position as u32,               // location
    2,                             // size (components per iteration)
    WebGl2RenderingContext::FLOAT, // type to get from buffer
    false,                         // normalized
    0,                             // stride (bytes to advance per iteration)
    0,                             // offset (bytes from start of buffer)
    );

  if let Some(resolution_location) = ctx.get_uniform_location(&arrow_program, "u_resolution") {
    ctx.uniform2f(Some(&resolution_location), dimensions.0 as f32, dimensions.1 as f32);
  }

}

pub fn buffer_arrow_data(ctx: &WebGl2RenderingContext) -> () {

  //      +1y
  //
  // -1x        +1x
  //
  //      -1y

  // Define the vertices for two triangles that form a sort of arrow
  let vertices: [f32; 12] = [
    0.0, 0.5,
    0.5, -0.5,
    0.0, -0.25,

    0.0, 0.5,
    -0.5, -0.5,
    0.0, -0.25,
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
