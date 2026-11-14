// SPDX-License-Identifier: MIT
// Ported to WGSL from the gl-transitions library (https://github.com/gl-transitions/gl-transitions),
// transitions/Swirl.glsl
// Author: Sergey Kosarevsky
// License: MIT
// See THIRD_PARTY_NOTICES.md for the licence text.
const STRENGTH: f32 = 1.0;
const RADIUS: f32 = 0.9;
const ZOOM: f32 = 0.12;
const BLUR: f32 = 0.05;
fn gl_mirrored(v: vec2<f32>) -> vec2<f32> {
  let m = gl_mod2(v, 2.0);
  return mix(m, vec2<f32>(2.0) - m, step(vec2<f32>(1.0), m));
}
fn gl_transition(uv: vec2<f32>, progress: f32) -> vec4<f32> {
  let ratio = gl_ratio();
  let center = vec2<f32>(0.5, 0.5);
  let TAU = 6.283185307179586;
  let s = sin(GL_PI * progress);
  let amount = s * s;
  let speed = GL_PI * sin(TAU * progress);
  let reveal = smoothstep(0.3, 0.7, progress);
  let aspect = vec2<f32>(ratio, 1.0);
  var d = (uv - center) * aspect;
  let x = clamp(length(d) / max(RADIUS, 1e-3), 0.0, 1.0);
  let falloff = (1.0 - x) * (1.0 - x);
  let twist = STRENGTH * TAU * falloff;
  let angle = twist * amount;
  let spread = clamp(twist * speed * BLUR, -1.2, 1.2);
  d = d * (1.0 - ZOOM * amount * falloff);
  var col = vec4<f32>(0.0);
  var total = 0.0;
  for (var i = 0; i < 12; i = i + 1) {
    let k = f32(i) / 11.0 - 0.5;
    let w = 1.0 - abs(k);
    let a = angle + spread * k;
    let c = cos(a);
    let sn = sin(a);
    let q = gl_mirrored(center + vec2<f32>(c * d.x - sn * d.y, sn * d.x + c * d.y) / aspect);
    col += mix(getFromColor(q), getToColor(q), reveal) * w;
    total += w;
  }
  return col / total;
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
