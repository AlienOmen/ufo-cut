// SPDX-License-Identifier: MIT
// Ported to WGSL from the gl-transitions library (https://github.com/gl-transitions/gl-transitions),
// transitions/ZoomInCircles.glsl
// Author: dycm8009
// License: MIT
// See THIRD_PARTY_NOTICES.md for the licence text.
fn gl_zoom(uv: vec2<f32>, amount: f32) -> vec2<f32> {
  return vec2<f32>(0.5) + ((uv - vec2<f32>(0.5)) * amount);
}
fn gl_transition(uv_in: vec2<f32>, progress: f32) -> vec4<f32> {
  var uv = uv_in;
  let ratio = gl_ratio();
  let ratio2 = vec2<f32>(1.0, 1.0 / ratio);
  let r = 2.0 * ((uv - vec2<f32>(0.5)) * ratio2);
  let pro = progress / 0.8;
  var z = pro * 0.2;
  var t = 0.0;
  if (pro > 1.0) {
    z = 0.2 + (pro - 1.0) * 5.0;
    t = clamp((progress - 0.8) / 0.07, 0.0, 1.0);
  }
  let len = length(r);
  if (len < 0.5 + z) {
  } else if (len < 0.8 + z * 1.5) {
    uv = gl_zoom(uv, 1.0 - 0.15 * pro);
    t = t * 0.5;
  } else if (len < 1.2 + z * 2.5) {
    uv = gl_zoom(uv, 1.0 - 0.2 * pro);
    t = t * 0.2;
  } else {
    uv = gl_zoom(uv, 1.0 - 0.25 * pro);
  }
  return mix(getFromColor(uv), getToColor(uv), t);
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
