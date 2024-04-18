use std::{
    env, fs,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::Context;

pub fn main() {
    let out_dir = env::var("OUT_DIR").expect("could not find 'out_dir'");

    let cargo_target_dir =
        get_cargo_target_dir(Path::new(&out_dir)).expect("could not find 'target_dir'");

    let root_dir = cargo_target_dir
        .parent()
        .expect("could not find 'root_dir'");

    let root_style_path = root_dir.join("style").join("lumx");
    let root_style_ui_path = root_style_path.join("lumx-ui.scss");

    fs::create_dir_all(&root_style_path).unwrap();

    Command::new("npx")
        .args([
            "tailwindcss",
            "-i",
            "./tailwind-entry.css",
            "-o",
            &root_style_ui_path.to_str().unwrap(),
        ])
        .status()
        .expect("unable to build lumx styles");
}

fn get_cargo_target_dir(out_dir: impl AsRef<Path>) -> anyhow::Result<PathBuf> {
    let mut target_dir = None;
    let mut sub_path = out_dir.as_ref();

    while let Some(parent) = sub_path.parent() {
        if parent.ends_with("target") {
            target_dir = Some(parent);
            break;
        }
        sub_path = parent;
    }
    let target_dir = target_dir.with_context(|| {
        format!(
            "Could not find `target` dir in parents of {}",
            out_dir.as_ref().display()
        )
    })?;

    Ok(target_dir.to_path_buf())
}
