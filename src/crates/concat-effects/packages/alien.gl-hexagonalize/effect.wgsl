// SPDX-License-Identifier: MIT
// Ported to WGSL from the gl-transitions library (https://github.com/gl-transitions/gl-transitions),
// transitions/Hexagonalize.glsl
// Author: Fernando Kuteken
// License: MIT
// See THIRD_PARTY_NOTICES.md for the licence text.
fn gl_hex_round(h: vec3<f32>) -> vec3<f32> {
  var q = floor(h.x + 0.5);
  var r = floor(h.y + 0.5);
  var s = floor(h.z + 0.5);
  let dq = abs(q - h.x);
  let dr = abs(r - h.y);
  let ds = abs(s - h.z);
  if (dq > dr && dq > ds) { q = -r - s; }
  else if (dr > ds) { r = -q - s; }
  else { s = -q - r; }
  return vec3<f32>(q, r, -q - r);
}
fn gl_hex_from_point(point_in: vec2<f32>, size: f32, ratio: f32) -> vec3<f32> {
  var point = point_in;
  point.y = point.y / ratio;
  point = (point - 0.5) / size;
  let q = (sqrt(3.0) / 3.0) * point.x + (-1.0 / 3.0) * point.y;
  let r = 2.0 / 3.0 * point.y;
  return gl_hex_round(vec3<f32>(q, r, -q - r));
}
fn gl_point_from_hex(h: vec3<f32>, size: f32, ratio: f32) -> vec2<f32> {
  let x = (sqrt(3.0) * h.x + (sqrt(3.0) / 2.0) * h.y) * size + 0.5;
  let y = (3.0 / 2.0) * h.y * size + 0.5;
  return vec2<f32>(x, y * ratio);
}
fn gl_transition(uv: vec2<f32>, progress: f32) -> vec4<f32> {
  let ratio = gl_ratio();
  let steps = 50.0;
  let horizontal_hexagons = 20.0;
  var dist = 2.0 * min(progress, 1.0 - progress);
  dist = ceil(dist * steps) / steps;
  let size = (sqrt(3.0) / 3.0) * dist / horizontal_hexagons;
  var point = uv;
  if (dist > 0.0) { point = gl_point_from_hex(gl_hex_from_point(uv, size, ratio), size, ratio); }
  return mix(getFromColor(point), getToColor(point), progress);
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
