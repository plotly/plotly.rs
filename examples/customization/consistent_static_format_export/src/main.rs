use log::info;
use plotly::color::{NamedColor, Rgb};
use plotly::common::{Anchor, Font, Line, Marker, MarkerSymbol, Mode, Title};
use plotly::layout::{Axis, ItemSizing, Legend, Margin, Shape, ShapeLine, ShapeType};
use plotly::plotly_static::{ImageFormat, StaticExporterBuilder};
use plotly::prelude::*;
use plotly::{Layout, Plot, Scatter};

fn line_and_scatter_plot(
    x1: Vec<f64>,
    y1: Vec<f64>,
    x2: Vec<f64>,
    y2: Vec<f64>,
    file_name: &str,
    title: &str,
) {
    let bgcol = Rgb::new(255, 255, 255);
    let linecol1 = NamedColor::DarkBlue;
    let linecol2 = NamedColor::DarkRed;
    let forecol = Rgb::new(0, 0, 0);
    let gridcol = Rgb::new(180, 180, 180);
    let transp = NamedColor::Transparent;
    let thick: usize = 3;
    let medium: usize = 3;
    let _thin: usize = 2;
    let msize: usize = 10;
    let fsz_title: usize = 40;
    let fsz_legend: usize = 35;
    let fsz_ticks: usize = 30;
    let fsz_axes: usize = 35;

    let trace1 = Scatter::new(x1, y1)
        .name("Diffusion")
        .mode(Mode::Lines)
        .line(Line::new().color(linecol1).width(medium as f64));

    let trace2 = Scatter::new(x2, y2)
        .name("Random walk")
        .mode(Mode::Markers)
        .marker(
            Marker::new()
                .size(msize)
                .color(linecol2)
                .symbol(MarkerSymbol::Circle),
        );

    let title = Title::from(title).font(
        Font::new()
            .size(fsz_title)
            .family("Times New Roman, serif")
            .color(forecol),
    );

    let legend = Legend::new()
        .x(0.99)
        .x_anchor(Anchor::Right)
        .y(1.0 - 0.0133)
        .y_anchor(Anchor::Top)
        .font(
            Font::new()
                .size(fsz_legend)
                .color(forecol)
                .family("Times New Roman, serif"),
        )
        .border_width(medium)
        .border_color(forecol)
        .background_color(bgcol)
        .item_width(52)
        .item_sizing(ItemSizing::Trace);

    let axis = Axis::new()
        .position(0.0)
        .show_line(true)
        .line_color(forecol)
        .line_width(thick)
        .tick_length(9)
        .tick_width(medium)
        .tick_color(forecol)
        .tick_font(Font::new().color(forecol))
        .zero_line(false)
        .show_grid(true)
        .grid_color(gridcol);

    let axisx = axis.clone().title(
        Title::from("x. a.u.").font(
            Font::new()
                .size(fsz_axes)
                .color(forecol)
                .family("Times New Roman, serif"),
        ),
    );

    let axisy = axis
        .clone()
        .title(
            Title::from("Number of particles").font(
                Font::new()
                    .size(fsz_axes)
                    .color(forecol)
                    .family("Times New Roman, serif"),
            ),
        )
        .tick_angle(270.0);

    let line_top = Shape::new()
        .shape_type(ShapeType::Line)
        .x_ref("paper")
        .y_ref("paper")
        .x0(0.)
        .y0(1.)
        .x1(1.)
        .y1(1.)
        .line(ShapeLine::new().color(forecol).width(thick as f64));

    let line_right = Shape::new()
        .shape_type(ShapeType::Line)
        .x_ref("paper")
        .y_ref("paper")
        .x0(1.)
        .y0(0.)
        .x1(1.)
        .y1(1.)
        .line(ShapeLine::new().color(forecol).width(thick as f64));

    let mut layout = Layout::new()
        .width(1024)
        .height(768)
        .font(Font::new().size(fsz_ticks))
        .title(title)
        .legend(legend)
        .show_legend(true)
        .x_axis(axisx)
        .y_axis(axisy)
        .plot_background_color(transp)
        .paper_background_color(bgcol)
        .margin(Margin::new().left(105).bottom(105));

    layout.add_shape(line_top);
    layout.add_shape(line_right);

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.set_layout(layout);

    let mut exporter = StaticExporterBuilder::default()
        .spawn_webdriver(true)
        .webdriver_port(4444)
        .build()
        .unwrap();

    info!("Exporting to PNG format...");
    exporter
        .write_image(&plot, file_name, ImageFormat::PNG, 1280, 960, 1.0)
        .unwrap();
    info!("Exporting to SVG format...");
    exporter
        .write_image(&plot, file_name, ImageFormat::SVG, 1280, 960, 1.0)
        .unwrap();
    info!("Exporting to PDF format...");
    exporter
        .write_image(&plot, file_name, ImageFormat::PDF, 1280, 960, 1.0)
        .unwrap();

    info!("Export complete. Check the output files:");
    info!("  - {file_name}.pdf");
    info!("  - {file_name}.svg");
    info!("  - {file_name}.png");

    // Always close the exporter to ensure proper release of WebDriver resources
    exporter.close();
}

fn read_from_file(file_path: &str) -> Vec<Vec<f64>> {
    use std::fs::File;
    use std::io::BufRead;
    use std::io::BufReader;

    let file = File::open(file_path).expect("Error opening file");
    let reader = BufReader::new(file);
    let mut file_contents: Vec<String> = Vec::new();

    for line in reader.lines() {
        file_contents.push(line.unwrap());
    }

    let mut data_rows: Vec<Vec<f64>> = Vec::new();
    let mut data_columns: Vec<Vec<f64>> = Vec::new();

    let mut n_columns: Vec<usize> = Vec::new();
    let mut n_rows: usize = 0;

    for line in file_contents {
        let row: Vec<f64> = line
            .split([' ', '\t', ','])
            .map(|s| s.trim())
            .filter(|s| !s.is_empty())
            .map(|s| s.parse().unwrap())
            .collect();

        n_columns.push(row.len());
        n_rows += 1;

        data_rows.push(row)
    }

    let n1 = n_columns[1];

    for _i in 0..n1 {
        let column: Vec<f64> = Vec::new();
        data_columns.push(column)
    }

    for j in 0..n_rows {
        for (i, column) in data_columns.iter_mut().enumerate().take(n_columns[j]) {
            column.push(data_rows[j][i])
        }
    }

    data_columns
}

fn main() {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("info")).init();

    let data = read_from_file("assets/data_file.dat");
    let x1 = data[0].clone();
    let y1 = data[1].clone();
    let x2 = data[3].clone();
    let y2 = data[4].clone();

    line_and_scatter_plot(x1, y1, x2, y2, "Data_plot", "Initial state δ(x)");
}
