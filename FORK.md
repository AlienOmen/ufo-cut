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

Internal names (the `concat-*` crates, the `concat` binary, file formats and
effect ids) are unchanged, so UFO Cut projects stay compatible with Concat.

## Names

"Concat" and the Concat logo belong to the Concat project and are used here
only to say what UFO Cut is based on.
