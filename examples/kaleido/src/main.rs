use plotly::{ImageFormat, Plot, Scatter};

fn main() {
    let mut plot = Plot::new();
    let trace = Scatter::new(vec![0, 1, 2], vec![2, 1, 0]);
    plot.add_trace(trace);

    // Adjust these arguments to set the width and height of the
    // output image.
    let width = 800;
    let height = 600;
    let scale = 1.0;

    std::fs::create_dir_all("./output").unwrap();
    let filename = "./output/inline_image".to_string();

    // The image will be saved to format!("output/image.{image_format}") relative to
    // the current working directory.
    plot.write_image(&filename, ImageFormat::EPS, width, height, scale);
    plot.write_image(&filename, ImageFormat::JPEG, width, height, scale);
    plot.write_image(&filename, ImageFormat::PDF, width, height, scale);
    plot.write_image(&filename, ImageFormat::PNG, width, height, scale);
    plot.write_image(&filename, ImageFormat::SVG, width, height, scale);
    plot.write_image(&filename, ImageFormat::WEBP, width, height, scale);

    let _svg_string = plot.to_svg(width, height, scale);
}
