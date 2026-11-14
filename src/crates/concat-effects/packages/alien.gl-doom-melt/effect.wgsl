// SPDX-License-Identifier: MIT
// Ported to WGSL from the gl-transitions library (https://github.com/gl-transitions/gl-transitions),
// transitions/DoomMelt.glsl
// Author: Zeh Fernando
// License: MIT
// See THIRD_PARTY_NOTICES.md for the licence text.
const BARS: i32 = 30;
const AMPLITUDE: f32 = 2.0;
const NOISE: f32 = 0.1;
const FREQUENCY: f32 = 0.5;
const DRIP_SCALE: f32 = 0.5;
fn gl_doom_rand(num: i32) -> f32 {
  let n = f32(num);
  return fract(gl_mod(n * 67123.313, 12.0) * sin(n * 10.3) * cos(n));
}
fn gl_doom_wave(num: i32) -> f32 {
  let fnum = f32(num) * FREQUENCY * 0.1 * f32(BARS);
  return cos(fnum * 0.5) * cos(fnum * 0.13) * sin((fnum + 10.0) * 0.3) / 2.0 + 0.5;
}
fn gl_doom_drip(num: i32) -> f32 {
  return sin(f32(num) / f32(BARS - 1) * 3.141592) * DRIP_SCALE;
}
fn gl_doom_pos(num: i32) -> f32 {
  return mix(gl_doom_wave(num), gl_doom_rand(num), NOISE) + gl_doom_drip(num);
}
fn gl_transition(uv: vec2<f32>, progress: f32) -> vec4<f32> {
  let bar = i32(uv.x * f32(BARS));
  let scale = 1.0 + gl_doom_pos(bar) * AMPLITUDE;
  let phase = progress * scale;
  if (phase + uv.y < 1.0) {
    return getFromColor(vec2<f32>(uv.x, uv.y + phase));
  }
  return getToColor(uv);
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
