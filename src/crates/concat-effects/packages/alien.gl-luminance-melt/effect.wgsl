// SPDX-License-Identifier: MIT
// Ported to WGSL from the gl-transitions library (https://github.com/gl-transitions/gl-transitions),
// transitions/LuminanceMelt.glsl
// Author: 0gust1
// License: MIT
// See THIRD_PARTY_NOTICES.md for the licence text.
fn gl_mod289_3(x: vec3<f32>) -> vec3<f32> { return x - floor(x * (1.0 / 289.0)) * 289.0; }
fn gl_mod289_2(x: vec2<f32>) -> vec2<f32> { return x - floor(x * (1.0 / 289.0)) * 289.0; }
fn gl_permute(x: vec3<f32>) -> vec3<f32> { return gl_mod289_3(((x * 34.0) + 1.0) * x); }
fn gl_snoise(v: vec2<f32>) -> f32 {
  let C = vec4<f32>(0.211324865405187, 0.366025403784439, -0.577350269189626, 0.024390243902439);
  var i = floor(v + dot(v, C.yy));
  let x0 = v - i + dot(i, C.xx);
  var i1 = vec2<f32>(0.0, 1.0);
  if (x0.x > x0.y) { i1 = vec2<f32>(1.0, 0.0); }
  var x12 = x0.xyxy + C.xxzz;
  x12 = vec4<f32>(x12.xy - i1, x12.zw);
  i = gl_mod289_2(i);
  let p = gl_permute(gl_permute(vec3<f32>(i.y) + vec3<f32>(0.0, i1.y, 1.0)) + vec3<f32>(i.x) + vec3<f32>(0.0, i1.x, 1.0));
  var m = max(vec3<f32>(0.5) - vec3<f32>(dot(x0, x0), dot(x12.xy, x12.xy), dot(x12.zw, x12.zw)), vec3<f32>(0.0));
  m = m * m;
  m = m * m;
  let x = 2.0 * fract(p * C.www) - vec3<f32>(1.0);
  let h = abs(x) - vec3<f32>(0.5);
  let ox = floor(x + vec3<f32>(0.5));
  let a0 = x - ox;
  m = m * (vec3<f32>(1.79284291400159) - 0.85373472095314 * (a0 * a0 + h * h));
  let g = vec3<f32>(a0.x * x0.x + h.x * x0.y, a0.y * x12.x + h.y * x12.y, a0.z * x12.z + h.z * x12.w);
  return 130.0 * dot(m, g);
}
fn gl_luminance(c: vec4<f32>) -> f32 { return c.r * 0.299 + c.g * 0.587 + c.b * 0.114; }
fn gl_transition(p: vec2<f32>, progress: f32) -> vec4<f32> {
  // Melt downwards the parts darker than the threshold (the original defaults).
  let l_threshold = 0.8;
  let center = vec2<f32>(1.0, 1.0);
  if (progress == 0.0) { return getFromColor(p); }
  if (progress == 1.0) { return getToColor(p); }
  let dist = distance(center, p) - progress * exp(gl_snoise(vec2<f32>(p.x, 0.0)));
  let r = progress - gl_rand(vec2<f32>(p.x, 0.1));
  var m = progress * progress * progress;
  if (dist <= r && gl_luminance(getFromColor(p)) < l_threshold) { m = 1.0; }
  return mix(getFromColor(p), getToColor(p), m);
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
