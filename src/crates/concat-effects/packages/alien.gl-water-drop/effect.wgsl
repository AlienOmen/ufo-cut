// SPDX-License-Identifier: MIT
// Ported to WGSL from the gl-transitions library (https://github.com/gl-transitions/gl-transitions),
// transitions/WaterDrop.glsl
// Author: Paweł Płóciennik
// License: MIT
// See THIRD_PARTY_NOTICES.md for the licence text.
const AMPLITUDE: f32 = 30.0;
const SPEED: f32 = 30.0;
const REFRACTION: f32 = 0.015;
const HIGHLIGHT: f32 = 0.5;
fn gl_mirror(v: vec2<f32>) -> vec2<f32> {
  return vec2<f32>(1.0) - abs(vec2<f32>(1.0) - gl_mod2(v, 2.0));
}
fn gl_wave(s: f32, k: f32, lambda: f32, t: f32) -> f32 {
  let env = smoothstep(-0.3 * lambda, 0.6 * lambda, s) * exp(-s / (3.5 * lambda));
  return env * sin(k * s + SPEED * t);
}
fn gl_transition(uv: vec2<f32>, progress: f32) -> vec4<f32> {
  if (progress <= 0.0) { return getFromColor(uv); }
  if (progress >= 1.0) { return getToColor(uv); }
  let ratio = gl_ratio();
  let center = vec2<f32>(0.5, 0.5);
  let k = 2.0 * max(AMPLITUDE, 1.0);
  let lambda = 6.2831853 / k;
  let q = (uv - center) * vec2<f32>(ratio, 1.0);
  let r = length(q);
  var dir = vec2<f32>(0.0);
  if (r > 0.0) { dir = q / r; }
  let far = max(center, vec2<f32>(1.0) - center) * vec2<f32>(ratio, 1.0);
  let R = length(far);
  let edge = 0.35 * lambda;
  let soft = 0.1 * lambda;
  let t = progress;
  let e = mix(t, 1.0 - (1.0 - t) * (1.0 - t), 0.4);
  let front = mix(-0.3 * lambda, R + edge + soft + 0.01, e);
  let s = front - r;
  var calm = 1.0 - smoothstep(0.55, 1.0, t);
  calm = calm * calm;
  let d = 0.02 * lambda;
  let r0 = max(r - d, 0.0);
  let slope = calm * (gl_wave(front - r - d, k, lambda, t) - gl_wave(front - r0, k, lambda, t)) / ((r + d - r0) * k);
  let offset = dir * slope * REFRACTION / vec2<f32>(ratio, 1.0);
  let ca = getFromColor(gl_mirror(uv - offset));
  let cb = getToColor(gl_mirror(uv - offset));
  let m = smoothstep(edge - soft, edge + soft, s);
  var c = mix(vec4<f32>(ca.rgb * ca.a, ca.a), vec4<f32>(cb.rgb * cb.a, cb.a), m);
  let h = calm * gl_wave(s, k, lambda, t);
  let lit = -dot(dir, normalize(vec2<f32>(-0.6, 0.8))) * slope;
  let glint = HIGHLIGHT * (0.2 * h + 0.6 * smoothstep(0.2, 0.9, lit) - 0.3 * smoothstep(0.2, 0.9, -lit));
  c = vec4<f32>(clamp(c.rgb + vec3<f32>(glint * c.a), vec3<f32>(0.0), vec3<f32>(c.a)), c.a);
  if (c.a > 0.0) { return vec4<f32>(c.rgb / c.a, c.a); }
  return vec4<f32>(0.0);
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
