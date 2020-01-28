extern crate askama;
extern crate num;
extern crate rand;
extern crate serde;

use askama::Template;
use rand::Rng;
use std::env;
use std::fs::File;
use std::io::Write;
use std::process::Command;

pub mod charts;

use crate::charts::Layout;

#[derive(Template)]
#[template(path = "plotly-latest.min.js", escape = "none")]
struct PlotlyJs;

#[derive(Template)]
#[template(path = "plot.html", escape = "none")]
struct PlotTemplate<'a> {
    plot_data: &'a str,
    plotly_javascript: &'a str,
    export_image: bool,
    image_type: &'a str,
    image_width: usize,
    image_height: usize,
}

pub trait TraceSerialize {
    fn serialize(&self) -> String;
}

pub struct Plot {
    traces: Vec<Box<dyn TraceSerialize>>,
    layout: Option<Layout>,
}

impl Plot {
    pub fn new() -> Plot {
        Plot {
            traces: Vec::with_capacity(1),
            layout: None,
        }
    }

    pub fn scatter<X, Y>(name: &str, x: Vec<X>, y: Vec<Y>) -> Plot
        where X: 'static + serde::Serialize,
              Y: 'static + num::Num + serde::Serialize
    {
        let mut plot = Plot {
            traces: Vec::with_capacity(1),
            layout: None,
        };

        let mut trace = charts::Scatter::new(name, x, y);
        trace.mode = charts::Mode::Markers;
        plot.add_trace(trace);
        plot
    }

    pub fn bar<X, Y>(name: &str, x: Vec<X>, y: Vec<Y>) -> Plot
        where X: 'static + serde::Serialize,
              Y: 'static + num::Num + serde::Serialize
    {
        let mut plot = Plot {
            traces: Vec::with_capacity(1),
            layout: None,
        };

        let mut trace = charts::Bar::new(name, x, y);
        plot.add_trace(trace);
        plot
    }

    pub fn candlestick<T, O>(time: Vec<T>, open: Vec<O>, high: Vec<O>, low: Vec<O>, close: Vec<O>) -> Plot
        where T: 'static + serde::Serialize,
              O: 'static + num::Num + serde::Serialize
    {
        let mut plot = Plot {
            traces: Vec::with_capacity(1),
            layout: None,
        };

        let trace = charts::Candlestick::new(time, open, high, low, close);
        plot.add_trace(trace);
        plot
    }

    pub fn ohlc<T, O>(time: Vec<T>, open: Vec<O>, high: Vec<O>, low: Vec<O>, close: Vec<O>) -> Plot
        where T: 'static + serde::Serialize,
              O: 'static + num::Num + serde::Serialize {
        let mut plot = Plot {
            traces: Vec::with_capacity(1),
            layout: None,
        };

        let trace = charts::Ohlc::new(time, open, high, low, close);
        plot.add_trace(trace);
        plot
    }

    pub fn add_trace(&mut self, trace: Box<dyn TraceSerialize>) {
        self.traces.push(trace);
    }

    pub fn add_layout(&mut self, layout: Layout) {
        self.layout = Some(layout);
    }

    fn render(&self, export_image: bool, image_type: &str, image_width: usize, image_height: usize) -> String {
        let mut plot_data = String::new();
        for (idx, trace) in self.traces.iter().enumerate() {
            let s = trace.serialize();
            plot_data.push_str(format!("var trace_{} = {};\n", idx, s).as_str());
        }
        plot_data.push_str("\n");
        plot_data.push_str("var data = [");
        for idx in 0..self.traces.len() {
            if idx != self.traces.len() - 1 {
                plot_data.push_str(format!("trace_{},", idx).as_str());
            } else {
                plot_data.push_str(format!("trace_{}", idx).as_str());
            }
        }
        plot_data.push_str("];\n");
        let layout_data = match &self.layout {
            Some(layout) => format!("var layout = {};\n", TraceSerialize::serialize(layout)),
            None => {
                let mut s = String::from("var layout = {");
                s.push_str("};\n");
                s
            }
        };
        plot_data.push_str(layout_data.as_str());

        let plotly_js = PlotlyJs {}.render().unwrap();
        let tmpl = PlotTemplate {
            plot_data: plot_data.as_str(),
            plotly_javascript: plotly_js.as_str(),
            export_image,
            image_type,
            image_width,
            image_height,
        };
        tmpl.render().unwrap()
    }

    #[cfg(target_os = "macos")]
    fn show_with_default_app(temp_path: &str) {
        Command::new("open").args(&[temp_path]).output().unwrap();
    }

    #[cfg(target_os = "linux")]
    fn show_with_default_app(temp_path: &str) {
        Command::new("xdg-open")
            .args(&[temp_path])
            .output()
            .unwrap();
    }

    #[cfg(target_os = "windows")]
    fn show_with_default_app(temp_path: &str) {
        Command::new("start")
            .args(&[r#""""#, temp_path])
            .output()
            .unwrap();
    }

    pub fn show(&self) {
        let rendered = self.render(false, "", 0, 0);
        let rendered = rendered.as_bytes();
        let mut temp = env::temp_dir();

        let mut plot_name = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(22)
            .collect::<String>();
        plot_name.push_str(".html");

        temp.push(plot_name);
        let temp_path = temp.to_str().unwrap();
        let mut file = File::create(temp_path).unwrap();
        file.write_all(rendered)
            .expect("failed to write html output");

        Plot::show_with_default_app(temp_path);
    }

    pub fn show_png(&self, width: usize, height: usize) {
        let rendered = self.render(true, "png", width, height);
        let rendered = rendered.as_bytes();
        let mut temp = env::temp_dir();

        let mut plot_name = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(22)
            .collect::<String>();
        plot_name.push_str(".html");

        temp.push(plot_name);
        let temp_path = temp.to_str().unwrap();
        let mut file = File::create(temp_path).unwrap();
        file.write_all(rendered)
            .expect("failed to write html output");

        Plot::show_with_default_app(temp_path);
    }

    pub fn show_jpg(&self, width: usize, height: usize) {
        let rendered = self.render(true, "jpg", width, height);
        let rendered = rendered.as_bytes();
        let mut temp = env::temp_dir();

        let mut plot_name = rand::thread_rng()
            .sample_iter(&rand::distributions::Alphanumeric)
            .take(22)
            .collect::<String>();
        plot_name.push_str(".html");

        temp.push(plot_name);
        let temp_path = temp.to_str().unwrap();
        let mut file = File::create(temp_path).unwrap();
        file.write_all(rendered)
            .expect("failed to write html output");

        Plot::show_with_default_app(temp_path);
    }

    pub fn to_html(&self, filename: &str) {
        let rendered = self.render(false, "", 0, 0);
        let rendered = rendered.as_bytes();
        let mut file = File::create(filename).unwrap();
        file.write_all(rendered)
            .expect("failed to write html output");
    }

    pub fn to_png(&self, filename: &str, width: usize, height: usize) {
        let rendered = self.render(true, "png", width, height);
        let rendered = rendered.as_bytes();
        let mut file = File::create(filename).unwrap();
        file.write_all(rendered)
            .expect("failed to write html output");
    }

    pub fn to_jpg(&self, filename: &str, width: usize, height: usize) {
        let rendered = self.render(true, "jpg", width, height);
        let rendered = rendered.as_bytes();
        let mut file = File::create(filename).unwrap();
        file.write_all(rendered)
            .expect("failed to write html output");
    }
}
