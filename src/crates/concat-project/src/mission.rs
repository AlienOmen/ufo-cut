// SPDX-License-Identifier: AGPL-3.0-or-later
// SPDX-FileCopyrightText: 2026 A.L.I.E.N. (UFO Cut)

//! The course's video missions: what each lesson hands in, under what exact
//! name, and where in the student's ALIEN_OS folder it goes.
//!
//! The Alien Designer course names every file a student submits
//! (`FOLDER_SYSTEM.md`, and each lesson's "Deliver ..." line). This module
//! holds those names as data, builds the folder path the same way the UFO
//! app does (`ALIEN_OS_<name>/02_Course1_Alien_Designer/<module>/<lesson
//! folder>/...`), picks the next free version so nothing is overwritten and
//! no name ever says "final", and says what to export with. It exports
//! nothing and writes nothing: it only answers "what would the names and
//! settings be".
//!
//! Where the lessons leave a choice open, the default is a named constant
//! with a `TODO(owner)` beside it.

use std::path::{Path, PathBuf};

use crate::preflight::{
    CAPTION_FONT, LessonSpec, MUSIC_BELOW_SPEECH_DB, SAFE_BOTTOM, SAFE_RIGHT, SAFE_SIDE, SAFE_TOP,
    SPEC_C07, SPEC_E19, SPEC_E20, SPEC_I09, SPEECH_LEVEL_DB, fps_text,
};

/// The lesson a check or a plan uses when no lesson is named and the
/// project's name does not start with a lesson code.
pub const DEFAULT_LESSON: &str = "E.19";

/// How many digits a version number is written with: `_v01`. TODO(owner):
/// the lessons write `_v01` and `_v02`; `FOLDER_SYSTEM.md` writes `_v2`,
/// `_v3`. The lessons' form is the default because it is what students are
/// shown next to the file name.
pub const VERSION_DIGITS: usize = 2;

/// Whether a lesson's files go in a folder named for the submission
/// (`E.19_AI_Video_Finishing`) inside the lesson folder. TODO(owner): the
/// lessons say "Deliver E.19_AI_Video_Finishing containing ...", so the
/// submission is a folder of that name; `FOLDER_SYSTEM.md` shows the file
/// name straight under the lesson folder. The folder is the default, because
/// a lesson that hands in several files needs one.
pub const PACKAGE_FOLDER_INSIDE_LESSON: bool = true;

/// How many versions are tried before giving up on finding a free name.
const MAX_ATTEMPTS: u32 = 99;

/// What the app does with a student's name for the folder: at most this
/// many characters, as in the UFO app.
const NAME_LENGTH: usize = 40;

/// What a lesson folder's title is cut to, in characters, as in the UFO app.
const TITLE_LENGTH: usize = 40;

/// The characters taken out of a student's name, as in the UFO app.
const FORBIDDEN: &str = "\\/:*?\"<>|";

/// The capstone's motion file, without version or extension. TODO(owner):
/// C.07 asks for "the Final MP4, 15 seconds, plus a Thumbnail" and gives no
/// name; the capstone checklist's example is `Motion_15s.mp4`, which does not
/// follow the Code_Project_Asset_Version rule E.23 teaches. This follows the
/// rule; the capstone is the student's own project, so the brand is not
/// LunarRoast. Which E.19/E.20 file is "the final" is also not said.
pub const C07_MOTION_STEM: &str = "C.07_Capstone_Motion_15s";

/// The I.09 animatic as the lesson writes it. TODO(owner): I.09 names it
/// `Animatic_15s.mp4` with no lesson code and no version, unlike every
/// other file; kept as written.
pub const I09_ANIMATIC_STEM: &str = "Animatic_15s";

/// What to export with: the numbers `export_start` takes.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct ExportProfile {
    /// Frame width in pixels.
    pub width: u32,
    /// Frame height in pixels.
    pub height: u32,
    /// Frame rate numerator.
    pub rate_num: i64,
    /// Frame rate denominator.
    pub rate_den: i64,
    /// "h264", "hevc" or "av1".
    pub codec: &'static str,
    /// Constant rate factor: lower is better and bigger.
    pub crf: u8,
    /// The x264 speed preset.
    pub preset: &'static str,
}

/// The Master: E.19 says "high quality". TODO(owner): the lessons give no
/// codec, bit rate or size cap for the Master (only C.07 says H.264 MP4).
/// UFO Cut sets quality by a factor, not a bit rate, so this is H.264 at
/// factor 18 with the slow preset, at the lesson's frame.
pub const MASTER_PROFILE: ExportProfile = ExportProfile {
    width: 1080,
    height: 1920,
    rate_num: 30,
    rate_den: 1,
    codec: "h264",
    crf: 18,
    preset: "slow",
};

/// The Proof: E.19 says "light, for review" and never says how light.
/// TODO(owner): a 720 x 1280 picture at factor 28, about a tenth of the
/// Master's weight, is the default.
pub const PROOF_PROFILE: ExportProfile = ExportProfile {
    width: 720,
    height: 1280,
    rate_num: 30,
    rate_den: 1,
    codec: "h264",
    crf: 28,
    preset: "fast",
};

/// What is handed in: the lesson's frame at the app's everyday quality
/// (factor 20, the middle of the export sheet). TODO(owner): no spec in the
/// lessons for E.17, E.20 or C.07 beyond "MP4 (H.264)".
pub const DELIVERY_PROFILE: ExportProfile = ExportProfile {
    width: 1080,
    height: 1920,
    rate_num: 30,
    rate_den: 1,
    codec: "h264",
    crf: 20,
    preset: "medium",
};

/// One file a lesson hands in.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Deliverable {
    /// What it is: "master", "proof", "captioned", "subtitles", "animatic",
    /// "motion", "animation".
    pub role: &'static str,
    /// The name without version and extension.
    pub stem: &'static str,
    /// Whether the name carries a version (`_v01`). A name without one
    /// takes a version only when its bare name is taken.
    pub versioned: bool,
    /// The extension, without the dot.
    pub ext: &'static str,
    /// What to export with, for a file UFO Cut renders; `None` for a file
    /// it does not (the .srt).
    pub export: Option<ExportProfile>,
}

/// Where a lesson's files go.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Folder {
    /// In the lesson's own folder, as the course's table says.
    Lesson,
    /// In a fixed folder under `02_Course1_Alien_Designer`, written with `/`.
    Fixed(&'static str),
}

/// One lesson of the course that hands in a video.
#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Lesson {
    /// The code: "E.19".
    pub code: &'static str,
    /// The module's folder: "E_Execution".
    pub module_dir: &'static str,
    /// The lesson's English title, from which its folder is named.
    pub title: &'static str,
    /// The submission's name, from the lesson index: "E.19_AI_Video_Finishing".
    pub package: &'static str,
    /// Where the files go.
    pub folder: Folder,
    /// The number the first set of files carries: E.20's captioned video is
    /// `_v02` because the picture was `_v01` in E.19.
    pub first_version: u32,
    /// What the lesson hands in.
    pub deliverables: &'static [Deliverable],
    /// What the lesson asks of the video, when it asks anything the
    /// preflight can read.
    pub spec: Option<LessonSpec>,
}

/// The lessons that hand in a video, in the order of the course. E.21 (a
/// Figma prototype) and E.23 (a zip, a register and notes) have no video
/// of their own, so they are not here.
pub const LESSONS: &[Lesson] = &[
    Lesson {
        code: "I.09",
        module_dir: "I_Intelligent_AI",
        title: "Directing AI Video: From Idea to Shots",
        package: "I.09_AI_Video_Shot_Briefs",
        folder: Folder::Lesson,
        first_version: 1,
        deliverables: &[Deliverable {
            role: "animatic",
            stem: I09_ANIMATIC_STEM,
            versioned: false,
            ext: "mp4",
            export: Some(DELIVERY_PROFILE),
        }],
        spec: Some(SPEC_I09),
    },
    Lesson {
        code: "E.17",
        module_dir: "E_Execution",
        title: "Animation, Scheduling, Collaboration and Approval",
        package: "E.17_Animation_Approval",
        folder: Folder::Lesson,
        first_version: 1,
        deliverables: &[Deliverable {
            role: "animation",
            stem: "E.17_LunarRoast_Slide1_Animated",
            versioned: false,
            ext: "mp4",
            export: Some(DELIVERY_PROFILE),
        }],
        spec: None,
    },
    Lesson {
        code: "E.19",
        module_dir: "E_Execution",
        title: "AI Video Finishing in Premiere, Resolve or CapCut",
        package: "E.19_AI_Video_Finishing",
        folder: Folder::Lesson,
        first_version: 1,
        deliverables: &[
            Deliverable {
                role: "master",
                stem: "E.19_LunarRoast_15s_MASTER",
                versioned: true,
                ext: "mp4",
                export: Some(MASTER_PROFILE),
            },
            Deliverable {
                role: "proof",
                stem: "E.19_LunarRoast_15s_PROOF",
                versioned: true,
                ext: "mp4",
                export: Some(PROOF_PROFILE),
            },
        ],
        spec: Some(SPEC_E19),
    },
    Lesson {
        code: "E.20",
        module_dir: "E_Execution",
        title: "Sound, Captions, Accessibility and Delivery",
        package: "E.20_Video_Delivery_Package",
        folder: Folder::Lesson,
        first_version: 2,
        deliverables: &[
            Deliverable {
                role: "captioned",
                stem: "E.20_LunarRoast_15s_Captioned",
                versioned: true,
                ext: "mp4",
                export: Some(DELIVERY_PROFILE),
            },
            Deliverable {
                role: "subtitles",
                stem: "E.20_LunarRoast_15s",
                versioned: false,
                ext: "srt",
                export: None,
            },
        ],
        spec: Some(SPEC_E20),
    },
    Lesson {
        code: "C.07",
        module_dir: "C_Capstone",
        title: "Submission Package",
        package: "C.07_Submission_Package",
        folder: Folder::Fixed("C_Capstone/_Capstone_Project/07_Motion_Video"),
        first_version: 1,
        deliverables: &[Deliverable {
            role: "motion",
            stem: C07_MOTION_STEM,
            versioned: true,
            ext: "mp4",
            export: Some(DELIVERY_PROFILE),
        }],
        spec: Some(SPEC_C07),
    },
];

/// The lesson with this code ("E.19", in any case, spaces around ignored).
pub fn lesson(code: &str) -> Option<&'static Lesson> {
    let code = code.trim();
    LESSONS
        .iter()
        .find(|lesson| lesson.code.eq_ignore_ascii_case(code))
}

/// The lesson a project's name starts with: "E.20_LunarRoast_Motion_v02"
/// is E.20's. `None` when the name does not start with a lesson code.
pub fn lesson_in_name(name: &str) -> Option<&'static Lesson> {
    LESSONS.iter().find(|lesson| starts_with_code(name, lesson.code))
}

/// Whether `name` starts with `code` and the code ends there (E.19 does not
/// start "E.190").
fn starts_with_code(name: &str, code: &str) -> bool {
    match (name.get(..code.len()), name.get(code.len()..)) {
        (Some(head), Some(rest)) => {
            head.eq_ignore_ascii_case(code)
                && rest.chars().next().is_none_or(|next| !next.is_alphanumeric())
        }
        _ => false,
    }
}

/// The codes of every lesson here, for a message that says what is known.
pub fn codes() -> Vec<&'static str> {
    LESSONS.iter().map(|lesson| lesson.code).collect()
}

/// A student's name as the ALIEN_OS folder carries it: the characters
/// `\ / : * ? " < > |` are taken out, runs of white space become one `_`,
/// and it is cut to 40 characters, as the UFO app does. A trailing dot is
/// dropped too, which Windows would silently remove from a folder's name.
/// Empty when nothing is left.
pub fn safe_name(name: &str) -> String {
    let kept: String = name
        .trim()
        .chars()
        .filter(|c| !FORBIDDEN.contains(*c) && !c.is_control())
        .collect();
    let mut out = String::new();
    let mut spaced = false;
    for c in kept.chars() {
        if c.is_whitespace() {
            if !spaced {
                out.push('_');
            }
            spaced = true;
        } else {
            out.push(c);
            spaced = false;
        }
    }
    let cut: String = out.chars().take(NAME_LENGTH).collect();
    cut.trim_end_matches('.').to_owned()
}

/// A lesson's title as its folder carries it: every character that is not a
/// letter or a digit becomes `_`, the ends are trimmed of `_`, a doubled `_`
/// is made single, and it is cut to 40 characters - the UFO app's rule.
fn slug(title: &str) -> String {
    let marked: String = title
        .chars()
        .map(|c| if c.is_alphanumeric() { c } else { '_' })
        .collect();
    let squeezed = marked.trim_matches('_').replace("__", "_");
    let cut: String = squeezed.chars().take(TITLE_LENGTH).collect();
    cut.trim_matches('_').to_owned()
}

/// The lesson's folder: its code and its title, `E.19_AI_Video_Finishing_in_Premiere_Resolve_o`.
pub fn lesson_folder(lesson: &Lesson) -> String {
    format!("{}_{}", lesson.code, slug(lesson.title))
}

/// True when a file name says "final" in any case. No name here does, and
/// the check is there for the tests and for a caller's own names.
pub fn says_final(name: &str) -> bool {
    name.to_ascii_lowercase().contains("final")
}

/// The name of `deliverable` on the `attempt`th try (from 0): the first try
/// is the lesson's own version, each later one the next number. A name
/// without a version is bare on the first try and takes `_v02` on the
/// second.
fn name_for(lesson: &Lesson, deliverable: &Deliverable, attempt: u32) -> String {
    if !deliverable.versioned && attempt == 0 {
        return format!("{}.{}", deliverable.stem, deliverable.ext);
    }
    format!(
        "{}_v{:0width$}.{}",
        deliverable.stem,
        lesson.first_version + attempt,
        deliverable.ext,
        width = VERSION_DIGITS
    )
}

/// One file of a plan.
#[derive(Clone, PartialEq, Debug)]
pub struct PlannedFile {
    /// What it is; see [`Deliverable::role`].
    pub role: &'static str,
    /// Its exact name.
    pub name: String,
    /// Its full path.
    pub path: PathBuf,
    /// What to export with, when UFO Cut renders it.
    pub export: Option<ExportProfile>,
}

/// Where a lesson's files go and what they are called.
#[derive(Clone, PartialEq, Debug)]
pub struct Plan {
    /// The lesson.
    pub lesson: &'static Lesson,
    /// The folder every file goes in.
    pub folder: PathBuf,
    /// The files, in the lesson's order.
    pub files: Vec<PlannedFile>,
    /// What the student should know, in plain words.
    pub notes: Vec<String>,
}

/// The folder a lesson's files go in, under `root`, for `student`.
pub fn folder_for(
    lesson: &Lesson,
    student: &str,
    root: &Path,
) -> Result<PathBuf, String> {
    let name = safe_name(student);
    if name.is_empty() {
        return Err("name the student first: the folder is ALIEN_OS_<name>".to_owned());
    }
    let mut folder = root.join(format!("ALIEN_OS_{name}"));
    folder.push("02_Course1_Alien_Designer");
    match lesson.folder {
        Folder::Lesson => {
            folder.push(lesson.module_dir);
            folder.push(lesson_folder(lesson));
            if PACKAGE_FOLDER_INSIDE_LESSON {
                folder.push(lesson.package);
            }
        }
        Folder::Fixed(path) => {
            for part in path.split('/') {
                folder.push(part);
            }
        }
    }
    Ok(folder)
}

/// The plan for `lesson`: its folder under `root` for `student`, and the
/// names of its files at the first version where none of them exists yet,
/// so a second export never overwrites the first. `exists` says whether a
/// path is taken; the caller supplies it (the disk's, or a test's).
pub fn plan(
    lesson: &'static Lesson,
    student: &str,
    root: &Path,
    exists: &dyn Fn(&Path) -> bool,
) -> Result<Plan, String> {
    let folder = folder_for(lesson, student, root)?;
    let mut chosen: Option<Vec<String>> = None;
    for attempt in 0..MAX_ATTEMPTS {
        let names: Vec<String> = lesson
            .deliverables
            .iter()
            .map(|deliverable| name_for(lesson, deliverable, attempt))
            .collect();
        if names.iter().all(|name| !exists(&folder.join(name))) {
            chosen = Some(names);
            break;
        }
    }
    let Some(names) = chosen else {
        return Err(format!(
            "every version of {}'s files up to {MAX_ATTEMPTS} is taken in {}",
            lesson.code,
            folder.display()
        ));
    };
    let files: Vec<PlannedFile> = lesson
        .deliverables
        .iter()
        .zip(names)
        .map(|(deliverable, name)| PlannedFile {
            role: deliverable.role,
            path: folder.join(&name),
            name,
            export: deliverable.export,
        })
        .collect();
    Ok(Plan {
        lesson,
        folder,
        files,
        notes: notes(lesson),
    })
}

/// What the student should know, in plain words, from the lesson's own
/// numbers.
fn notes(lesson: &Lesson) -> Vec<String> {
    let mut notes: Vec<String> = Vec::new();
    if let Some(spec) = lesson.spec {
        notes.push(format!(
            "The lesson asks for a {} x {} frame at {} fps, {} s long.",
            spec.width,
            spec.height,
            fps_text(spec.rate_num, spec.rate_den),
            spec.duration
        ));
        if let Some(lines) = spec.max_caption_lines {
            notes.push(format!(
                "Captions: {CAPTION_FONT}, Off-White on a Void Black bar, {lines} lines at most, \
                 kept between {:.0}% and {:.0}% of the frame's height and {:.0}% from the sides \
                 ({:.0}% on the right, where the platform's buttons are).",
                SAFE_TOP * 100.0,
                (1.0 - SAFE_BOTTOM) * 100.0,
                SAFE_SIDE * 100.0,
                SAFE_RIGHT * 100.0
            ));
        }
        if spec.audio.is_some() {
            notes.push(format!(
                "Sound: speech around {SPEECH_LEVEL_DB} dB on the meter, music {} to {} dB below \
                 it while someone talks.",
                MUSIC_BELOW_SPEECH_DB.0, MUSIC_BELOW_SPEECH_DB.1
            ));
        }
    }
    notes.push(
        "Names follow Code_Project_Asset_Version, with no spaces and never the word final. A new \
         export gets the next free version; nothing is overwritten."
            .to_owned(),
    );
    notes.push("Watch the exported file on a phone, outside the editor, before you hand it in.".to_owned());
    notes
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    fn nothing(_: &Path) -> bool {
        false
    }

    fn lesson_of(code: &str) -> &'static Lesson {
        lesson(code).expect("a lesson")
    }

    #[test]
    fn the_lesson_folders_are_the_ones_the_app_builds() {
        assert_eq!(
            lesson_folder(lesson_of("E.19")),
            "E.19_AI_Video_Finishing_in_Premiere_Resolve_o"
        );
        assert_eq!(
            lesson_folder(lesson_of("E.20")),
            "E.20_Sound_Captions_Accessibility_and_Deliver"
        );
        assert_eq!(
            lesson_folder(lesson_of("E.17")),
            "E.17_Animation_Scheduling_Collaboration_and_A"
        );
        assert_eq!(
            lesson_folder(lesson_of("I.09")),
            "I.09_Directing_AI_Video_From_Idea_to_Shots"
        );
        assert_eq!(lesson_folder(lesson_of("C.07")), "C.07_Submission_Package");
    }

    #[test]
    fn a_title_with_punctuation_becomes_single_underscores() {
        assert_eq!(slug("Welcome to A.L.I.E.N. Academy"), "Welcome_to_A_L_I_E_N_Academy");
        assert_eq!(slug("  --Hello, World!--  "), "Hello_World");
    }

    #[test]
    fn a_name_is_made_safe_as_the_app_does() {
        assert_eq!(safe_name("  Sara  Ali "), "Sara_Ali");
        assert_eq!(safe_name("Sa/ra:*?\"<>|"), "Sara");
        assert_eq!(safe_name(&"x".repeat(60)).chars().count(), 40);
        assert_eq!(safe_name("Sara."), "Sara");
        assert_eq!(safe_name("///"), "");
    }

    #[test]
    fn the_e19_files_land_in_the_alien_os_path_with_exact_names() {
        let plan = plan(lesson_of("E.19"), "Sara", Path::new("/work"), &nothing).expect("plans");
        let names: Vec<&str> = plan.files.iter().map(|file| file.name.as_str()).collect();
        assert_eq!(
            names,
            vec![
                "E.19_LunarRoast_15s_MASTER_v01.mp4",
                "E.19_LunarRoast_15s_PROOF_v01.mp4"
            ]
        );
        let expected = Path::new("/work")
            .join("ALIEN_OS_Sara")
            .join("02_Course1_Alien_Designer")
            .join("E_Execution")
            .join("E.19_AI_Video_Finishing_in_Premiere_Resolve_o")
            .join("E.19_AI_Video_Finishing");
        assert_eq!(plan.folder, expected);
        assert_eq!(plan.files[0].path, expected.join(&plan.files[0].name));
        let master = plan.files[0].export.expect("the master has settings");
        let proof = plan.files[1].export.expect("the proof has settings");
        assert_eq!((master.width, master.height), (1080, 1920));
        assert!(proof.width < master.width && proof.crf > master.crf);
    }

    #[test]
    fn e20_is_v02_with_a_bare_srt() {
        let plan = plan(lesson_of("E.20"), "Sara", Path::new("/w"), &nothing).expect("plans");
        let names: Vec<&str> = plan.files.iter().map(|file| file.name.as_str()).collect();
        assert_eq!(
            names,
            vec![
                "E.20_LunarRoast_15s_Captioned_v02.mp4",
                "E.20_LunarRoast_15s.srt"
            ]
        );
        assert!(plan.files[1].export.is_none(), "the srt is not rendered");
    }

    #[test]
    fn a_taken_name_moves_the_whole_set_to_the_next_version() {
        let taken: HashSet<String> = [
            "E.19_LunarRoast_15s_MASTER_v01.mp4",
            "E.19_LunarRoast_15s_MASTER_v02.mp4",
        ]
        .iter()
        .map(|name| (*name).to_owned())
        .collect();
        let exists = |path: &Path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| taken.contains(name))
        };
        let plan = plan(lesson_of("E.19"), "Sara", Path::new("/w"), &exists).expect("plans");
        let names: Vec<&str> = plan.files.iter().map(|file| file.name.as_str()).collect();
        assert_eq!(
            names,
            vec![
                "E.19_LunarRoast_15s_MASTER_v03.mp4",
                "E.19_LunarRoast_15s_PROOF_v03.mp4"
            ]
        );
    }

    #[test]
    fn a_taken_srt_takes_the_videos_next_version() {
        let exists = |path: &Path| {
            path.file_name()
                .and_then(|name| name.to_str())
                .is_some_and(|name| name == "E.20_LunarRoast_15s.srt")
        };
        let plan = plan(lesson_of("E.20"), "Sara", Path::new("/w"), &exists).expect("plans");
        assert_eq!(plan.files[0].name, "E.20_LunarRoast_15s_Captioned_v03.mp4");
        assert_eq!(plan.files[1].name, "E.20_LunarRoast_15s_v03.srt");
    }

    #[test]
    fn when_every_version_is_taken_it_says_so_instead_of_overwriting() {
        let everything = |_: &Path| true;
        let result = plan(lesson_of("E.19"), "Sara", Path::new("/w"), &everything);
        assert!(result.is_err());
    }

    #[test]
    fn the_capstone_motion_goes_to_the_capstone_project() {
        let plan = plan(lesson_of("c.07"), "Sara", Path::new("/w"), &nothing).expect("plans");
        let expected = Path::new("/w")
            .join("ALIEN_OS_Sara")
            .join("02_Course1_Alien_Designer")
            .join("C_Capstone")
            .join("_Capstone_Project")
            .join("07_Motion_Video");
        assert_eq!(plan.folder, expected);
        assert_eq!(plan.files[0].name, "C.07_Capstone_Motion_15s_v01.mp4");
    }

    #[test]
    fn an_unnamed_student_is_refused() {
        let result = plan(lesson_of("E.19"), " /: ", Path::new("/w"), &nothing);
        assert!(result.is_err());
    }

    #[test]
    fn no_name_says_final_and_none_has_a_space() {
        for lesson in LESSONS {
            for attempt in 0..3 {
                for deliverable in lesson.deliverables {
                    let name = name_for(lesson, deliverable, attempt);
                    assert!(!says_final(&name), "{name}");
                    assert!(!name.contains(' '), "{name}");
                    assert!(name.ends_with(deliverable.ext), "{name}");
                }
            }
            assert!(!says_final(lesson.package), "{}", lesson.package);
        }
        assert!(says_final("slide3_Final.png"));
    }

    #[test]
    fn lessons_are_found_by_code_and_by_a_project_name() {
        assert!(lesson(" e.20 ").is_some());
        assert!(lesson("E.99").is_none());
        let by_name = lesson_in_name("E.20_LunarRoast_Motion_v02").expect("a lesson");
        assert_eq!(by_name.code, "E.20");
        assert!(lesson_in_name("e.19").is_some());
        assert!(lesson_in_name("E.190_thing").is_none());
        assert!(lesson_in_name("My holiday").is_none());
        assert!(lesson_in_name("").is_none());
        // A multi-byte character where the code would end is not a panic.
        assert!(lesson_in_name("E.1\u{00e9}").is_none());
    }

    #[test]
    fn every_lesson_with_a_spec_is_a_vertical_fifteen_seconds_at_thirty() {
        for lesson in LESSONS {
            if let Some(spec) = lesson.spec {
                assert_eq!((spec.width, spec.height), (1080, 1920), "{}", lesson.code);
                assert_eq!((spec.rate_num, spec.rate_den), (30, 1), "{}", lesson.code);
                assert_eq!(spec.duration, 15.0, "{}", lesson.code);
            }
        }
    }

    #[test]
    fn the_notes_quote_the_lessons_numbers() {
        let plan = plan(lesson_of("E.20"), "Sara", Path::new("/w"), &nothing).expect("plans");
        let text = plan.notes.join("\n");
        assert!(text.contains("1080 x 1920"), "{text}");
        assert!(text.contains("Cairo"), "{text}");
        assert!(text.contains("-6 dB"), "{text}");
        assert!(text.contains("never the word final"), "{text}");
    }
}
