# UFO Cut is a fork of Concat

UFO Cut is a modified version of [Concat](https://github.com/jub0t/Concat),
the free, open-source video editor by Jareer Abdullah and its contributors.
It is published by A.L.I.E.N. (Ahmed Albadry) and is not made, endorsed or
supported by the Concat project. Please report UFO Cut problems to this
repository, not to Concat.

## Licence

UFO Cut is licensed under the GNU Affero General Public License, version 3
or later, the same licence as Concat. See [`LICENSE`](LICENSE). Every copy
of UFO Cut, paid or free, comes with the right to get, change and share its
complete source code under that licence. The source for each release is in
this repository at the tag of the same version.

Concat's copyright notices and its third-party notices
([`THIRD_PARTY_NOTICES.md`](THIRD_PARTY_NOTICES.md)) are kept as they are.

## What is changed

Changes made in this fork, as AGPL section 5(a) asks, starting 2026-11-10:

- The product is named UFO Cut in the window, the messages, every
  translation and the Windows installer.
- The app keeps its settings and models in its own folder
  (`app.ufocut.editor`), apart from Concat's.
- The updater reads releases from this repository, not from Concat's.
- The GitHub and release-notes links in the app point to this repository.
- The icon and in-app logo are UFO Cut's own; Concat's logo is not used.
- The dark theme uses A.L.I.E.N.'s colours, and the start screen credits
  A.L.I.E.N. beside the version.
- AI models and the sound shelf no longer read from the Concat project's
  repositories. Models are fetched from their original publishers; the
  sound shelf reads from UFO Cut's own asset repository.
- The build recipe makes the Windows x86_64 bundle only, with files named
  `UFO-Cut-*`; the phone builds are not part of a release; the Windows
  installers carry their own identity, so they never replace a Concat
  install.
- The test and Nix workflows run only when started by hand.
- Five A.L.I.E.N. title presets are added (`text-presets/alien.*`).
- The first launch asks which language to use, once (2026-11-10).
- The start screen's cards go to A.L.I.E.N.'s pages; the GitHub and
  Discord cards are now the studio's and the Alien Designer course's.
- The Audio page has a "UFO Cut" shelf: the app's sounds and music, by
  category, read from UFO Cut's own asset repository.
- The Media page can import a video from a web address
  (`src/link_import.rs`). It runs yt-dlp, a separate program that is not
  part of UFO Cut and is fetched from its own releases on first use.
- The Windows setup always asks for the language first, and the app opens
  in that language on its first run (`src/setup_language.rs`).
- Import from link offers a quality choice, from 360p to the best a site
  has; larger pictures are joined to their sound by an ffmpeg program
  fetched once from the ffmpeg-static project.
- Settings › Automation has an "AI assistants" section that copies the
  line or the settings an assistant needs, and `CONNECT-AI.md` explains
  the steps in English, Spanish and Arabic.
- Each launch opens on a short welcome: the clay UFO hovering, drawn by the
  window (`ui/app.slint`).
- The Windows setup puts a shortcut on the desktop by default, and beside
  it one that opens `CONNECT-AI.html`, the guide as a page.
- The program's file is `ufocut.exe`; a project's file is `ufocut.json` (a
  `concat.json` is still read and renamed); the assistant tool is
  `ufocut_version`. The Media page has a downloader card for links.
- The sound shelf also lists sound packs placed on the machine
  (`<data>/sound-packs/<pack>/manifest.json`), and has more categories.
- The Animate panel has sixteen more moves and ten more loops, the ALIEN
  set (`concat-core/src/motion.rs`).
- The shelf accepts sounds marked Free beside the CC0 ones.
- Ten ALIEN transitions (`concat-effects/packages/alien.*`).
- Twelve A.L.I.E.N. colour looks (`src/crates/concat/luts/`) are added to
  the user's LUTs on the first run (`src/alien_looks.rs`).
- Eighteen ALIEN templates (`src/crates/concat/templates/`) are added to
  the template library on the first run (`src/alien_templates.rs`).
- `--software` (the "safe graphics" shortcut) draws in software for that
  launch only, instead of for every launch after it.
- A template gallery on the Templates page: a click asks for the clips and
  opens the project made of the template (`studio/speech.rs`,
  `alien_templates.rs`, `ui/workspace/media-pane.slint`).
- The link downloader takes several links at once, reports progress and
  saves to a folder of the person's choice (`link_import.rs`).
- A clip dragged from the library onto the edge of a lane, between two
  lanes or below the last one makes a new lane there (`InsertTrack` in
  `concat-project`, `studio/placing.rs`).
- The launch screen's links go to the UFO Cut guides in the interface's
  language (`panes/start.rs`).

- **Cut silences** (written, never compiled): right-click a clip with sound
  and choose "Cut silences" to take the quiet gaps out of it and out of the
  rest of the selection (a talk's picture and sound), as one undo step with
  the lanes closed up. The waveform analysis is `concat-media/src/analysis.rs`
  (silences, where sound is present, beats and tempo, all from the peaks the
  timeline already keeps); the cut is a new command, `Command::CutRanges`
  (`concat-project/src/commands/clips.rs`), built from the existing split and
  remove so it cannot disagree with them; `concat-project/src/autocut.rs`
  maps the file's seconds onto the timeline; `concat-host/src/autocut.rs`
  reads the clip. The same is available to assistants as `clip_analyze_sound`,
  `clips_cut_silence` and `clips_cut_ranges` (`concat-api`, `concat-mcp`,
  `docs/MCP.md`). Strings: `studio.cutSilences`, `studio.noSilencesFound`,
  `studio.silencesCut` in the English, Spanish and Arabic locale files (the
  rest show the English).

## UFO Cut 2.0

The release named "version 2". The workspace version is 2.0.0 (`src/Cargo.toml`
and the 17 `concat*` entries of `src/Cargo.lock`); the installer, start screen,
About and updater read that one number. Written without a compiler: the first
CI build is also the first compile of everything below.

- **Safe against a bad package:** a built-in effect package that fails to load
  is left out and named (`Catalogue::compiled_in_checked`, test
  `every_built_in_package_loads`) instead of panicking the app at start, since
  this release adds 67 packages.
- **40 GL Transitions** (`concat-effects/packages/alien.gl-*`): ported from
  gl-transitions, MIT-licensed files only, headers kept, listed in
  `THIRD_PARTY_NOTICES.md`. Fixed defaults, no knobs.
- **6 fades** (`alien.fade-*`: black, white, acid, additive, grain, soft) and
  **20 effects** (`alien.fx-*`: chroma, RGB split, glitch blocks, pixel streak,
  CRT, VHS, grain, bloom, light leak, shake, punch pulse, lens, vignette,
  halftone, duotone, posterize, edge glow, kaleido, liquid glass, gradient),
  all original (`docs/THIRD_PARTY_DATAPACK.md`).
- **12 more colour looks** (`luts/`, seeded once under marker `alien-looks-2`).
- **Safe-zone overlay** `alien.safe-zones` (add as an effect layer, hide before
  export). Numbers in `docs/SAFE_ZONES.md`; no platform publishes an official
  pixel table, so they are conservative defaults with stated confidence.
- **12 more templates** (ALIEN-19 to ALIEN-30, including the LUNAR ROAST 15 s
  reel) and **9 text presets** (Alien Lesson, karaoke, pop, box, one-word,
  shadow, Arabic poster, Ramadan, Eid); templates are seeded once under marker
  `alien-templates-2`. The library now holds thirty templates.
- **Lesson spec check and mission plan** (`concat-project/src/preflight.rs`,
  `mission.rs`; MCP `project_preflight`, `export_mission_plan`; clip menu row
  "Check lesson spec"): the E.19/E.20/I.09/C.07 deliverables with exact file
  names and the ALIEN_OS path. Defaults the course does not settle are named
  constants marked `TODO(owner)`.
- **Add-ons:** nothing new is needed to drop packs in; see
  `.claude/council/sessions/ufo-cut-ideas-2026-11-14/ADDONS_CATALOG.md`.

Not yet seen by anyone in the window or run by any compiler: everything in this
section and the "Cut silences" item above.

Internal names (the `concat-*` crates, the `concat` binary, file formats and
effect ids) are unchanged, so UFO Cut projects stay compatible with Concat.

## Names

"Concat" and the Concat logo belong to the Concat project and are used here
only to say what UFO Cut is based on.
