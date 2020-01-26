extern crate askama;
extern crate serde;
extern crate num;
use serde::Serialize;
use askama::Template;
use std::fs::File;
use std::process::Command;
use std::io::Write;

pub mod charts;

use crate::charts::Scatter;
use crate::charts::Trace;

#[derive(Template)]
#[template(path = "plotly-latest.min.js", escape="none")]
struct PlotlyJs;

#[derive(Template)]
#[template(path = "plot.html", escape = "none")]
pub struct PlotTemplate<'a> {
    pub plot_data: &'a str,
    pub plotly_javascript: &'a str,
}

pub struct Plot<X, Y> where X: num::Num + serde::Serialize, Y: num::Num + serde::Serialize {
    traces: Vec<Trace<X, Y>>,
}

impl<X, Y> Plot<X, Y> where X: num::Num + serde::Serialize, Y: num::Num + serde::Serialize {
    pub fn new() -> Plot<X, Y> {
        Plot { traces: Vec::with_capacity(2)}
    }

    pub fn add_trace(&mut self, trace: Trace<X, Y>) {
        self.traces.push(trace);
    }

    pub fn show(&self) {
        let mut plot_data = String::new();
        for (idx, trace) in self.traces.iter().enumerate() {
            let mut s = match trace {
              Trace::Scatter(t) => { serde_json::to_string(&t).unwrap() },
              _ => panic!("not implemented"),
            };
            plot_data.push_str(format!("var trace_{} = {};\n", idx, s).as_str());
        }
        plot_data.push_str("\n");
        plot_data.push_str("var data = [");
        for idx in 0..self.traces.len() {
            if(idx != self.traces.len()-1) {
                plot_data.push_str(format!("trace_{},", idx).as_str());
            } else {
                plot_data.push_str(format!("trace_{}", idx).as_str());
            }
        }
        plot_data.push_str("];\n");

        let plotly_js = PlotlyJs{}.render().unwrap();

        let tmpl = PlotTemplate{plot_data: plot_data.as_str(), plotly_javascript: plotly_js.as_str()};
        let mut file = File::create("/tmp/plot.html").unwrap();
        file.write_all( tmpl.render().unwrap().as_bytes());

        let r = Command::new("xdg-open").args(&["/tmp/plot.html"]).output().unwrap();
    }
}

#[derive(Serialize, Debug)]
pub struct Axis {

}

#[derive(Serialize, Debug)]
pub struct LayoutTitle {

}

#[derive(Serialize, Debug)]
pub struct Layout {

}