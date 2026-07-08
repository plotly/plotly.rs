use plotly::{Configuration, Plot};

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
/// # Returns the path to the standalone HTML file
pub fn write_example_to_html(plot: &Plot, name: &str) -> String {
    write_example_to_html_with_inline_config(plot, name, None)
}

/// Write a plot to HTML files for documentation and display
///
/// This function creates both an inline HTML file (for mdbook inclusion) and
/// a standalone HTML file (for direct viewing). The inline file is prefixed
/// with "inline_" and both files are placed in the "./output" directory.
/// The plot for inline HTML display can be configured with the provided
/// `inline_config` when provided, otherwise the standalone inherited plot
/// configuration is used.
///
/// # Arguments
///
/// * `plot` - The plot to write to HTML
/// * `name` - The base name for the HTML files (without extension)
/// * `inline_config` - The configuration to use for the inline HTML file
///
/// # Returns the path to the standalone HTML file
pub fn write_example_to_html_with_inline_config(
    plot: &Plot,
    name: &str,
    inline_config: Option<Configuration>,
) -> String {
    std::fs::create_dir_all("./output").unwrap();

    let standalone_config = plot.configuration().clone();
    let inline_config = inline_config.unwrap_or(standalone_config.clone());

    // Write inline HTML
    let mut inline_plot = plot.clone();
    inline_plot.set_configuration(inline_config);
    let html = inline_plot.to_inline_html(Some(name));
    let inline_path = format!("./output/inline_{name}.html");
    std::fs::write(inline_path, html).unwrap();

    // Write standalone HTML
    let mut standalone_plot = plot.clone();
    standalone_plot.set_configuration(standalone_config);
    let path = format!("./output/{name}.html");
    standalone_plot.write_html(&path);
    path
}
