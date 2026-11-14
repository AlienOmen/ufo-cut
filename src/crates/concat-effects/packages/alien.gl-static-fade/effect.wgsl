// SPDX-License-Identifier: MIT
// Ported to WGSL from the gl-transitions library (https://github.com/gl-transitions/gl-transitions),
// transitions/StaticFade.glsl
// Author: Ben Lucas
// License: MIT
// See THIRD_PARTY_NOTICES.md for the licence text.
fn gl_rnd(st: vec2<f32>) -> f32 {
  return fract(sin(dot(st, vec2<f32>(10.5302340293, 70.23492931))) * 12345.5453123);
}
fn gl_static_noise(st: vec2<f32>, offset: f32, luminosity: f32, alpha: f32) -> vec4<f32> {
  let r = luminosity * gl_rnd(st * vec2<f32>(offset * 2.0, offset * 3.0));
  let g = luminosity * gl_rnd(st * vec2<f32>(offset * 3.0, offset * 5.0));
  let b = luminosity * gl_rnd(st * vec2<f32>(offset * 5.0, offset * 7.0));
  return vec4<f32>(r, g, b, alpha);
}
fn gl_static_intensity(t: f32) -> f32 {
  let transition_progress = abs(2.0 * (t - 0.5));
  let transformed_threshold = 1.2 * (1.0 - transition_progress) - 0.1;
  return min(1.0, transformed_threshold);
}
fn gl_transition(uv: vec2<f32>, progress: f32) -> vec4<f32> {
  let n_noise_pixels = 200.0;
  let static_luminosity = 0.8;
  let base_mix = step(0.5, progress);
  let transition_mix = mix(getFromColor(uv), getToColor(uv), base_mix);
  let uv_static = floor(uv * n_noise_pixels) / n_noise_pixels;
  let static_color = gl_static_noise(uv_static, progress, static_luminosity, transition_mix.a);
  let static_thresh = gl_static_intensity(progress);
  let static_mix = step(gl_rnd(uv_static), static_thresh);
  return mix(transition_mix, static_color, static_mix);
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
