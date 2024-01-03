use std::fs;
use std::path::{Path, PathBuf};
use zip_archive::Format;
use zip_archive::Archiver;

fn main() {
    println!("Building");

    std::process::Command::new("cargo").arg("build").arg("--release").stdout(std::process::Stdio::null()).output().expect("Failed to build!");

    println!("Moving files");

    fs::create_dir_all("./build").expect("Failed to create build dir");
    copy_dir("./assets", "./build/assets").expect("Failed to copy assets dir");
    fs::copy("./target/release/first-test.exe", "./build/first-test.exe").expect("Failed to copy exe");
    fs::copy("./build_readme.txt", "./build/README.txt").expect("Failed to copy readme");

    let origin = PathBuf::from("./build");
    let dest = PathBuf::from("./");

    println!("Zipping");

    let mut archiver = Archiver::new();
    archiver.push(origin);
    archiver.set_destination(dest);
    archiver.set_format(Format::Zip);
    archiver.archive().expect("Failed to zip");

    println!("Finished: created build directory and build.zip");
}   

// From https://stackoverflow.com/questions/26958489/how-to-copy-a-folder-recursively-in-rust
fn copy_dir(src: impl AsRef<Path>, dst: impl AsRef<Path>) -> std::io::Result<()> {
    fs::create_dir_all(&dst)?;
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let ty = entry.file_type()?;
        if ty.is_dir() {
            copy_dir(entry.path(), dst.as_ref().join(entry.file_name()))?;
        } else {
            fs::copy(entry.path(), dst.as_ref().join(entry.file_name()))?;
        }
    }
    Ok(())
}

