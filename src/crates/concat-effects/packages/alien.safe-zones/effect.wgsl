// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 A.L.I.E.N. (Ahmed Albadry)
struct Params { platform: f32, opacity: f32 }

// GUIDE ONLY: hide or remove this layer before export.
// Margins as fractions: (top, bottom, left, right) of the frame, see docs/SAFE_ZONES.md.
const ACID: vec3<f32> = vec3<f32>(0.578, 1.0, 0.0);
const VOID: vec3<f32> = vec3<f32>(0.003, 0.0033, 0.003);

// Fractions of a 1080x1920 frame, measured from the owner's "Free Social Media Safe Zone
// Templates" pack (docs/SAFE_ZONES.md). margins() is (top, bottom, left, right); column() is the
// corner that is also unsafe: x beyond it and y below it (x fraction, y fraction).
// 0 Alien Safe Zone: what is safe on Instagram, Facebook and TikTok at once
// 1 Instagram Reel   2 Instagram Story   3 Facebook Reel   4 Facebook Story
// 5 TikTok Video     6 TikTok Ad         7 Amazon vertical video ad   8 LinkedIn video ad
// 9 YouTube Shorts (Reformat guide, no template in the pack)   10 WhatsApp Status (low confidence)
fn margins(p: i32) -> vec4<f32> {
    if (p == 1 || p == 3) { return vec4<f32>(0.1406, 0.3516, 0.0611, 0.0620); }
    if (p == 2 || p == 4) { return vec4<f32>(0.1406, 0.2016, 0.0611, 0.0620); }
    if (p == 5) { return vec4<f32>(0.1323, 0.2516, 0.1130, 0.1139); }
    if (p == 6) { return vec4<f32>(0.1323, 0.3349, 0.1130, 0.1139); }
    if (p == 7) { return vec4<f32>(0.1010, 0.2021, 0.0519, 0.0537); }
    if (p == 8) { return vec4<f32>(0.1365, 0.2932, 0.1056, 0.1972); }
    if (p == 9) { return vec4<f32>(0.1979, 0.1979, 0.0556, 0.1111); }
    if (p == 10) { return vec4<f32>(0.1300, 0.2100, 0.0600, 0.0600); }
    return vec4<f32>(0.1406, 0.3516, 0.1130, 0.1139);   // 0: Alien Safe Zone
}

// Instagram and Facebook Reels: the action buttons make the right part end at y = 1152 (x from 855).
// TikTok: its button column makes everything right of x = 837 unsafe below y = 362.
// The Alien Safe Zone takes TikTok's (it covers the Reels one). Others return a point outside the frame.
fn column(p: i32) -> vec2<f32> {
    if (p == 1 || p == 3) { return vec2<f32>(0.7917, 0.6000); }
    if (p == 5 || p == 6 || p == 0) { return vec2<f32>(0.7750, 0.1885); }
    return vec2<f32>(2.0, 2.0);
}

fn effect(uv: vec2<f32>) -> vec4<f32> {
    let c = sample(uv);
    let pi = i32(round(clamp(params.platform, 0.0, 10.0)));
    let m = margins(pi);
    let col = column(pi);
    // The host mixes this layer over the picture by "opacity" (the intensity key), so k stays 1.
    let k = 1.0 + 0.0 * params.opacity;
    let px = uv * frame.size;
    let size = frame.size;
    let l = m.z * size.x;
    let r = size.x - m.w * size.x;
    let t = m.x * size.y;
    let b = size.y - m.y * size.y;
    let outside = (uv.x < m.z) || (uv.x > 1.0 - m.w) || (uv.y < m.x) || (uv.y > 1.0 - m.y)
        || (uv.x > col.x && uv.y > col.y);
    var rgb = c.rgb;
    if (outside) {
        rgb = mix(rgb, VOID, 0.75 * k);
        // faint diagonal hatch so the unsafe area reads at a glance
        let hatch = step(0.5, fract((px.x + px.y) / (0.012 * size.x)));
        rgb = mix(rgb, ACID, 0.06 * hatch * k);
    }
    // Thin acid line on the edge of the safe rectangle.
    let w = max(1.0, 0.002 * size.x);
    let inx = px.x > l - w && px.x < r + w;
    let iny = px.y > t - w && px.y < b + w;
    let on_v = (abs(px.x - l) < w || abs(px.x - r) < w) && iny;
    let on_h = (abs(px.y - t) < w || abs(px.y - b) < w) && inx;
    if (on_v || on_h) {
        rgb = mix(rgb, ACID, clamp(0.35 + 0.65 * k, 0.0, 1.0));
    }
    return vec4<f32>(rgb, c.a);
}
