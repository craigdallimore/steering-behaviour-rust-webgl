use web_sys::{WebGl2RenderingContext, WebGlProgram};

use crate::canvas::{get_shader_string_by_id, compile_shader, link_program};

pub fn get_stage_program(ctx: &WebGl2RenderingContext) -> Result<WebGlProgram, String> {

  let vert_shader_content = get_shader_string_by_id(String::from("vs-stage"));

  let vert_shader = compile_shader(
    &ctx,
    WebGl2RenderingContext::VERTEX_SHADER,
    &vert_shader_content
  )?;

  let frag_shader_content = get_shader_string_by_id(String::from("fs-stage"));

  let frag_shader = compile_shader(
    &ctx,
    WebGl2RenderingContext::FRAGMENT_SHADER,
    &frag_shader_content
  )?;

  let program = link_program(
    &ctx,
    &vert_shader,
    &frag_shader,
  )?;

  setup_stage_program(ctx, &program);

  Ok(program)

}

fn setup_stage_program(ctx: &WebGl2RenderingContext, stage_program: &WebGlProgram) -> () {

  // Define the vertices for two triangles that form a square
  let vertices: [f32; 12] = [
    -1.0, -1.0,
    1.0, -1.0,
    -1.0, 1.0,
    -1.0, 1.0,
    1.0, -1.0,
    1.0, 1.0,
  ];

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

  unsafe {
    let vertex_array = js_sys::Float32Array::view(&vertices);
    ctx.buffer_data_with_array_buffer_view(
      WebGl2RenderingContext::ARRAY_BUFFER,
      &vertex_array,
      WebGl2RenderingContext::STATIC_DRAW,
      );
  }




}
