// SPDX-License-Identifier: MIT
// Ported to WGSL from the gl-transitions library (https://github.com/gl-transitions/gl-transitions),
// transitions/Doorway.glsl
// Author: gre
// License: MIT
// See THIRD_PARTY_NOTICES.md for the licence text.
const REFLECTION: f32 = 0.4;
const PERSPECTIVE: f32 = 0.4;
const DEPTH: f32 = 3.0;
fn gl_leaf_hit(x: f32, w: f32, eye: f32, a: f32) -> vec2<f32> {
  let c = cos(a);
  let s = sin(a);
  let t = (eye * c - 0.5 * w * s) / (s * (x - 0.5 * w) + eye * c);
  let hit = vec2<f32>(0.5 * w + t * (x - 0.5 * w), eye * (t - 1.0));
  return vec2<f32>(t, hit.x * c - hit.y * s);
}
fn gl_project(p: vec2<f32>, w: f32, eye: f32) -> f32 {
  return 0.5 * w + (p.x - 0.5 * w) * eye / (eye + p.y);
}
fn gl_ease_in_out(t: f32) -> f32 {
  if (t < 0.5) { return 4.0 * t * t * t; }
  return 1.0 - 4.0 * (1.0 - t) * (1.0 - t) * (1.0 - t);
}
fn gl_room(uv: vec2<f32>, x: f32, edge_l: f32, edge_r: f32, close: f32, dolly: f32) -> vec4<f32> {
  let size = mix(max(DEPTH, 1.0), 1.0, dolly);
  let p = (uv - vec2<f32>(0.5)) * size + vec2<f32>(0.5);
  let soft = 0.004 * (size - 1.0) + 1e-6;
  let m = smoothstep(vec2<f32>(-soft), vec2<f32>(soft), p) * smoothstep(vec2<f32>(-soft), vec2<f32>(soft), vec2<f32>(1.0) - p);
  var c = getToColor(clamp(p, vec2<f32>(0.0), vec2<f32>(1.0))) * (m.x * m.y);
  if (p.y < 0.0) {
    let r = vec2<f32>(p.x, -1.2 * p.y - 0.02);
    let f = REFLECTION * (1.0 - r.y) * m.x * smoothstep(0.0, 0.01, r.y);
    c = c + getToColor(clamp(r, vec2<f32>(0.0), vec2<f32>(1.0))) * f * (1.0 - m.y);
  }
  let wl = 0.03 + 0.12 * close;
  let shadow = (1.0 - smoothstep(0.0, wl, x - edge_l)) + (1.0 - smoothstep(0.0, wl, edge_r - x));
  let light = (1.0 + 0.6 * close * close) * (1.0 - 0.7 * close * min(shadow, 1.0));
  return vec4<f32>(min(c.rgb * light, vec3<f32>(c.a)), c.a);
}
fn gl_transition(uv: vec2<f32>, progress: f32) -> vec4<f32> {
  let w = gl_ratio();
  let x = uv.x * w;
  let eye = max(w, 1.0) * (0.5 + 0.2 / max(PERSPECTIVE, 0.01));
  let swing = progress * progress * (3.0 - 2.0 * progress);
  let a = min(atan(2.0 * eye / w) + 0.1, 0.5 * GL_PI) * swing;
  let dolly = mix(gl_ease_in_out(progress), progress, 0.3);
  let hw = 0.5 * w;
  let front = hw * vec2<f32>(cos(a), -sin(a));
  let thick = 0.02 * w * (1.0 - smoothstep(0.8, 1.0, swing));
  let edge_l = gl_project(front, w, eye);
  let back_l = gl_project(front + thick * vec2<f32>(sin(a), cos(a)), w, eye);
  let close = 1.0 - swing;
  let back = gl_room(uv, x, back_l, w - back_l, close, dolly);
  let left = x < hw;
  var lx = w - x;
  if (left) { lx = x; }
  let hit = gl_leaf_hit(lx, w, eye, a);
  let s = clamp(hit.y, 0.0, hw);
  let v = 0.5 + (uv.y - 0.5) * hit.x;
  var leaf_u = 1.0 - s / w;
  if (left) { leaf_u = s / w; }
  var leaf = getFromColor(clamp(vec2<f32>(leaf_u, v), vec2<f32>(0.0), vec2<f32>(1.0)));
  leaf = vec4<f32>(leaf.rgb * (1.0 - 0.5 * (1.0 - cos(a)) - 0.25 * sin(a) * (s / hw)), leaf.a);
  var side_u = 0.501;
  if (left) { side_u = 0.499; }
  var side = getFromColor(clamp(vec2<f32>(side_u, v), vec2<f32>(0.0), vec2<f32>(1.0)));
  side = vec4<f32>(min(side.rgb * 1.5, vec3<f32>(side.a)), side.a);
  let aa = 0.003 * min(1.0, 20.0 * min(a, close)) + 1e-6;
  let c = mix(back, side, smoothstep(-aa, aa, back_l - lx));
  return mix(c, leaf, smoothstep(-aa, aa, edge_l - lx));
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
