use web_sys::WebGl2RenderingContext;

pub fn draw_stage(
  ctx: &web_sys::WebGl2RenderingContext,
) -> () {
  ctx.draw_arrays(
    WebGl2RenderingContext::TRIANGLES,
    0, // starting point
    6  // number of points to draw
  );
}

pub fn draw_arrow(
  ctx: &web_sys::WebGl2RenderingContext,
) -> () {
  ctx.draw_arrays(
    WebGl2RenderingContext::TRIANGLES,
    0, // starting point
    6  // number of points to draw
  );
}
