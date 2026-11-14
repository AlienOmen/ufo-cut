// SPDX-License-Identifier: MIT
// Ported to WGSL from the gl-transitions library (https://github.com/gl-transitions/gl-transitions),
// transitions/CrossZoom.glsl
// Author: rectalogic
// License: MIT
// See THIRD_PARTY_NOTICES.md for the licence text.
const STRENGTH: f32 = 0.4;
fn gl_expo_ease(t: f32) -> f32 {
  if (t <= 0.0) { return 0.0; }
  if (t >= 1.0) { return 1.0; }
  let k = t * 2.0;
  if (k < 1.0) { return 0.5 * pow(2.0, 10.0 * (k - 1.0)); }
  return 0.5 * (-pow(2.0, -10.0 * (k - 1.0)) + 2.0);
}
fn gl_cross_fade(uv: vec2<f32>, dissolve: f32) -> vec4<f32> {
  return mix(getFromColor(uv), getToColor(uv), dissolve);
}
fn gl_transition(uv: vec2<f32>, progress: f32) -> vec4<f32> {
  let center = vec2<f32>(0.25 + 0.5 * progress, 0.5);
  let dissolve = gl_expo_ease(progress);
  // Mirrored sinusoidal loop, 0 to strength to 0.
  let strength = -STRENGTH / 2.0 * (cos(GL_PI * progress / 0.5) - 1.0);
  var color = vec4<f32>(0.0);
  var total = 0.0;
  let to_center = center - uv;
  let offset = gl_rand(uv);
  for (var i = 0; i <= 40; i = i + 1) {
    let percent = (f32(i) + offset) / 40.0;
    let weight = 4.0 * (percent - percent * percent);
    color += gl_cross_fade(uv + to_center * percent * strength, dissolve) * weight;
    total += weight;
  }
  return color / total;
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
