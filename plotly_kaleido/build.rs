extern crate zip;
use std::env;
use std::io::Result;
use std::path::PathBuf;
use std::process::Command;

use std::fs;
use std::io;

#[cfg(target_os = "linux")]
const KALEIDO_URL: &str =
    "https://github.com/plotly/Kaleido/releases/download/v0.0.1rc9/kaleido_linux-0.0.1rc9.zip";
#[cfg(target_os = "windows")]
const KALEIDO_URL: &str =
    "https://github.com/plotly/Kaleido/releases/download/v0.0.1rc9/kaleido_win-0.0.1rc9.zip";
#[cfg(target_os = "macos")]
const KALEIDO_URL: &str =
    "https://github.com/plotly/Kaleido/releases/download/v0.0.1rc9/kaleido_mac-0.0.1rc9.zip";

fn extract_zip(p: &PathBuf, zip_file: &PathBuf) -> Result<()> {
    let file = fs::File::open(&zip_file).unwrap();
    let mut archive = zip::ZipArchive::new(file).unwrap();

    for i in 0..archive.len() {
        let mut file = archive.by_index(i).unwrap();
        let outpath = file.sanitized_name();
        let outpath = p.join(outpath);
        println!("outpath: {:?}", outpath);

        {
            let comment = file.comment();
            if !comment.is_empty() {
                println!("File {} comment: {}", i, comment);
            }
        }

        if (&*file.name()).ends_with('/') {
            println!(
                "File {} extracted to \"{}\"",
                i,
                outpath.as_path().display()
            );
            fs::create_dir_all(&outpath).unwrap();
        } else {
            println!(
                "File {} extracted to \"{}\" ({} bytes)",
                i,
                outpath.as_path().display(),
                file.size()
            );
            if let Some(p) = outpath.parent() {
                if !p.exists() {
                    fs::create_dir_all(&p).unwrap();
                }
            }
            let mut outfile = fs::File::create(&outpath).unwrap();
            io::copy(&mut file, &mut outfile).unwrap();
        }

        // Get and Set permissions
        #[cfg(unix)]
        {
            use std::os::unix::fs::PermissionsExt;

            if let Some(mode) = file.unix_mode() {
                fs::set_permissions(&outpath, fs::Permissions::from_mode(mode)).unwrap();
            }
        }
    }

    fs::remove_file(zip_file)?;
    Ok(())
}

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=src/lib.rs");
    let p = PathBuf::from(env::var("OUT_DIR").unwrap());
    let mut dst = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    dst = dst.parent().unwrap().to_path_buf();
    dst = dst.join("plotly_kaleido");

    let kaleido_zip_file = p.join("kaleido.zip");

    let mut cmd = Command::new("cargo")
        .args(&["install", "ruget"])
        .spawn()
        .unwrap();
    cmd.wait()?;

    let mut cmd = Command::new("ruget")
        .args(&[
            KALEIDO_URL,
            "-o",
            kaleido_zip_file.as_path().to_str().unwrap(),
        ])
        .spawn()
        .unwrap();
    cmd.wait()?;

    extract_zip(&dst, &kaleido_zip_file)?;
    Ok(())
}
