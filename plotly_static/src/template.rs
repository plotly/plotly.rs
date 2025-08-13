use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
#[cfg(any(test, feature = "debug"))]
use std::println as debug;

use anyhow::{Context, Result};
#[cfg(not(any(test, feature = "debug")))]
use log::debug;
use rand::{
    distr::{Alphanumeric, SampleString},
    rng,
};

pub(crate) fn image_export_js_script() -> String {
    r#"
    const plot = arguments[0];
    const format = arguments[1];
    const width = arguments[2];
    const height = arguments[3];
    const scale = arguments[4];
    const callback = arguments[arguments.length - 1];
    console.log(plot);
    const graph_div = document.getElementById("plotly-html-element");
    Plotly.newPlot(graph_div, plot).then(function() {
        return Plotly.toImage(graph_div, {
            format: format,
            width: width,
            height: height,
        });
    }).then(function(dataUrl) {
        callback(dataUrl);
    }).catch(function(err) {
        console.error('Plotly error:', err);
        callback('ERROR:' + err.toString());
    });
"#
    .to_string()
}

pub(crate) fn pdf_export_js_script(timeout_ms: u32) -> String {
    let foreign_object_rendering = if cfg!(feature = "chromedriver") {
        "true"
    } else {
        "false"
    };

    format!(
        r##"
    const plot = arguments[0];
    const format = arguments[1];
    const width = arguments[2];
    const height = arguments[3];
    const scale = arguments[4];
    const callback = arguments[arguments.length - 1];

    console.log('Starting PDF export process...');
    console.log('Plot data:', plot);
    console.log('Dimensions:', width, 'x', height);

    const graph_div = document.getElementById('plotly-html-element');

    // Check if html2pdf is available and wait for it to load
    const waitForHtml2Pdf = (maxWaitMs = 5000) => {{
        return new Promise((resolve, reject) => {{
            const startTime = Date.now();
            
            const checkLibrary = () => {{
                if (typeof html2pdf !== 'undefined' && html2pdf) {{
                    console.log('html2pdf library loaded successfully');
                    resolve();
                }} else if (Date.now() - startTime > maxWaitMs) {{
                    console.error('html2pdf library failed to load within timeout');
                    reject(new Error('html2pdf library not loaded within timeout'));
                }} else {{
                    setTimeout(checkLibrary, 100);
                }}
            }};
            
            checkLibrary();
        }});
    }};

    // Wait for html2pdf to be available before proceeding
    waitForHtml2Pdf().then(() => {{
        console.log('html2pdf library is ready, starting PDF export...');
        
        let tempDiv = null;

        const cleanup = () => {{
            if (tempDiv) {{
                document.body.removeChild(tempDiv);
            }}
        }};

        Plotly.newPlot(graph_div, plot).then(function() {{
            return Plotly.toImage(graph_div, {{
            format: format,
            width: width,
            height: height,
            }});
        }}).then(function(dataUrl) {{
            console.log('Plotly image generated successfully');
            console.log('SVG data URL length:', dataUrl.length);
            console.log('SVG data URL preview:', dataUrl.substring(0, 200) + '...');
            console.log('PDF dimensions (px):', width, 'x', height);

            // Create a temporary div for the image
            tempDiv = document.createElement('div');
            tempDiv.style.width = width + 'px';
            tempDiv.style.height = height + 'px';
            tempDiv.style.background = 'white';
            tempDiv.style.position = 'fixed';
            tempDiv.style.top = '0px';
            tempDiv.style.left = '0px';
            tempDiv.style.margin = '0px';
            tempDiv.style.padding = '0px';
            tempDiv.style.overflow = 'hidden';
            tempDiv.style.boxSizing = 'border-box';
            tempDiv.style.zIndex = '9999';
            tempDiv.style.display = 'block';
            document.body.appendChild(tempDiv);

            // Use simple img approach with SVG data URL
            console.log('Using SVG data URL directly with img element');
            const img = document.createElement('img');
            img.src = dataUrl;
            img.style.width = '100%';
            img.style.height = '100%';
            img.style.display = 'block';
            img.style.objectFit = 'contain';
            img.style.maxWidth = '100%';
            img.style.maxHeight = '100%';
            img.style.verticalAlign = 'top';
            img.style.boxSizing = 'border-box';
            tempDiv.appendChild(img);

            // Wait for the image to load
            return new Promise(function(resolve) {{
                img.onload = function() {{
                    console.log('SVG image loaded successfully');
                    // Additional delay to ensure image is fully rendered
                    // Brief delay to ensure image is fully rendered
                    setTimeout(resolve, {timeout_ms});
                }};
                img.onerror = function() {{
                    cleanup();
                    callback('ERROR:Failed to load SVG image');
                }};
            }});
        }}).then(function() {{
            console.log('Starting PDF generation...');
            return html2pdf().from(tempDiv).set({{
                margin: 0,
                filename: 'plotly-plot.pdf',
                image: {{ type: 'jpeg', quality: 1}},
                html2canvas: {{
                    scale: scale,
                    backgroundColor: '#fff',
                    useCORS: true,
                    allowTaint: true,
                    logging: true,
                    width: width,
                    height: height,
                    imageTimeout: 15000,
                    removeContainer: true,
                    foreignObjectRendering: {foreign_object_rendering},
                    scrollY: 0,
                    scrollX: 0
                }},
                jsPDF: {{
                    unit: 'px',
                    format: [width, height],
                    orientation: width > height ? 'landscape' : 'portrait',
                    compress: true
                }}
            }}).toPdf().output('datauristring');
        }}).then(function(dataUri) {{
            cleanup();
                callback(dataUri);
        }}).catch(function(err) {{
            cleanup();
            callback('ERROR:' + err.toString());
        }});
    }}).catch((err) => {{
        console.error('Failed to load html2pdf library:', err);
        callback('ERROR:' + err.toString());
    }});
"##
    )
}

pub(crate) fn get_html_body(offline: bool) -> String {
    let offline_js = offline_js_sources();
    let cdn_js = online_js_cdn();
    if offline {
        html_body(&offline_js)
    } else {
        html_body(&cdn_js)
    }
}

pub(crate) fn html_body(js_source: &str) -> String {
    // HTML with embedded script
    let html = format!(
        r#"
        <!doctype html>
        <html lang="en">
        <head>
        <style>
            /* Ensures the image has no extra spacing */
            #plotly-img-element {{
                display: block;
                margin: 0;
                padding: 0;
                background: white;
            }}
        </style>
            {js_source}
        </head>
        <body>
            <div id="plotly-html-element" style="position: absolute; left: -9999px; top: -9999px; width: 1200px; height: 900px;"></div>
        </body>
        </html>"#
    );

    #[cfg(any(test, feature = "debug"))]
    if let Err(e) = to_file(&html) {
        debug!("Failed to save HTML to file: {e}");
    }

    html
}

/// Save the html file to a temporary file
pub(crate) fn to_file(data: &str) -> Result<PathBuf> {
    use std::env;
    // Set up the temp file with a unique filename.
    let mut tmp_path = env::temp_dir();
    let mut plot_name = Alphanumeric.sample_string(&mut rng(), 22);
    plot_name.push_str(".html");
    plot_name = format!("plotly_{plot_name}");
    tmp_path.push(plot_name);

    debug!("Write template plotly html file to {tmp_path:?}");

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
    // tex-mml-chtml conflicts with tex-svg when generating Latex Titles
    // let local_tex_mml_js = include_str!("../resource/tex-mml-chtml-3.2.0.js");
    let local_plotly_js = include_str!("../resource/plotly.min.js");
    let local_tex_svg_js = include_str!("../resource/tex-svg-3.2.2.js");
    let local_html2pdf_js = include_str!("../resource/html2pdf.bundle.min.js");
    format!(
        "<script type=\"text/javascript\">{local_plotly_js}</script>\n
         <script type=\"text/javascript\">{local_tex_svg_js}</script>\n
         <script type=\"text/javascript\">{local_html2pdf_js}</script>\n"
    )
    .to_string()
}

fn online_js_cdn() -> String {
    // tex-mml-chtml conflicts with tex-svg when generating Latex Titles
    r##"
    <script src="https://cdn.plot.ly/plotly-3.0.1.min.js"></script>
    <script src="https://cdn.jsdelivr.net/npm/mathjax@3.2.2/es5/tex-svg.js"></script>
    <script src="https://cdnjs.cloudflare.com/ajax/libs/html2pdf.js/0.10.1/html2pdf.bundle.min.js"></script>
    "##
    .to_string()
}
