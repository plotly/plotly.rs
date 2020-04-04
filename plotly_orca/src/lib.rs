//! # Plotly Orca
//! Plotly Orca implements the `orca` feature for [Plotly for Rust](https://github.com/igiagkiozis/plotly)
//!
//! The `orca` feature enables `Plot` conversion to the following output formats: png, jpeg, webp, svg, pdf and eps.
//!
//! ## Installation instructions
//! To use `plotly_orca` which is used by the `orca` feature for `plotly`, first you need to install the
//! [Orca command line utility](https://github.com/plotly/orca).
//!
//! Download the appropriate binary of Orca for your system from [here](https://github.com/plotly/orca/releases).
//!
//! ### Linux
//! Copy the orca-<version>-x86_64.AppImage anywhere in your home directory.
//! Say you saved this in: /home/<user_name>/apps/orca-<version>-x86_64.AppImage
//! Then simply create a symbolic link pointing to the AppImage:
//!
//! ```bash
//! chmod +x /home/<user_name>/apps/orca-<version>-x86_64.AppImage
//! sudo ln -s /home/<user_name>/apps/orca-<version>-x86_64.AppImage /usr/bin/plotly_orca
//! ```
//!
//! Note, it's important that the symbolic link is named exactly as shown above. The name of the link is not `orca` as there
//! already exists an executable on RHEL 8 and Centos 8 with that name.
//!
//! ### MacOSX
//! Install the dmg package. After that the `orca` binary will be detected by `plotly_orca`.
//!
//! ### Windows
//! Run the installation executable with the default target path. After that `plotly_orca` will be able to find the `orca.exe`.

const ORCA_INSTALLATION_INSTRUCTIONS: &str = r#"
To use `plotly_orca` which is used by the `orca` feature for `plotly`, first you need to install the
[Orca command line utility](https://github.com/plotly/orca).

Download the appropriate binary of Orca for your system from [here](https://github.com/plotly/orca/releases).

### Linux
Copy the orca-<version>-x86_64.AppImage anywhere in your home directory.
Say you saved this in: /home/<user_name>/apps/orca-<version>-x86_64.AppImage
Then simply create a symbolic link pointing to the AppImage:

```bash
chmod +x /home/<user_name>/apps/orca-<version>-x86_64.AppImage
sudo ln -s /home/<user_name>/apps/orca-<version>-x86_64.AppImage /usr/bin/plotly_orca
```

Note, it's important that the symbolic link is named exactly as shown above. The name of the link is not `orca` as there
already exists an executable on RHEL 8 and Centos 8 with that name.

### MacOSX
Install the dmg package. After that the `orca` binary will be detected by `plotly_orca`.

### Windows
Run the installation executable with the default target path. After that `plotly_orca` will be able to find the `orca.exe`.
"#;

extern crate rand;
use rand::Rng;
use std::env;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::Command;

#[derive(Default)]
pub struct Orca {
    cmd_path: PathBuf,
    plotly_path: Option<PathBuf>,
}

impl Orca {
    pub fn new() -> Orca {
        let p = match Orca::find_orca_executable() {
            Ok(path) => path,
            Err(msg) => panic!(msg),
        };

        Orca {
            cmd_path: p,
            plotly_path: None,
        }
    }

    pub fn from<P: AsRef<Path>>(plotly_path: P) -> Orca {
        let plotly_path = PathBuf::from(plotly_path.as_ref());
        let p = match Orca::find_orca_executable() {
            Ok(path) => path,
            Err(msg) => panic!(msg),
        };

        if !plotly_path.exists() {
            return Orca {
                cmd_path: p,
                plotly_path: None,
            };
        }
        Orca {
            cmd_path: p,
            plotly_path: Some(plotly_path),
        }
    }

    fn save(&self, dst: &Path, plotly_data: &str, image_format: &str, width: usize, height: usize) {
        let mut dst = PathBuf::from(dst);
        dst.set_extension(image_format);

        let mut plot_data_path = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(22)
            .collect::<String>();
        plot_data_path.push_str(".json");

        let mut temp = env::temp_dir();
        temp.push(plot_data_path);
        let temp_path = temp.to_str().unwrap();
        {
            let mut file = File::create(temp_path).unwrap();
            file.write_all(plotly_data.as_bytes())
                .expect("failed to write html output");
            file.flush().unwrap();
        }

        let orca_executable = self.cmd_path.to_str().unwrap();
        let mut cmd = Command::new(orca_executable);
        cmd.arg("graph")
            .arg(&temp_path)
            .arg("-o")
            .arg(dst.to_str().unwrap())
            .arg("--width")
            .arg(format!("{}", width))
            .arg("--height")
            .arg(format!("{}", height))
            .arg("--format")
            .arg(image_format);

        match &self.plotly_path {
            Some(p) => cmd
                .arg("--plotly")
                .arg(p.to_str().unwrap())
                .output()
                .unwrap(),
            None => cmd.output().unwrap(),
        };

        // Cleanup
        std::fs::remove_file(temp_path).unwrap();
    }

    pub fn save_png<P: AsRef<Path>>(&self, dst: P, plot_data: &str, width: usize, height: usize) {
        self.save(dst.as_ref(), plot_data, "png", width, height);
    }

    pub fn save_jpeg<P: AsRef<Path>>(&self, dst: P, plot_data: &str, width: usize, height: usize) {
        self.save(dst.as_ref(), plot_data, "jpeg", width, height);
    }

    pub fn save_webp<P: AsRef<Path>>(&self, dst: P, plot_data: &str, width: usize, height: usize) {
        self.save(dst.as_ref(), plot_data, "webp", width, height);
    }

    pub fn save_svg<P: AsRef<Path>>(&self, dst: P, plot_data: &str, width: usize, height: usize) {
        self.save(dst.as_ref(), plot_data, "svg", width, height);
    }

    pub fn save_pdf<P: AsRef<Path>>(&self, dst: P, plot_data: &str, width: usize, height: usize) {
        self.save(dst.as_ref(), plot_data, "pdf", width, height);
    }

    pub fn save_eps<P: AsRef<Path>>(&self, dst: P, plot_data: &str, width: usize, height: usize) {
        self.save(dst.as_ref(), plot_data, "eps", width, height);
    }

    #[cfg(target_os = "linux")]
    fn find_orca_executable() -> Result<PathBuf, &'static str> {
        let p = PathBuf::from("/usr/bin/plotly_orca");
        if !p.exists() {
            return Err(ORCA_INSTALLATION_INSTRUCTIONS);
        }
        Ok(p)
    }

    #[cfg(target_os = "macos")]
    fn find_orca_executable() -> Result<PathBuf, &'static str>  {
        let orca_path = PathBuf::from("/Applications/orca.app/Contents/MacOS/orca");
        if !orca_path.exists() {
            return Err(ORCA_INSTALLATION_INSTRUCTIONS);
        }

        Ok(orca_path)
    }

    #[cfg(target_os = "windows")]
    fn find_orca_executable() -> Result<PathBuf, &'static str> {
        let app_data = std::env::var_os("LOCALAPPDATA").expect("Could not expand LOCALAPPDATA");
        let mut orca_path = PathBuf::from(app_data);
        orca_path.push("Programs");
        orca_path.push("orca");
        orca_path.push("orca.exe");
        if !orca_path.exists() {
            return Err(ORCA_INSTALLATION_INSTRUCTIONS);
        }

        Ok(orca_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_PLOT: &str = r#"{
    "data": [{"type":"scatter","x":[1,2,3,4],"y":[10,15,13,17],"name":"trace1","mode":"markers"},
             {"type":"scatter","x":[2,3,4,5],"y":[16,5,11,9],"name":"trace2","mode":"lines"},
             {"type":"scatter","x":[1,2,3,4],"y":[12,9,15,12],"name":"trace3"}],
    "layout": {}
    }"#;

    #[test]
    fn test_can_find_plotly_executable() {
        let _o = Orca::new();
    }

    #[test]
    fn test_save_png() {
        let o = Orca::new();
        let dst = PathBuf::from("example.png");
        assert!(!dst.exists());
        o.save_png(&dst, TEST_PLOT, 1024, 680);
        assert!(dst.exists());
        match std::fs::remove_file(&dst) {
            Ok(_) => {}
            Err(e) => panic!(format!("could not cleanup file, error: {}", e)),
        };
        assert!(!dst.exists());
    }

    #[test]
    fn test_save_jpeg() {
        let o = Orca::new();
        let dst = PathBuf::from("example.jpeg");
        assert!(!dst.exists());
        o.save_jpeg(&dst, TEST_PLOT, 1024, 680);
        assert!(dst.exists());
        match std::fs::remove_file(&dst) {
            Ok(_) => {}
            Err(e) => panic!(format!("could not cleanup file, error: {}", e)),
        };
        assert!(!dst.exists());
    }

    #[test]
    fn test_save_webp() {
        let o = Orca::new();
        let dst = PathBuf::from("example.webp");
        assert!(!dst.exists());
        o.save_webp(&dst, TEST_PLOT, 1024, 680);
        assert!(dst.exists());
        match std::fs::remove_file(&dst) {
            Ok(_) => {}
            Err(e) => panic!(format!("could not cleanup file, error: {}", e)),
        };
        assert!(!dst.exists());
    }

    #[test]
    fn test_save_svg() {
        let o = Orca::new();
        let dst = PathBuf::from("example.svg");
        assert!(!dst.exists());
        o.save_svg(&dst, TEST_PLOT, 1024, 680);
        assert!(dst.exists());
        match std::fs::remove_file(&dst) {
            Ok(_) => {}
            Err(e) => panic!(format!("could not cleanup file, error: {}", e)),
        };
        assert!(!dst.exists());
    }

    #[test]
    fn test_save_pdf() {
        let o = Orca::new();
        let dst = PathBuf::from("example.pdf");
        assert!(!dst.exists());
        o.save_pdf(&dst, TEST_PLOT, 1024, 680);
        assert!(dst.exists());
        match std::fs::remove_file(&dst) {
            Ok(_) => {}
            Err(e) => panic!(format!("could not cleanup file, error: {}", e)),
        };
        assert!(!dst.exists());
    }

    #[test]
    fn test_save_eps() {
        let o = Orca::new();
        let dst = PathBuf::from("example.eps");
        assert!(!dst.exists());
        o.save_eps(&dst, TEST_PLOT, 1024, 680);
        assert!(dst.exists());
        match std::fs::remove_file(&dst) {
            Ok(_) => {}
            Err(e) => panic!(format!("could not cleanup file, error: {}", e)),
        };
        assert!(!dst.exists());
    }
}
