// SPDX-License-Identifier: MIT
// Ported to WGSL from the gl-transitions library (https://github.com/gl-transitions/gl-transitions),
// transitions/Pinwheel.glsl
// Author: Mr Speaker
// License: MIT
// See THIRD_PARTY_NOTICES.md for the licence text.
fn gl_transition(p: vec2<f32>, progress: f32) -> vec4<f32> {
  let speed = 2.0;
  let circ_pos = atan2(p.y - 0.5, p.x - 0.5) + progress * speed;
  let mod_pos = gl_mod(circ_pos, 3.1415 / 4.0);
  let sgn = sign(progress - mod_pos);
  return mix(getToColor(p), getFromColor(p), step(sgn, 0.5));
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
