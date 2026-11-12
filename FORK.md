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

Internal names (the `concat-*` crates, the `concat` binary, file formats and
effect ids) are unchanged, so UFO Cut projects stay compatible with Concat.

## Names

"Concat" and the Concat logo belong to the Concat project and are used here
only to say what UFO Cut is based on.
