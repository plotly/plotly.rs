#![allow(dead_code)]

use ndarray::arr2;
use plotly::{color::Rgb, image::ColorModel, Image, Plot};

fn basic_image() {
    let w = Rgb::new(255, 255, 255);
    let b = Rgb::new(0, 0, 0);
    let r = Rgb::new(240, 8, 5);
    let db = Rgb::new(145, 67, 7);
    let lb = Rgb::new(251, 200, 129);
    let s = Rgb::new(153, 75, 10);
    let bl = Rgb::new(3, 111, 191);
    let y = Rgb::new(251, 250, 15);

    let pixels = vec![
        vec![w, w, w, w, r, r, r, r, r, w, w, w, w, w, w],
        vec![w, w, w, r, r, r, r, r, r, r, r, r, w, w, w],
        vec![w, w, w, db, db, db, lb, lb, b, lb, w, w, w, w, w],
        vec![w, w, db, lb, db, lb, lb, lb, b, lb, lb, lb, w, w, w],
        vec![w, w, db, lb, db, db, lb, lb, lb, db, lb, lb, lb, w, w],
        vec![w, w, db, db, lb, lb, lb, lb, db, db, db, db, w, w, w],
        vec![w, w, w, w, lb, lb, lb, lb, lb, lb, lb, w, w, w, w],
        vec![w, w, w, r, r, bl, r, r, r, w, w, w, w, w, w],
        vec![w, w, r, r, r, bl, r, r, bl, r, r, r, w, w, w],
        vec![w, r, r, r, r, bl, bl, bl, bl, r, r, r, r, w, w],
        vec![w, lb, lb, r, bl, y, bl, bl, y, bl, r, lb, lb, w, w],
        vec![w, lb, lb, lb, bl, bl, bl, bl, bl, bl, lb, lb, lb, w, w],
        vec![w, lb, lb, bl, bl, bl, bl, bl, bl, bl, bl, lb, lb, w, w],
        vec![w, w, w, bl, bl, bl, w, w, bl, bl, bl, w, w, w, w],
        vec![w, w, s, s, s, w, w, w, w, s, s, s, w, w, w],
        vec![w, s, s, s, s, w, w, w, w, w, s, s, s, s, w],
    ];
    let trace = Image::new(pixels).color_model(ColorModel::RGB);

    let mut plot = Plot::new();
    plot.add_trace(trace);

    plot.show()
}

fn trace_from_image_crate_rgb() {
    let im = image::open("assets/mario.png").unwrap().into_rgb8();
    let trace = Image::new(im).color_model(ColorModel::RGB);

    let mut plot = Plot::new();
    plot.add_trace(trace);

    plot.show()
}

fn trace_from_image_crate_rgba() {
    let im = image::open("assets/mario.png").unwrap().into_rgba8();
    let trace = Image::new(im).color_model(ColorModel::RGBA);

    let mut plot = Plot::new();
    plot.add_trace(trace);

    plot.show()
}

fn trace_from_ndarray_rgb() {
    let pixels = arr2(&[
        [(255, 255, 255), (0, 0, 0)],
        [(0, 0, 0), (255, 255, 255)],
        [(255, 255, 255), (0, 0, 0)],
    ]);
    let trace = Image::new(pixels).color_model(ColorModel::RGB);

    let mut plot = Plot::new();
    plot.add_trace(trace);

    plot.show()
}

fn trace_from_ndarray_rgba() {
    let pixels = arr2(&[
        [(255, 255, 255, 1.), (0, 0, 0, 0.25)],
        [(0, 0, 0, 0.5), (255, 255, 255, 1.)],
        [(255, 255, 255, 1.), (0, 0, 0, 0.75)],
    ]);
    let trace = Image::new(pixels).color_model(ColorModel::RGBA);

    let mut plot = Plot::new();
    plot.add_trace(trace);

    plot.show()
}

fn main() {
    // Uncomment any of these lines to display the example.

    // basic_image();
    // trace_from_image_crate_rgb();
    // trace_from_image_crate_rgba();
    // trace_from_ndarray_rgb();
    // trace_from_ndarray_rgba();
}
