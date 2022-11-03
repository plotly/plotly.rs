use plotly::{ImageFormat, Plot, Scatter};

fn main() {
    let mut plot = Plot::new();
    let trace = Scatter::new(vec![0, 1, 2], vec![2, 1, 0]);
    plot.add_trace(trace);

    // Adjust these arguments to set the image format, width and height of the
    // output image.
    let filename = "out";
    let image_format = ImageFormat::PNG;
    let width = 800;
    let height = 600;
    let scale = 1.0;

    // The image will be saved to format!("{filename}.{image_format}") relative to
    // the current working directory.
    plot.write_image(filename, image_format, width, height, scale);
}
