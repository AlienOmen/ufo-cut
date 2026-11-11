// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 A.L.I.E.N. (Ahmed Albadry)

//! UFO Cut: the language chosen in the Windows setup.
//!
//! The setup asks which language to speak before anything else, and
//! writes the answer to `setup.ini` beside the program. A first run reads
//! it, so the app opens in that language and the person is asked once.

/// The app's locale code for the language the setup was run in, when the
/// setup left one and the app has it.
pub fn chosen() -> Option<&'static str> {
    let program = std::env::current_exe().ok()?;
    let text = std::fs::read_to_string(program.parent()?.join("setup.ini")).ok()?;
    let name = text
        .lines()
        .find_map(|line| line.trim().strip_prefix("Language="))?
        .trim()
        .to_ascii_lowercase();
    // The setup's names for its languages, as assets/windows/concat.iss
    // lists them.
    Some(match name.as_str() {
        "english" => "en",
        "german" => "de",
        "spanish" => "es",
        "french" => "fr",
        "italian" => "it",
        "japanese" => "ja",
        "brazilianportuguese" => "pt-BR",
        "russian" => "ru",
        "turkish" => "tr",
        "chinesesimplified" => "zh-Hans",
        "chinesetraditional" => "zh-TW",
        "korean" => "ko",
        "croatian" => "hr",
        "farsi" => "fa",
        "dutch" => "nl",
        "hebrew" => "he",
        "polish" => "pl",
        "ukrainian" => "uk",
        "arabic" => "ar",
        "thai" => "th",
        "bengali" => "bn",
        "hindi" => "hi",
        "indonesian" => "id",
        "urdu" => "ur",
        "vietnamese" => "vi",
        _ => return None,
    })
}
