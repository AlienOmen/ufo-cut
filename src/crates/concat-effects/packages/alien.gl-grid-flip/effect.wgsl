// SPDX-License-Identifier: MIT
// Ported to WGSL from the gl-transitions library (https://github.com/gl-transitions/gl-transitions),
// transitions/GridFlip.glsl
// Author: TimDonselaar
// License: MIT
// See THIRD_PARTY_NOTICES.md for the licence text.
const GRID_SIZE: vec2<f32> = vec2<f32>(4.0, 4.0);
const PAUSE: f32 = 0.1;
const DIVIDER_WIDTH: f32 = 0.05;
const BGCOLOR: vec4<f32> = vec4<f32>(0.0, 0.0, 0.0, 1.0);
const RANDOMNESS: f32 = 0.1;
const EDGE: f32 = 0.0012;
const EYE: f32 = 2.2;
const STAGGER: f32 = 0.55;
const LIGHT: vec2<f32> = vec2<f32>(-0.45, 0.3);
fn gl_hash12(v: vec2<f32>) -> f32 {
  var v3 = fract(vec3<f32>(v.x, v.y, v.x) * 0.1031);
  v3 = v3 + vec3<f32>(dot(v3, v3.yzx + vec3<f32>(33.33)));
  return fract((v3.x + v3.y) * v3.z);
}
fn gl_ease_in_out(t: f32) -> f32 {
  return mix(t, t * t * (3.0 - 2.0 * t), 0.7);
}
fn gl_pause_part() -> f32 {
  return clamp(PAUSE, 0.05, 0.4);
}
fn gl_flip_time(cell: vec2<f32>, grid: vec2<f32>, qf: f32, stagger: f32) -> f32 {
  let wave = (cell.x + (grid.y - 1.0 - cell.y)) / max(grid.x + grid.y - 2.0, 1.0);
  let order = mix(wave, gl_hash12(cell + vec2<f32>(7.31)), clamp(RANDOMNESS, 0.0, 1.0));
  return gl_ease_in_out(clamp((qf - order * stagger) / (1.0 - stagger), 0.0, 1.0));
}
fn gl_gap_amount(progress: f32) -> f32 {
  let pp = gl_pause_part();
  return smoothstep(0.0, pp, progress) * (1.0 - smoothstep(1.0 - pp, 1.0, progress));
}
fn gl_aa(d: f32, e: f32) -> f32 {
  return clamp(d / max(2.0 * e, 1e-6) + 0.5, 0.0, 1.0);
}
fn gl_box_shadow(p: vec2<f32>, half_size: vec2<f32>, blur: f32) -> f32 {
  let d = abs(p) - half_size;
  let dist = length(max(d, vec2<f32>(0.0))) + min(max(d.x, d.y), 0.0);
  return 1.0 - smoothstep(-blur, blur, dist);
}
fn gl_transition(uv: vec2<f32>, progress: f32) -> vec4<f32> {
  let ratio = gl_ratio();
  let grid = max(GRID_SIZE, vec2<f32>(1.0));
  let fr = vec2<f32>(ratio, 1.0);
  let tile_size = fr / grid;
  let cell = clamp(floor(uv * grid), vec2<f32>(0.0), grid - vec2<f32>(1.0));
  let pos = uv * fr;
  let gap_open = gl_gap_amount(progress);
  let min_side = min(tile_size.x, tile_size.y);
  let gap = clamp(DIVIDER_WIDTH, 0.0, 0.4) * min_side * gap_open;
  let half_tile = 0.5 * tile_size - vec2<f32>(gap);
  let eye = EYE * max(tile_size.x, tile_size.y);
  let pp = 0.6 * gl_pause_part();
  let qf = clamp((progress - pp) / (1.0 - 2.0 * pp), 0.0, 1.0);
  var stagger = 0.0;
  if (grid.x * grid.y > 1.5) { stagger = STAGGER; }

  var c1 = vec4<f32>(0.0);
  var c2 = vec4<f32>(0.0);
  var m1 = 0.0;
  var m2 = 0.0;
  var z1 = 1e9;
  var z2 = 1e9;
  var rest1 = true;
  var rest2 = true;
  var shadow_bg = 0.0;
  var shadow_flip = 0.0;

  for (var j = -1; j <= 1; j = j + 1) {
    for (var i = -1; i <= 1; i = i + 1) {
      let nc = cell + vec2<f32>(f32(i), f32(j));
      if (nc.x < 0.0 || nc.y < 0.0 || nc.x > grid.x - 1.0 || nc.y > grid.y - 1.0) { continue; }
      let t = gl_flip_time(nc, grid, qf, stagger);
      let a = GL_PI * t;
      var ca = cos(a);
      var sa = sin(a);
      let rest = t <= 0.0 || t >= 1.0;
      if (rest) {
        ca = 1.0;
        if (t > 0.0) { ca = -1.0; }
        sa = 0.0;
      }
      let center = (nc + vec2<f32>(0.5)) * tile_size;
      let q = pos - center;

      let lift = 0.05 * gap_open * min_side + 0.35 * half_tile.x * sa;
      let sh_offset = -LIGHT * lift;
      let sh_half = vec2<f32>(half_tile.x * abs(ca) + 0.5 * lift, half_tile.y);
      let sh = gl_box_shadow(q - sh_offset, sh_half, 0.01 * min_side + 0.5 * lift);
      shadow_bg = 1.0 - (1.0 - shadow_bg) * (1.0 - 0.45 * gap_open * sh);
      shadow_flip = max(shadow_flip, 0.45 * sa * sa * sh);

      let den = eye * ca - q.x * sa;
      if (abs(den) < 1e-6) { continue; }
      let s = eye * ca / den;
      if (s <= 0.0) { continue; }
      let hit = vec3<f32>(s * q.x, s * q.y, eye * (1.0 - s));
      let u = hit.x * ca - hit.z * sa;
      let xa = half_tile.x * ca * eye / (eye + half_tile.x * sa);
      let xb = -half_tile.x * ca * eye / (eye - half_tile.x * sa);
      let dx = min(q.x - min(xa, xb), max(xa, xb) - q.x);
      let dy = (half_tile.y - abs(hit.y)) / s;
      let e = max(min(EDGE, gap), EDGE * min(8.0 * sa, 1.0));
      let m = gl_aa(min(dx, dy), e);
      if (m <= 0.0) { continue; }

      let front = ca > 0.0;
      var src_u = -u;
      if (front) { src_u = u; }
      let src = (center + vec2<f32>(src_u, hit.y)) / fr;
      var col = getToColor(src);
      if (front) { col = getFromColor(src); }
      if (!rest) {
        var nx = -sa;
        if (front) { nx = sa; }
        let lit = abs(ca) + nx * LIGHT.x;
        var rgb = col.rgb * (mix(0.35, 1.0, clamp(lit, 0.0, 1.0)) + 0.25 * max(lit - 1.0, 0.0));
        rgb = rgb + vec3<f32>(0.12 * sa * (1.0 - smoothstep(0.0, 3.0 * EDGE, min(dx, dy))));
        col = vec4<f32>(rgb, col.a);
      }

      if (s < z1) {
        c2 = c1; m2 = m1; z2 = z1; rest2 = rest1;
        c1 = col; m1 = m; z1 = s; rest1 = rest;
      } else if (s < z2) {
        c2 = col; m2 = m; z2 = s; rest2 = rest;
      }
    }
  }

  var color = BGCOLOR;
  color = vec4<f32>(color.rgb * (1.0 - clamp(max(shadow_bg, shadow_flip), 0.0, 1.0)), color.a);
  if (rest2) { c2 = vec4<f32>(c2.rgb * (1.0 - shadow_flip), c2.a); }
  if (rest1) { c1 = vec4<f32>(c1.rgb * (1.0 - shadow_flip), c1.a); }
  color = mix(color, c2, m2);
  color = mix(color, c1, m1);
  return color;
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
