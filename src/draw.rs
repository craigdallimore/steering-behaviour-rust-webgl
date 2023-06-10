use web_sys::{WebGl2RenderingContext, WebGlBuffer};

pub fn draw_stage(
  ctx: &web_sys::WebGl2RenderingContext,
  stage_vertex_buffer: &WebGlBuffer
) -> () {

  ctx.bind_buffer(WebGl2RenderingContext::ARRAY_BUFFER, Some(&stage_vertex_buffer));

  ctx.draw_arrays(
    WebGl2RenderingContext::TRIANGLES,
    0, // starting point
    6  // number of points to draw
  );
}

/*
pub fn draw_arrow<'a>(
    ctx: &'a web_sys::CanvasRenderingContext2d,
    c: &Kinematic,
) -> &'a web_sys::CanvasRenderingContext2d {
    ctx.save();

    ctx.transform(1.0, 0.0, 0.0, 1.0, c.position.0, c.position.1)
        .unwrap();
    ctx.set_fill_style(&"rgb(240, 98, 146)".into());
    ctx.rotate(c.orientation).unwrap();
    ctx.move_to(0.0, -10.0);
    ctx.begin_path();
    ctx.line_to(5.0, 10.0);
    ctx.line_to(0.0, 5.0);
    ctx.line_to(-5.0, 10.0);
    ctx.line_to(0.0, -10.0);
    ctx.fill();

    ctx.restore();
    ctx
}
*/
