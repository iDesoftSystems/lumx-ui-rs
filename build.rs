use std::{
    env::{self, VarError},
    fs::{self, File},
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

use anyhow::Context;

pub fn main() {
    let target_dir = env::var("OUT_DIR").expect("could not find 'out_dir'");
    let cargo_target_dir = get_cargo_target_dir(Path::new(&target_dir)).unwrap();

    // let root_dir = Path::new(&target_dir).parent().unwrap();
    let root_dir = cargo_target_dir.parent().unwrap();

    let style_dest_path = Path::new(&target_dir).join("lumx.css");
    let mut style_f = File::create(&style_dest_path).unwrap();
    style_f
        .write_all(b".lumx-card { color: #000000; } ")
        .unwrap();

    let root_ui_path = root_dir.join("lumx-ui.css");
    // let mut ui_f = File::create(&root_ui_path).unwrap();
    fs::copy("./assets/lumx-ui.css", root_ui_path).unwrap();
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
