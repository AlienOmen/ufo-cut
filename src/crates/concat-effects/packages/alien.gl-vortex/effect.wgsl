// SPDX-License-Identifier: MIT
// Ported to WGSL from the gl-transitions library (https://github.com/gl-transitions/gl-transitions),
// transitions/Vortex.glsl
// Author: gre
// License: MIT
// See THIRD_PARTY_NOTICES.md for the licence text.
const GRID: i32 = 24;
const SPIN: f32 = 1.0;
const RANDOMNESS: f32 = 0.2;
const GROUPS: i32 = 24;
const DURATION: f32 = 0.25;
const AA: f32 = 0.0015;
const TAU: f32 = 6.2831853;
fn gl_hash(n: vec2<f32>) -> f32 {
  return fract(sin(dot(n, vec2<f32>(12.9898, 78.233))) * 43758.5453);
}
fn gl_group_of(n: vec2<f32>, h: f32, c: vec2<f32>, max_r: f32) -> i32 {
  let r = min(length((n + vec2<f32>(0.5)) * h - c) / (0.85 * max_r), 1.0);
  let key = clamp(mix(r * sqrt(r), gl_hash(n), clamp(RANDOMNESS, 0.0, 1.0)), 0.0, 0.9999);
  return i32(floor(key * f32(GROUPS)));
}
fn gl_flight(g: i32, progress: f32) -> f32 {
  let delay = f32(g) / f32(GROUPS) * (1.0 - DURATION);
  return clamp((progress - delay) / DURATION, 0.0, 1.0);
}
fn gl_premultiplied(c: vec4<f32>) -> vec4<f32> {
  return vec4<f32>(c.rgb * c.a, c.a);
}
fn gl_transition(uv: vec2<f32>, progress: f32) -> vec4<f32> {
  if (progress <= 0.0) { return getFromColor(uv); }
  if (progress >= 1.0) { return getToColor(uv); }
  let ratio = gl_ratio();
  let center = vec2<f32>(0.5, 0.5);
  let aspect = vec2<f32>(ratio, 1.0);
  let p = uv * aspect;
  let c = center * aspect;
  let max_r = max(max(length(c), length(c - vec2<f32>(ratio, 0.0))), max(length(c - vec2<f32>(0.0, 1.0)), length(c - aspect)));
  let h = 1.0 / f32(max(GRID, 1));
  let reach = 0.7 * max_r;
  let n0 = floor(p / h);
  var col = gl_premultiplied(getToColor(uv));
  if (gl_flight(gl_group_of(n0, h, c, max_r), progress) <= 0.0) { col = gl_premultiplied(getFromColor(uv)); }
  for (var k = 0; k < GROUPS; k = k + 1) {
    let g = GROUPS - 1 - k;
    let s = gl_flight(g, progress);
    if (s <= 0.0 || s >= 1.0) { continue; }
    let e = s * s;
    let angle = SPIN * TAU * e * 2.0 / (1.0 + 6.0 * f32(g) / f32(GROUPS));
    let push = reach * e * s;
    let size = (1.0 + 0.35 * sin(3.1415927 * s)) * (1.0 - s * s * s);
    let ca = cos(angle);
    let sa = sin(angle);
    let rot = mat2x2<f32>(ca, sa, -sa, ca);
    let u = (p - c) * rot;
    let ru = length(u);
    var dir_u = vec2<f32>(0.0);
    if (ru > 0.0) { dir_u = u / ru; }
    let guess = c + dir_u * max(ru - push, 0.0);
    let ng = floor(guess / h);
    for (var j = -1; j <= 1; j = j + 1) {
      for (var i = -1; i <= 1; i = i + 1) {
        let n = ng + vec2<f32>(f32(i), f32(j));
        if (gl_group_of(n, h, c, max_r) != g) { continue; }
        let home = (n + vec2<f32>(0.5)) * h;
        let rel = home - c;
        let rr = length(rel);
        var dir = vec2<f32>(0.0);
        if (rr > 0.0) { dir = rel / rr; }
        let pos = c + rot * (dir * (rr + push));
        let tumble = angle + (gl_hash(n + vec2<f32>(17.0)) - 0.5) * 3.0 * e * sign(SPIN);
        let ct = cos(tumble);
        let st = sin(tumble);
        let d = ((p - pos) * mat2x2<f32>(ct, st, -st, ct)) / max(size, 1e-4);
        let edge = 0.5 * h - max(abs(d.x), abs(d.y));
        let cov = clamp(edge * size / (2.0 * AA) + 0.5, 0.0, 1.0);
        let src = home + d;
        if (cov <= 0.0 || src.x < 0.0 || src.y < 0.0 || src.x > ratio || src.y > 1.0) { continue; }
        let piece = gl_premultiplied(getFromColor(src / aspect)) * cov;
        col = piece + col * (1.0 - piece.a);
      }
    }
  }
  if (col.a > 0.0) { return vec4<f32>(col.rgb / col.a, col.a); }
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
