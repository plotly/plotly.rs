use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
#[cfg(test)]
use std::println as debug;

use anyhow::{Context, Result};
#[cfg(not(test))]
use log::debug;
use rand::{
    distr::{Alphanumeric, SampleString},
    rng,
};

pub(crate) fn html_body(offline: bool) -> String {
    let offline_js = offline_js_sources();
    let cdn_js = online_js_cdn();

    // HTML with embedded script
    if offline {
        format!(
            r#"
        <!doctype html>
        <html lang="en">
            <body>
                <div>
                    {offline_js}
                </div>
                <div id="plotly-html-element" hidden></div>
            </body>
        </html>"#,
            offline_js = offline_js
        )
    } else {
        format!(
            r#"
        <!doctype html>
        <html lang="en">
            <body>
                <div>
                    {cdn_js}
                </div>
                <div id="plotly-html-element" hidden></div>
            </body>
        </html>"#,
            cdn_js = cdn_js
        )
    }
}

/// Save the html file to a temporary file
pub(crate) fn to_file(data: &str) -> Result<PathBuf> {
    debug!("Generate plotly html file");
    use std::env;
    // Set up the temp file with a unique filename.
    let mut tmp_path = env::temp_dir();
    let mut plot_name = Alphanumeric.sample_string(&mut rng(), 22);
    plot_name.push_str(".html");
    plot_name = format!("plotly_{}", plot_name);
    tmp_path.push(plot_name);

    // Save the rendered plot to the temp file.
    let temp_path = tmp_path
        .to_str()
        .context("Failed to convert path to string")?;
    let mut file = File::create(temp_path)?;
    file.write_all(data.as_bytes())?;
    file.flush()?;
    Ok(tmp_path)
}

fn offline_js_sources() -> String {
    let local_plotly_js = include_str!("../resource/plotly.min.js");
    let local_tex_mml_js = include_str!("../resource/tex-mml-chtml-3.2.0.js");
    let local_tex_svg_js = include_str!("../resource/tex-svg-3.2.2.js");
    format!(
        "<script type=\"text/javascript\">{}</script>\n
            <script type=\"text/javascript\">
            /**
             * tex-mml-chtml JS script
             **/
            {}
            </script>\n
            <script type=\"text/javascript\">
            /**
             * tex-svg JS script
             **/
            {}
            </script>\n",
        local_plotly_js, local_tex_mml_js, local_tex_svg_js
    )
    .to_string()
}

fn online_js_cdn() -> String {
    r##"
                <script src="https://cdn.plot.ly/plotly-2.12.1.min.js"></script>
                <script src="https://cdn.jsdelivr.net/npm/mathjax@3.2.2/es5/tex-svg.js"></script>
                <script src="https://cdn.jsdelivr.net/npm/mathjax@3.2.0/es5/tex-mml-chtml.js"></script>
    "##
    .to_string()
}
