# Safe zones for vertical video (1080 x 1920)

Source of record (2026-11-14): the owner's "Free Social Media Safe Zone Templates" pack, eight 1080 x 1920 outline images, measured pixel by pixel. Two more references (a Reformat.video guide for YouTube Shorts and the first text research for WhatsApp Status) fill the two platforms the pack does not cover. **No platform publishes one official pixel table**; these are third-party templates, so always check a real-phone preview before publishing.

All values are pixels of a 1080 x 1920 frame (the effect scales them as fractions to any size). "Safe box" is where text, faces and logos may go.

| Effect value | Preset | Safe box (x from-to, y from-to) | Margins top / bottom / left / right | Extra unsafe area |
|---|---|---|---|---|
| 1 | Instagram Reel | x 66-1013, y 270-1245 | 270 / 675 / 66 / 67 | right of x 855 below y 1152 (the action buttons) |
| 2 | Instagram Story | x 66-1013, y 270-1533 | 270 / 387 / 66 / 67 | none |
| 3 | Facebook Reel | x 66-1013, y 270-1245 | 270 / 675 / 66 / 67 | right of x 855 below y 1152 |
| 4 | Facebook Story | x 66-1013, y 270-1533 | 270 / 387 / 66 / 67 | none |
| 5 | TikTok Video | x 122-957, y 254-1437 | 254 / 483 / 122 / 123 | right of x 837 below y 362 (the button column; the top strip y 254-362 may use x up to 957) |
| 6 | TikTok Ad | x 122-957, y 254-1277 | 254 / 643 / 122 / 123 | right of x 837 below y 362 |
| 7 | Amazon vertical video ad | x 56-1022, y 194-1532 | 194 / 388 / 56 / 58 | none |
| 8 | LinkedIn video ad | x 114-867, y 262-1357 | 262 / 563 / 114 / 213 | none |
| 9 | YouTube Shorts | x 60-960, y 380-1540 | 380 / 380 / 60 / 120 | none (Reformat.video guide; not in the pack) |
| 10 | WhatsApp Status | x 65-1015, y 250-1520 | 250 / 400 / 65 / 65 | none (low confidence; no template) |
| 0 | **Alien Safe Zone** | x 122-957, y 270-1245, narrowed to x 837 below y 362 | 270 / 675 / 122 / 123 | right of x 837 below y 362 |

**The Alien Safe Zone** is what is safe on Instagram, Facebook and TikTok at the same time: the intersection of presets 1 to 6. In practice keep text, faces and logos inside **x 122 to 837 and y 270 to 1245** (715 x 975 px); the top strip (y 270 to 362) may use x up to 957. It is strict on purpose: the bottom 675 px is where Reels puts its caption and buttons. For a single platform, pick that preset instead.

**What the course checker uses:** the lesson check (`concat-project/src/preflight.rs`) warns when a caption leaves this zone vertically, with the same top (14.06%) and bottom (35.16%) margins. TODO(owner): the lessons (E.19, E.20, C.03) say "inside the Safe Zone" without numbers; quote these in the lessons, or choose looser ones.

## How the numbers were taken
For each template the outline's line positions were read from the image (line centres), e.g. Instagram Reel: top 270, bottom edge 1245 on the left part and 1152 on the right part, left 66, right 1013, notch at x 855. Left/right margins are 1080 minus the right edge; top/bottom 1920 minus the bottom edge. The templates were drawn at 1080 x 1920, so no scaling was applied.

## The effect

`alien.safe-zones` is an effect layer: add it above the picture, choose a preset in the Platform list, set the opacity. It shades the unsafe areas with void black and a faint acid hatch, and draws thin acid lines on the edge of the safe box. **It is a guide only: hide or remove the layer before export, or it is burned into the video.**

## Still unverified

- The shader and the eleven-entry Platform list were checked by structure and in a browser GPU test, never loaded by UFO Cut itself.
- Shorts and WhatsApp Status are not from the pack; platforms change their interface often, so treat everything here as a snapshot of November 2026.
- The earlier reference images (Reels top 220 / bottom 420, TikTok 160 / 480, Meta ads 270 / 670 / 65) disagree with the pack on several margins, which is normal for unofficial guides; the pack is used because the owner chose it.
