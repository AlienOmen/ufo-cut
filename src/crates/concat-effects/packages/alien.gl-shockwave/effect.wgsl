// SPDX-License-Identifier: MIT
// Ported to WGSL from the gl-transitions library (https://github.com/gl-transitions/gl-transitions),
// transitions/Shockwave.glsl
// Author: Gilad Levi, github @Giladx
// License: MIT
// See THIRD_PARTY_NOTICES.md for the licence text.
fn gl_tosee_hash(p: vec2<f32>) -> f32 {
  return fract(sin(dot(p, vec2<f32>(127.1, 311.7))) * 43758.5453123);
}
fn gl_transition(uv: vec2<f32>, progress: f32) -> vec4<f32> {
  let tile_size = 0.035;
  let scatter = 0.85;
  let diagonal = 0.9;
  let prism = 0.65;
  let glow = 0.75;
  if (progress <= 0.0) { return getFromColor(uv); }
  if (progress >= 1.0) { return getToColor(uv); }
  let ratio = gl_ratio();
  let size = max(tile_size, 0.008);
  let grid = max(vec2<f32>(1.0), floor(vec2<f32>(ratio, 1.0) / size));
  let cell = floor(uv * grid);
  let cell_uv = fract(uv * grid);
  let center_uv = (cell + vec2<f32>(0.5)) / grid;
  let sweep = mix(center_uv.x, 0.64 * center_uv.x + 0.36 * (1.0 - center_uv.y), clamp(diagonal, 0.0, 1.0));
  let noise = gl_tosee_hash(cell);
  let start = 0.62 * sweep + 0.07 * noise;
  let t = clamp((progress - start) / 0.29, 0.0, 1.0);
  let movement = t * clamp(scatter, 0.0, 1.5);
  let wobble = gl_tosee_hash(cell + vec2<f32>(19.3, 7.7)) * 2.0 - 1.0;
  let drift = vec2<f32>(0.65 * wobble, -0.95) * movement;
  let moving_cell_uv = cell_uv - drift;
  let inside = step(0.012, moving_cell_uv.x) * step(moving_cell_uv.x, 0.988) * step(0.012, moving_cell_uv.y) * step(moving_cell_uv.y, 0.988);
  let tile_shape = mix(1.0, inside, smoothstep(0.0, 0.13, t));
  let old_visibility = 1.0 - smoothstep(0.48, 0.87, t);
  let old_mask = tile_shape * old_visibility;
  let old_uv = clamp(uv - drift / grid, vec2<f32>(0.0), vec2<f32>(1.0));
  var old_color = getFromColor(old_uv);
  let flare = sin(3.14159265 * t);
  let shift = vec2<f32>(0.006, -0.002) * prism * flare;
  let offset_color = getFromColor(clamp(old_uv + shift, vec2<f32>(0.0), vec2<f32>(1.0)));
  old_color.r = offset_color.r;
  var result = mix(getToColor(uv), old_color, old_mask);
  let edge_distance = min(min(moving_cell_uv.x, 1.0 - moving_cell_uv.x), min(moving_cell_uv.y, 1.0 - moving_cell_uv.y));
  let edge = (1.0 - smoothstep(0.0, 0.10, edge_distance)) * inside;
  let cyan = vec3<f32>(0.12, 0.95, 1.0);
  let pink = vec3<f32>(1.0, 0.20, 0.65);
  let rim_color = mix(cyan, pink, gl_tosee_hash(cell + vec2<f32>(5.4, 28.6)));
  result = vec4<f32>(clamp(result.rgb + rim_color * edge * flare * glow * old_mask * 0.60, vec3<f32>(0.0), vec3<f32>(1.0)), result.a);
  return result;
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
