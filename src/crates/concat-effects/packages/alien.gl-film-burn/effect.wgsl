// SPDX-License-Identifier: MIT
// Ported to WGSL from the gl-transitions library (https://github.com/gl-transitions/gl-transitions),
// transitions/FilmBurn.glsl
// Author: Anastasia Dunbar
// License: MIT
// See THIRD_PARTY_NOTICES.md for the licence text.
const SEED: f32 = 2.31;
fn gl_sigmoid(x: f32, a: f32) -> f32 {
  var b = pow(x * 2.0, a) / 2.0;
  if (x > 0.5) { b = 1.0 - pow(2.0 - (x * 2.0), a) / 2.0; }
  return b;
}
fn gl_rand1(co: f32) -> f32 {
  return fract(sin((co * 24.9898) + SEED) * 43758.5453);
}
fn gl_apow(a: f32, b: f32) -> f32 { return pow(abs(a), b) * sign(b); }
fn gl_pow3(a: vec3<f32>, b: vec3<f32>) -> vec3<f32> {
  return vec3<f32>(gl_apow(a.r, b.r), gl_apow(a.g, b.g), gl_apow(a.b, b.b));
}
fn gl_smooth_mix(a: f32, b: f32, c: f32) -> f32 { return mix(a, b, gl_sigmoid(c, 2.0)); }
fn gl_random2(co_in: vec2<f32>, shft: f32) -> f32 {
  let co = co_in + vec2<f32>(10.0);
  return gl_smooth_mix(
    fract(sin(dot(co, vec2<f32>(12.9898 + (floor(shft) * 0.5), 78.233 + SEED))) * 43758.5453),
    fract(sin(dot(co, vec2<f32>(12.9898 + (floor(shft + 1.0) * 0.5), 78.233 + SEED))) * 43758.5453),
    fract(shft));
}
fn gl_smooth_random(co: vec2<f32>, shft: f32) -> f32 {
  return gl_smooth_mix(
    gl_smooth_mix(gl_random2(floor(co), shft), gl_random2(floor(co + vec2<f32>(1.0, 0.0)), shft), fract(co.x)),
    gl_smooth_mix(gl_random2(floor(co + vec2<f32>(0.0, 1.0)), shft), gl_random2(floor(co + vec2<f32>(1.0, 1.0)), shft), fract(co.x)),
    fract(co.y));
}
fn gl_blended_color(p: vec2<f32>, progress: f32) -> vec4<f32> {
  return mix(getFromColor(p), getToColor(p), gl_sigmoid(progress, 10.0));
}
fn gl_clamps(x: f32) -> f32 { return clamp(x, 0.0, 1.0); }
fn gl_transition(p_in: vec2<f32>, progress: f32) -> vec4<f32> {
  var p = p_in;
  var f = vec3<f32>(0.0);
  for (var i = 0; i < 13; i = i + 1) {
    let fi = f32(i);
    f = f + vec3<f32>(sin(((p.x * gl_rand1(fi) * 6.0) + (progress * 8.0)) + gl_rand1(fi + 1.43)) * sin(((p.y * gl_rand1(fi + 4.4) * 6.0) + (progress * 6.0)) + gl_rand1(fi + 2.4)));
    f = f + vec3<f32>(1.0 - gl_clamps(length(p - vec2<f32>(gl_smooth_random(vec2<f32>(progress * 1.3), fi + 1.0), gl_smooth_random(vec2<f32>(progress * 0.5), fi + 6.25))) * mix(20.0, 70.0, gl_rand1(fi))));
  }
  f = f + vec3<f32>(4.0);
  f = f / 11.0;
  f = gl_pow3(f * vec3<f32>(1.0, 0.7, 0.6), vec3<f32>(1.0, 2.0 - sin(progress * GL_PI), 1.3));
  f = f * sin(progress * GL_PI);
  p = p - vec2<f32>(0.5);
  p = p * (1.0 + (gl_smooth_random(vec2<f32>(progress * 5.0), 6.3) * sin(progress * GL_PI) * 0.05));
  p = p + vec2<f32>(0.5);
  var blurred_image = vec4<f32>(0.0);
  let bluramount = sin(progress * GL_PI) * 0.03;
  let repeats = 50;
  for (var i = 0; i < 50; i = i + 1) {
    let fi = f32(i);
    let ang = (fi / 50.0) * 6.283185307179586;
    let q = vec2<f32>(cos(ang), sin(ang)) * (gl_rand(vec2<f32>(fi, p.x + p.y)) + bluramount);
    let uv2 = p + (q * bluramount);
    blurred_image = blurred_image + gl_blended_color(uv2, progress);
  }
  blurred_image = blurred_image / 50.0;
  return blurred_image + vec4<f32>(f, 0.0);
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
