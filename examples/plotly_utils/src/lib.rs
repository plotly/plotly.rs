use plotly::Plot;

/// Write a plot to HTML files for documentation and display
///
/// This function creates both an inline HTML file (for mdbook inclusion) and
/// a standalone HTML file (for direct viewing). The inline file is prefixed
/// with "inline_" and both files are placed in the "./output" directory.
///
/// # Arguments
///
/// * `plot` - The plot to write to HTML
/// * `name` - The base name for the HTML files (without extension)
///
/// # Returns
///
/// The path to the standalone HTML file
pub fn write_example_to_html(plot: &Plot, name: &str) -> String {
    std::fs::create_dir_all("./output").unwrap();
    // Write inline HTML
    let html = plot.to_inline_html(Some(name));
    let path = format!("./output/inline_{name}.html");
    std::fs::write(path, html).unwrap();
    // Write standalone HTML
    let path = format!("./output/{name}.html");
    plot.write_html(&path);
    path
}
