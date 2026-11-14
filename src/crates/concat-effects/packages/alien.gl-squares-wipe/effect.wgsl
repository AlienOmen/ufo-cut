// SPDX-License-Identifier: MIT
// Ported to WGSL from the gl-transitions library (https://github.com/gl-transitions/gl-transitions),
// transitions/SquaresWipe.glsl
// Author: gre
// License: MIT
// See THIRD_PARTY_NOTICES.md for the licence text.
fn gl_transition(p: vec2<f32>, progress: f32) -> vec4<f32> {
  let squares = vec2<f32>(10.0, 10.0);
  let direction = vec2<f32>(1.0, -0.5);
  let smoothness = 1.6;
  let center = vec2<f32>(0.5, 0.5);
  var v = normalize(direction);
  v = v / (abs(v.x) + abs(v.y));
  let d = v.x * center.x + v.y * center.y;
  let offset = smoothness;
  let pr = smoothstep(-offset, 0.0, v.x * p.x + v.y * p.y - (d - 0.5 + progress * (1.0 + offset)));
  let squarep = fract(p * squares);
  let squaremin = vec2<f32>(pr / 2.0);
  let squaremax = vec2<f32>(1.0 - pr / 2.0);
  let a = (1.0 - step(progress, 0.0)) * step(squaremin.x, squarep.x) * step(squaremin.y, squarep.y) * step(squarep.x, squaremax.x) * step(squarep.y, squaremax.y);
  return mix(getFromColor(p), getToColor(p), a);
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
