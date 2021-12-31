extern crate zip;
extern crate directories;
use std::env;
use std::io::Result;
use std::path::PathBuf;
use std::process::Command;
use directories::ProjectDirs;
use std::fs;
use std::io;


#[cfg(target_os = "linux")]
const KALEIDO_URL: &str =
    "https://github.com/plotly/Kaleido/releases/download/v0.2.1/kaleido_linux_x64.zip";

#[cfg(target_os = "windows")]
const KALEIDO_URL: &str =
    "https://github.com/plotly/Kaleido/releases/download/v0.2.1/kaleido_win_x64.zip";

#[cfg(all(target_arch = "x86_64", target_os = "macos"))]
const KALEIDO_URL: &str =
    "https://github.com/plotly/Kaleido/releases/download/v0.2.1/kaleido_mac_x64.zip";

#[cfg(all(target_arch = "aarch64", target_os = "macos"))]
const KALEIDO_URL: &str =
    "https://github.com/plotly/Kaleido/releases/download/v0.2.1/kaleido_mac_arm64.zip";


#[cfg(target_os = "linux")]
const KALEIDO_BIN: &str = "kaleido";

#[cfg(target_os = "windows")]
const KALEIDO_BIN: &str = "kaleido.exe";

#[cfg(target_os = "macos")]
const KALEIDO_BIN: &str = "kaleido";

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
    let project_dirs = ProjectDirs::from("org", "plotly", "kaleido").expect("Could not create plotly_kaleido config directory.");
    let dst: PathBuf = project_dirs.config_dir().into();

    let kaleido_binary = dst.join("bin").join(KALEIDO_BIN);
    if kaleido_binary.exists() {
        return Ok(());
    }

    let p = PathBuf::from(env::var("OUT_DIR").unwrap());
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
    println!("cargo:rerun-if-changed=src/lib.rs");
    Ok(())
}
