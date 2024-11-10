use std::{
    env,
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
};

const ASSET_PATH_VAR: &str = "BEVY_ASSET_PATH";
const OUTPUT_FILE_NAME: &str = "bevy_i18n.rs";
const ALLOWED_EXTENSIONS: &[&str] = &["otf", "ttf"];

fn main() {
    cargo_emit::rerun_if_env_changed!(ASSET_PATH_VAR);

    let out_dir = env::var_os("OUT_DIR").unwrap();

    let mut files = Vec::new();

    // Check if env variable is set for the assets folder
    if let Some(dir) = env::var(ASSET_PATH_VAR)
        .ok()
        .map(|v| Path::new(&v).to_path_buf())
        .and_then(|path| {
            if path.exists() {
                Some(path)
            } else {
                cargo_emit::warning!(
                    "${} points to an unknown folder: {}",
                    ASSET_PATH_VAR,
                    path.to_string_lossy()
                );
                None
            }
        })
        // Otherwise, search for the target folder and look for an assets folder next to it
        .or_else(|| {
            env::var("OUT_DIR")
                .ok()
                .map(|v| Path::new(&v).to_path_buf())
                .and_then(|path| {
                    for ancestor in path.ancestors() {
                        if let Some(last) = ancestor.file_name() {
                            if last == "target" {
                                return ancestor.parent().map(|parent| {
                                    let imported_dir = parent.join("imported_assets");
                                    if imported_dir.exists() {
                                        imported_dir.join("Default")
                                    } else {
                                        parent.join("assets")
                                    }
                                });
                            }
                        }
                    }
                    None
                })
                .and_then(|path| {
                    if path.exists() {
                        Some(path)
                    } else {
                        cargo_emit::warning!(
                            "Could not find asset folder from Cargo build directory"
                        );
                        None
                    }
                })
        })
    {
        cargo_emit::rerun_if_changed!(dir.to_string_lossy());
        // cargo_emit::warning!("Asset folder found: {}", dir.to_string_lossy());

        let building_for_wasm = std::env::var("CARGO_CFG_TARGET_ARCH") == Ok("wasm32".to_string());

        visit_dirs(&dir)
            .iter()
            .map(|path| (path, path.strip_prefix(&dir).unwrap()))
            .for_each(|(full_path, path)| {
                let mut string_path = path.to_string_lossy().to_string();
                if building_for_wasm {
                    // building for wasm. replace paths with forward slash in case we're building from windows
                    string_path = string_path.replace(std::path::MAIN_SEPARATOR, "/");
                }
                cargo_emit::rerun_if_changed!(full_path.to_string_lossy());
                if let Some(ext) = full_path.extension().and_then(|e| e.to_str()) {
                    if ALLOWED_EXTENSIONS.contains(&ext) {
                        // Extract filename without extension
                        let locale = path.file_stem().unwrap().to_string_lossy().into_owned();

                        // Extract file extension
                        let ext = path.extension().unwrap().to_string_lossy().into_owned();

                        // Extract the most immediate folder name
                        let family = path
                            .parent()
                            .unwrap()
                            .file_name()
                            .unwrap()
                            .to_string_lossy()
                            .into_owned();

                        files.push(FontAsset {
                            is_fallback: locale == "fallback",
                            path: PathBuf::from(string_path),
                            family,
                            locale,
                            ext,
                        });

                        // cargo_emit::warning!("{} {} {}", file_name, extension, folder);
                        // if let Some(family) = path.parent() {
                        //     let family = family.to_string_lossy().to_string();
                        //     if let Some(locale) = full_path.file_stem() {
                        //         let locale = locale.to_string_lossy().to_string();
                        //         files.push(FontAsset {
                        //             is_fallback: locale == family,
                        //             path: string_path,
                        //             family,
                        //             locale,
                        //         });
                        //     }
                        // }
                    }
                }
            });
    } else if std::env::var("DOCS_RS").is_ok() {
        // We're building the docs, so we don't need to do anything
    } else {
        cargo_emit::warning!(
            "Could not find asset folder, please specify its path with ${}",
            ASSET_PATH_VAR
        );
        // panic!("No asset folder found");
    }

    let mut families: Vec<FontFamily> = Vec::new();
    for asset in files.iter() {
        if let Some(family) = families.iter_mut().find(|f| f.folder == asset.family) {
            family
                .locales
                .push(format!("{}.{}", asset.locale, asset.ext));
        } else {
            families.push(FontFamily {
                path: asset.path.parent().unwrap().to_string_lossy().to_string(),
                folder: asset.family.clone(),
                locales: if asset.is_fallback {
                    vec![]
                } else {
                    vec![format!("{}.{}", asset.locale, asset.ext)]
                },
            });
        }
    }
    let mut marker_file = File::create(Path::new(&out_dir).join(OUTPUT_FILE_NAME)).unwrap();

    marker_file
        .write_all(
            format!(
                r#"#[derive(Debug)]
pub(crate) struct FontFamily {{
    pub path: &'static str,
    pub family: &'static str,
    pub locales: &'static [&'static str],
}}

{}
pub(crate) const FONT_FAMILIES: &'static [FontFamily] = &[{}];
"#,
                families
                    .iter()
                    .map(|s| s.write())
                    .collect::<Vec<_>>()
                    .join("\n"),
                families
                    .iter()
                    .map(|s| s.push_const())
                    .collect::<Vec<_>>()
                    .join(", ")
            )
            .as_bytes(),
        )
        .unwrap();
}

struct FontAsset {
    path: PathBuf,
    ext: String,
    family: String,
    locale: String,
    is_fallback: bool,
}

struct FontFamily {
    path: String,
    folder: String,
    locales: Vec<String>,
}

impl FontFamily {
    fn write(&self) -> String {
        format!(
            r#"pub(crate) const {}: FontFamily = FontFamily {{
    path: {:?},
    family: "{}",
    locales: &{:?},
}};
"#,
            self.snake_case().to_uppercase(),
            self.path,
            self.folder,
            self.locales
        )
    }

    fn push_const(&self) -> String {
        format!("{}", self.snake_case().to_uppercase())
    }

    fn snake_case(&self) -> String {
        let mut snake_case = String::new();
        let name = self.folder.replace(&['/', '\\', '.', '-'][..], "_");
        let mut prev_char = '\0';
        for (i, ch) in name.chars().enumerate() {
            if ch.is_uppercase() && i > 0 && prev_char != '_' {
                snake_case.push('_');
            }
            snake_case.push(ch.to_ascii_lowercase());
            prev_char = ch;
        }
        snake_case
    }
}

fn visit_dirs(dir: &Path) -> Vec<PathBuf> {
    let mut collected = vec![];
    if dir.is_dir() {
        for entry in fs::read_dir(dir).unwrap() {
            let entry = entry.unwrap();
            let path = entry.path();
            if path.is_dir() {
                collected.append(&mut visit_dirs(&path));
            } else {
                collected.push(path);
            }
        }
    }
    collected
}
