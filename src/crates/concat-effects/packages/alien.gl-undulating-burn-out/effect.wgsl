// SPDX-License-Identifier: MIT
// Ported to WGSL from the gl-transitions library (https://github.com/gl-transitions/gl-transitions),
// transitions/UndulatingBurnOut.glsl
// Author: pthrasher
// License: MIT
// See THIRD_PARTY_NOTICES.md for the licence text.
const SMOOTHNESS: f32 = 0.03;
fn gl_quad_in_out(t: f32) -> f32 {
  let p = 2.0 * t * t;
  if (t < 0.5) { return p; }
  return -p + (4.0 * t) - 1.0;
}
fn gl_get_gradient(r: f32, dist: f32) -> f32 {
  let d = r - dist;
  return mix(
    smoothstep(-SMOOTHNESS, 0.0, r - dist * (1.0 + SMOOTHNESS)),
    -1.0 - step(0.005, d),
    step(-0.005, d) * step(d, 0.01)
  );
}
fn gl_get_wave(p: vec2<f32>, progress: f32, center: vec2<f32>) -> f32 {
  let q = p - center;
  let rads = atan2(q.y, q.x);
  var degs = degrees(rads) + 180.0;
  let ratio = (GL_PI * 30.0) / 360.0;
  degs = degs * ratio;
  let x = progress;
  let magnitude = mix(0.02, 0.09, smoothstep(0.0, 1.0, x));
  let offset = mix(40.0, 30.0, smoothstep(0.0, 1.0, x));
  let ease_degs = gl_quad_in_out(sin(degs));
  let deg_wave_pos = (ease_degs * magnitude) * sin(x * offset);
  return x - max(SMOOTHNESS, 0.01) * (1.0 - smoothstep(0.0, 0.1, x)) + deg_wave_pos;
}
fn gl_transition(p: vec2<f32>, progress: f32) -> vec4<f32> {
  let center = vec2<f32>(0.5, 0.5);
  let color = vec4<f32>(0.0, 0.0, 0.0, 1.0);
  let dist = distance(center, p);
  let m = gl_get_gradient(gl_get_wave(p, progress, center), dist);
  let cfrom = getFromColor(p);
  let cto = getToColor(p);
  return mix(mix(cfrom, cto, m), mix(cfrom, color, 0.75), step(m, -2.0));
}

const GL_PI: f32 = 3.141592653589793;
fn getFromColor(p: vec2<f32>) -> vec4<f32> { return from_at(vec2<f32>(p.x, 1.0 - p.y)); }
fn getToColor(p: vec2<f32>) -> vec4<f32> { return to_at(vec2<f32>(p.x, 1.0 - p.y)); }
fn gl_ratio() -> f32 { return frame.size.x / max(frame.size.y, 1.0); }
fn gl_mod(x: f32, y: f32) -> f32 { return x - y * floor(x / y); }
fn gl_mod2(x: vec2<f32>, y: f32) -> vec2<f32> { return x - y * floor(x / y); }
fn gl_rand(co: vec2<f32>) -> f32 { return fract(sin(dot(co, vec2<f32>(12.9898, 78.233))) * 43758.5453); }
// gl-transitions draw with y pointing up; this frame has y pointing down.
fn transition(uv: vec2<f32>, progress: f32) -> vec4<f32> {
    return gl_transition(vec2<f32>(uv.x, 1.0 - uv.y), clamp(progress, 0.0, 1.0));
}
