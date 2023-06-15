use web_sys::{WebGl2RenderingContext, WebGlProgram};

use crate::vector::Vector;

pub fn setup_stage_program(
  ctx: &WebGl2RenderingContext,
  stage_program: &WebGlProgram,
  dimensions: &Vector
) -> () {

  let position = ctx.get_attrib_location(&stage_program, "position");
  ctx.enable_vertex_attrib_array(position as u32);
  ctx.vertex_attrib_pointer_with_i32(
    position as u32,               // indx
    2,                             // size
    WebGl2RenderingContext::FLOAT, // type_
    false,                         // normalized
    0,                             // stride
    0,                             // offset
    );

  if let Some(resolution_location) = ctx.get_uniform_location(&stage_program, "u_resolution") {
    ctx.uniform2f(Some(&resolution_location), dimensions.0 as f32, dimensions.1 as f32);
  }

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
