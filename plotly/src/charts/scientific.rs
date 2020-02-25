use crate::charts::{
    owned_string_vector, Calendar, Color, ColorBar, ColorScale, Dim, Font, HoverInfo, Label, Line,
    PlotType,
};
use crate::Trace;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub enum ContoursType {
    #[serde(rename = "levels")]
    Levels,
    #[serde(rename = "constraint")]
    Constraint,
}

#[derive(Serialize, Debug)]
pub enum ContoursColoring {
    #[serde(rename = "fill")]
    Fill,
    #[serde(rename = "heatmap")]
    HeatMap,
    #[serde(rename = "lines")]
    Lines,
    #[serde(rename = "none")]
    None,
}

#[derive(Serialize, Debug)]
pub struct Contours {
    #[serde(skip_serializing_if = "Option::is_none")]
    r#type: Option<ContoursType>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    coloring: Option<ContoursColoring>,
    #[serde(skip_serializing_if = "Option::is_none")]
    show_lines: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "showlabels")]
    show_labels: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "labelfont")]
    label_font: Option<Font>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "labelformat")]
    label_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    operation: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    value: Option<f64>,
}

impl Contours {
    pub fn new() -> Contours {
        Contours {
            r#type: None,
            start: None,
            end: None,
            size: None,
            coloring: None,
            show_lines: None,
            show_labels: None,
            label_font: None,
            label_format: None,
            operation: None,
            value: None,
        }
    }

    pub fn type_(mut self, t: ContoursType) -> Contours {
        self.r#type = Some(t);
        self
    }

    pub fn start(mut self, start: f64) -> Contours {
        self.start = Some(start);
        self
    }

    pub fn end(mut self, end: f64) -> Contours {
        self.end = Some(end);
        self
    }

    pub fn size(mut self, size: usize) -> Contours {
        self.size = Some(size);
        self
    }

    pub fn coloring(mut self, coloring: ContoursColoring) -> Contours {
        self.coloring = Some(coloring);
        self
    }

    pub fn show_lines(mut self, show_lines: bool) -> Contours {
        self.show_lines = Some(show_lines);
        self
    }

    pub fn show_labels(mut self, show_labels: bool) -> Contours {
        self.show_labels = Some(show_labels);
        self
    }

    pub fn label_font(mut self, label_font: Font) -> Contours {
        self.label_font = Some(label_font);
        self
    }

    pub fn label_format(mut self, label_format: &str) -> Contours {
        self.label_format = Some(label_format.to_owned());
        self
    }

    pub fn operation(mut self, operation: &str) -> Contours {
        self.operation = Some(operation.to_owned());
        self
    }

    pub fn value(mut self, value: f64) -> Contours {
        self.value = Some(value);
        self
    }
}

#[derive(Serialize, Debug)]
pub struct Contour<X, Y, Z>
where
    X: num::Num + Serialize,
    Y: num::Num + Serialize,
    Z: num::Num + Serialize,
{
    r#type: PlotType,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "showlegend")]
    show_legend: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "legendgroup")]
    legend_group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opacity: Option<f64>,
    x: Vec<X>,
    y: Vec<Y>,
    z: Vec<Z>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hovertext")]
    hover_text: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverinfo")]
    hover_info: Option<HoverInfo>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hovertemplate")]
    hover_template: Option<Dim<String>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    line: Option<Line>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "colorbar")]
    color_bar: Option<ColorBar>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "autocolorscale")]
    auto_color_scale: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "colorscale")]
    color_scale: Option<ColorScale>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "showscale")]
    show_scale: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "reversescale")]
    reverse_scale: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zauto: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "zhoverformat")]
    zhover_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zmax: Option<Z>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zmid: Option<Z>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zmin: Option<Z>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "connectgaps")]
    connect_gaps: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contours: Option<Contours>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fill_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverlabel")]
    hover_label: Option<Label>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverongaps")]
    hover_on_gaps: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    n_contours: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transpose: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xcalendar")]
    x_calendar: Option<Calendar>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ycalendar")]
    y_calendar: Option<Calendar>,
}

impl<X, Y, Z> Contour<X, Y, Z>
where
    X: num::Num + Serialize,
    Y: num::Num + Serialize,
    Z: num::Num + Serialize,
{
    pub fn new(x: Vec<X>, y: Vec<Y>, z: Vec<Z>) -> Box<Contour<X, Y, Z>> {
        Box::new(Contour {
            r#type: PlotType::Contour,
            x,
            y,
            z,
            name: None,
            visible: None,
            show_legend: None,
            legend_group: None,
            opacity: None,
            text: None,
            hover_text: None,
            hover_info: None,
            hover_template: None,
            line: None,
            color_bar: None,
            auto_color_scale: None,
            color_scale: None,
            show_scale: None,
            reverse_scale: None,
            zauto: None,
            zhover_format: None,
            zmax: None,
            zmid: None,
            zmin: None,
            connect_gaps: None,
            contours: None,
            fill_color: None,
            hover_label: None,
            hover_on_gaps: None,
            n_contours: None,
            transpose: None,
            x_calendar: None,
            y_calendar: None,
        })
    }

    pub fn name(mut self, name: &str) -> Box<Contour<X, Y, Z>> {
        self.name = Some(name.to_owned());
        Box::new(self)
    }

    pub fn visible(mut self, visible: bool) -> Box<Contour<X, Y, Z>> {
        self.visible = Some(visible);
        Box::new(self)
    }

    pub fn show_legend(mut self, show_legend: bool) -> Box<Contour<X, Y, Z>> {
        self.show_legend = Some(show_legend);
        Box::new(self)
    }

    pub fn legend_group(mut self, legend_group: &str) -> Box<Contour<X, Y, Z>> {
        self.legend_group = Some(legend_group.to_owned());
        Box::new(self)
    }

    pub fn opacity(mut self, opacity: f64) -> Box<Contour<X, Y, Z>> {
        self.opacity = Some(opacity);
        Box::new(self)
    }

    pub fn text<S: AsRef<str>>(mut self, text: Vec<S>) -> Box<Contour<X, Y, Z>> {
        let text = owned_string_vector(text);
        self.text = Some(text);
        Box::new(self)
    }

    pub fn hover_text(mut self, hover_text: Vec<String>) -> Box<Contour<X, Y, Z>> {
        let hover_text = owned_string_vector(hover_text);
        self.hover_text = Some(hover_text);
        Box::new(self)
    }

    pub fn hover_info(mut self, hover_info: HoverInfo) -> Box<Contour<X, Y, Z>> {
        self.hover_info = Some(hover_info);
        Box::new(self)
    }

    pub fn hover_template(mut self, hover_template: &str) -> Box<Contour<X, Y, Z>> {
        self.hover_template = Some(Dim::Scalar(hover_template.to_owned()));
        Box::new(self)
    }

    pub fn hover_template_array<S: AsRef<str>>(
        mut self,
        hover_template: Vec<S>,
    ) -> Box<Contour<X, Y, Z>> {
        let hover_template = owned_string_vector(hover_template);
        self.hover_template = Some(Dim::Vector(hover_template));
        Box::new(self)
    }

    pub fn line(mut self, line: Line) -> Box<Contour<X, Y, Z>> {
        self.line = Some(line);
        Box::new(self)
    }

    pub fn color_bar(mut self, color_bar: ColorBar) -> Box<Contour<X, Y, Z>> {
        self.color_bar = Some(color_bar);
        Box::new(self)
    }

    pub fn auto_color_scale(mut self, auto_color_scale: bool) -> Box<Contour<X, Y, Z>> {
        self.auto_color_scale = Some(auto_color_scale);
        Box::new(self)
    }

    pub fn color_scale(mut self, color_scale: ColorScale) -> Box<Contour<X, Y, Z>> {
        self.color_scale = Some(color_scale);
        Box::new(self)
    }

    pub fn show_scale(mut self, show_scale: bool) -> Box<Contour<X, Y, Z>> {
        self.show_scale = Some(show_scale);
        Box::new(self)
    }

    pub fn reverse_scale(mut self, reverse_scale: bool) -> Box<Contour<X, Y, Z>> {
        self.reverse_scale = Some(reverse_scale);
        Box::new(self)
    }

    pub fn zauto(mut self, zauto: bool) -> Box<Contour<X, Y, Z>> {
        self.zauto = Some(zauto);
        Box::new(self)
    }

    pub fn zhover_format(mut self, zhover_format: &str) -> Box<Contour<X, Y, Z>> {
        self.zhover_format = Some(zhover_format.to_owned());
        Box::new(self)
    }

    pub fn zmax(mut self, zmax: Z) -> Box<Contour<X, Y, Z>> {
        self.zmax = Some(zmax);
        Box::new(self)
    }

    pub fn zmid(mut self, zmid: Z) -> Box<Contour<X, Y, Z>> {
        self.zmid = Some(zmid);
        Box::new(self)
    }

    pub fn zmin(mut self, zmin: Z) -> Box<Contour<X, Y, Z>> {
        self.zmin = Some(zmin);
        Box::new(self)
    }

    pub fn connect_gaps(mut self, connect_gaps: bool) -> Box<Contour<X, Y, Z>> {
        self.connect_gaps = Some(connect_gaps);
        Box::new(self)
    }

    pub fn contours(mut self, contours: Contours) -> Box<Contour<X, Y, Z>> {
        self.contours = Some(contours);
        Box::new(self)
    }

    pub fn fill_color(mut self, fill_color: Color) -> Box<Contour<X, Y, Z>> {
        self.fill_color = Some(fill_color);
        Box::new(self)
    }

    pub fn hover_label(mut self, hover_label: Label) -> Box<Contour<X, Y, Z>> {
        self.hover_label = Some(hover_label);
        Box::new(self)
    }

    pub fn hover_on_gaps(mut self, hover_on_gaps: bool) -> Box<Contour<X, Y, Z>> {
        self.hover_on_gaps = Some(hover_on_gaps);
        Box::new(self)
    }

    pub fn n_contours(mut self, n_contours: usize) -> Box<Contour<X, Y, Z>> {
        self.n_contours = Some(n_contours);
        Box::new(self)
    }

    pub fn transpose(mut self, transpose: bool) -> Box<Contour<X, Y, Z>> {
        self.transpose = Some(transpose);
        Box::new(self)
    }

    pub fn x_calendar(mut self, x_calendar: Calendar) -> Box<Contour<X, Y, Z>> {
        self.x_calendar = Some(x_calendar);
        Box::new(self)
    }

    pub fn y_calendar(mut self, y_calendar: Calendar) -> Box<Contour<X, Y, Z>> {
        self.y_calendar = Some(y_calendar);
        Box::new(self)
    }
}

impl<X, Y, Z> Trace for Contour<X, Y, Z>
where
    X: num::Num + Serialize,
    Y: num::Num + Serialize,
    Z: num::Num + Serialize,
{
    fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Serialize, Debug)]
pub struct HeatMap<X, Y, Z>
where
    X: num::Num + Serialize,
    Y: num::Num + Serialize,
    Z: num::Num + Serialize,
{
    r#type: PlotType,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "showlegend")]
    show_legend: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "legendgroup")]
    legend_group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opacity: Option<f64>,
    x: Vec<X>,
    y: Vec<Y>,
    z: Vec<Z>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hovertext")]
    hover_text: Option<Vec<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverinfo")]
    hover_info: Option<HoverInfo>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hovertemplate")]
    hover_template: Option<Dim<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "colorbar")]
    color_bar: Option<ColorBar>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "autocolorscale")]
    auto_color_scale: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "colorscale")]
    color_scale: Option<ColorScale>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "showscale")]
    show_scale: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "reversescale")]
    reverse_scale: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zauto: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "zhoverformat")]
    zhover_format: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zmax: Option<Z>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zmid: Option<Z>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zmin: Option<Z>,
    #[serde(skip_serializing_if = "Option::is_none")]
    zsmooth: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "connectgaps")]
    connect_gaps: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverlabel")]
    hover_label: Option<Label>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverongaps")]
    hover_on_gaps: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    transpose: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xcalendar")]
    x_calendar: Option<Calendar>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ycalendar")]
    y_calendar: Option<Calendar>,
}

impl<X, Y, Z> HeatMap<X, Y, Z>
where
    X: num::Num + Serialize,
    Y: num::Num + Serialize,
    Z: num::Num + Serialize,
{
    pub fn new(x: Vec<X>, y: Vec<Y>, z: Vec<Z>) -> Box<HeatMap<X, Y, Z>> {
        Box::new(HeatMap {
            x,
            y,
            z,
            r#type: PlotType::HeatMap,
            name: None,
            visible: None,
            show_legend: None,
            legend_group: None,
            opacity: None,
            text: None,
            hover_text: None,
            hover_info: None,
            hover_template: None,
            color_bar: None,
            auto_color_scale: None,
            color_scale: None,
            show_scale: None,
            reverse_scale: None,
            zauto: None,
            zhover_format: None,
            zmax: None,
            zmid: None,
            zmin: None,
            zsmooth: None,
            connect_gaps: None,
            hover_label: None,
            hover_on_gaps: None,
            transpose: None,
            x_calendar: None,
            y_calendar: None,
        })
    }

    pub fn name(mut self, name: &str) -> Box<HeatMap<X, Y, Z>> {
        self.name = Some(name.to_owned());
        Box::new(self)
    }

    pub fn visible(mut self, visible: bool) -> Box<HeatMap<X, Y, Z>> {
        self.visible = Some(visible);
        Box::new(self)
    }

    pub fn show_legend(mut self, show_legend: bool) -> Box<HeatMap<X, Y, Z>> {
        self.show_legend = Some(show_legend);
        Box::new(self)
    }

    pub fn legend_group(mut self, legend_group: &str) -> Box<HeatMap<X, Y, Z>> {
        self.legend_group = Some(legend_group.to_owned());
        Box::new(self)
    }

    pub fn opacity(mut self, opacity: f64) -> Box<HeatMap<X, Y, Z>> {
        self.opacity = Some(opacity);
        Box::new(self)
    }

    pub fn text<S: AsRef<str>>(mut self, text: Vec<S>) -> Box<HeatMap<X, Y, Z>> {
        let text = owned_string_vector(text);
        self.text = Some(text);
        Box::new(self)
    }

    pub fn hover_text<S: AsRef<str>>(mut self, hover_text: Vec<S>) -> Box<HeatMap<X, Y, Z>> {
        let hover_text = owned_string_vector(hover_text);
        self.hover_text = Some(hover_text);
        Box::new(self)
    }

    pub fn hover_info(mut self, hover_info: HoverInfo) -> Box<HeatMap<X, Y, Z>> {
        self.hover_info = Some(hover_info);
        Box::new(self)
    }

    pub fn hover_template(mut self, hover_template: &str) -> Box<HeatMap<X, Y, Z>> {
        self.hover_template = Some(Dim::Scalar(hover_template.to_owned()));
        Box::new(self)
    }

    pub fn hover_template_array<S: AsRef<str>>(
        mut self,
        hover_template: Vec<S>,
    ) -> Box<HeatMap<X, Y, Z>> {
        let hover_template = owned_string_vector(hover_template);
        self.hover_template = Some(Dim::Vector(hover_template));
        Box::new(self)
    }

    pub fn color_bar(mut self, color_bar: ColorBar) -> Box<HeatMap<X, Y, Z>> {
        self.color_bar = Some(color_bar);
        Box::new(self)
    }

    pub fn auto_color_scale(mut self, auto_color_scale: bool) -> Box<HeatMap<X, Y, Z>> {
        self.auto_color_scale = Some(auto_color_scale);
        Box::new(self)
    }

    pub fn color_scale(mut self, color_scale: ColorScale) -> Box<HeatMap<X, Y, Z>> {
        self.color_scale = Some(color_scale);
        Box::new(self)
    }

    pub fn show_scale(mut self, show_scale: bool) -> Box<HeatMap<X, Y, Z>> {
        self.show_scale = Some(show_scale);
        Box::new(self)
    }

    pub fn reverse_scale(mut self, reverse_scale: bool) -> Box<HeatMap<X, Y, Z>> {
        self.reverse_scale = Some(reverse_scale);
        Box::new(self)
    }

    pub fn zauto(mut self, zauto: bool) -> Box<HeatMap<X, Y, Z>> {
        self.zauto = Some(zauto);
        Box::new(self)
    }

    pub fn zhover_format(mut self, zhover_format: &str) -> Box<HeatMap<X, Y, Z>> {
        self.zhover_format = Some(zhover_format.to_owned());
        Box::new(self)
    }

    pub fn zmax(mut self, zmax: Z) -> Box<HeatMap<X, Y, Z>> {
        self.zmax = Some(zmax);
        Box::new(self)
    }

    pub fn zmid(mut self, zmid: Z) -> Box<HeatMap<X, Y, Z>> {
        self.zmid = Some(zmid);
        Box::new(self)
    }

    pub fn zmin(mut self, zmin: Z) -> Box<HeatMap<X, Y, Z>> {
        self.zmin = Some(zmin);
        Box::new(self)
    }

    pub fn zsmooth(mut self, zsmooth: &str) -> Box<HeatMap<X, Y, Z>> {
        self.zsmooth = Some(zsmooth.to_owned());
        Box::new(self)
    }

    pub fn connect_gaps(mut self, connect_gaps: bool) -> Box<HeatMap<X, Y, Z>> {
        self.connect_gaps = Some(connect_gaps);
        Box::new(self)
    }

    pub fn hover_label(mut self, hover_label: Label) -> Box<HeatMap<X, Y, Z>> {
        self.hover_label = Some(hover_label);
        Box::new(self)
    }

    pub fn hover_on_gaps(mut self, hover_on_gaps: bool) -> Box<HeatMap<X, Y, Z>> {
        self.hover_on_gaps = Some(hover_on_gaps);
        Box::new(self)
    }

    pub fn transpose(mut self, transpose: bool) -> Box<HeatMap<X, Y, Z>> {
        self.transpose = Some(transpose);
        Box::new(self)
    }

    pub fn x_calendar(mut self, calendar: Calendar) -> Box<HeatMap<X, Y, Z>> {
        self.x_calendar = Some(calendar);
        Box::new(self)
    }

    pub fn y_calendar(mut self, calendar: Calendar) -> Box<HeatMap<X, Y, Z>> {
        self.y_calendar = Some(calendar);
        Box::new(self)
    }
}

impl<X, Y, Z> Trace for HeatMap<X, Y, Z>
where
    X: num::Num + Serialize,
    Y: num::Num + Serialize,
    Z: num::Num + Serialize,
{
    fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[derive(Serialize, Debug)]
pub struct Lighting {
    #[serde(skip_serializing_if = "Option::is_none")]
    ambient: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    diffuse: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    specular: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    roughness: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    fresnel: Option<f64>,
}

impl Lighting {
    pub fn new() -> Lighting {
        Lighting {
            ambient: None,
            diffuse: None,
            specular: None,
            roughness: None,
            fresnel: None,
        }
    }

    pub fn ambient(mut self, ambient: f64) -> Lighting {
        self.ambient = Some(ambient);
        self
    }

    pub fn diffuse(mut self, diffuse: f64) -> Lighting {
        self.diffuse = Some(diffuse);
        self
    }

    pub fn specular(mut self, specular: f64) -> Lighting {
        self.specular = Some(specular);
        self
    }

    pub fn roughness(mut self, roughness: f64) -> Lighting {
        self.roughness = Some(roughness);
        self
    }

    pub fn fresnel(mut self, fresnel: f64) -> Lighting {
        self.fresnel = Some(fresnel);
        self
    }
}

#[derive(Serialize, Debug)]
pub struct Position {
    x: i32,
    y: i32,
    z: i32,
}

impl Position {
    pub fn new(x: i32, y: i32, z: i32) -> Position {
        Position { x, y, z }
    }
}

#[derive(Serialize, Debug)]
pub struct PlaneProject {
    #[serde(skip_serializing_if = "Option::is_none")]
    x: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    y: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    z: Option<bool>,
}

impl PlaneProject {
    pub fn new() -> PlaneProject {
        PlaneProject {
            x: None,
            y: None,
            z: None,
        }
    }

    pub fn x(mut self, x: bool) -> PlaneProject {
        self.x = Some(x);
        self
    }

    pub fn y(mut self, y: bool) -> PlaneProject {
        self.y = Some(y);
        self
    }

    pub fn z(mut self, z: bool) -> PlaneProject {
        self.z = Some(z);
        self
    }
}

#[derive(Serialize, Debug)]
pub struct PlaneContours {
    #[serde(skip_serializing_if = "Option::is_none")]
    show: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    start: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    end: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    size: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    project: Option<PlaneProject>,
    #[serde(skip_serializing_if = "Option::is_none")]
    color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "usecolormap")]
    use_colormap: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    width: Option<usize>,
    #[serde(skip_serializing_if = "Option::is_none")]
    highlight: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "highlightcolor")]
    highlight_color: Option<Color>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "highlightwidth")]
    highlight_width: Option<usize>,
}

impl PlaneContours {
    pub fn new() -> PlaneContours {
        PlaneContours {
            show: None,
            start: None,
            end: None,
            size: None,
            project: None,
            color: None,
            use_colormap: None,
            width: None,
            highlight: None,
            highlight_color: None,
            highlight_width: None,
        }
    }

    pub fn show(mut self, show: bool) -> PlaneContours {
        self.show = Some(show);
        self
    }

    pub fn start(mut self, start: f64) -> PlaneContours {
        self.start = Some(start);
        self
    }

    pub fn end(mut self, end: f64) -> PlaneContours {
        self.end = Some(end);
        self
    }

    pub fn size(mut self, size: usize) -> PlaneContours {
        self.size = Some(size);
        self
    }

    pub fn project(mut self, project: PlaneProject) -> PlaneContours {
        self.project = Some(project);
        self
    }

    pub fn color(mut self, color: Color) -> PlaneContours {
        self.color = Some(color);
        self
    }

    pub fn use_colormap(mut self, use_colormap: bool) -> PlaneContours {
        self.use_colormap = Some(use_colormap);
        self
    }

    pub fn width(mut self, width: usize) -> PlaneContours {
        self.width = Some(width);
        self
    }

    pub fn highlight(mut self, highlight: bool) -> PlaneContours {
        self.highlight = Some(highlight);
        self
    }

    pub fn highlight_color(mut self, highlight_color: Color) -> PlaneContours {
        self.highlight_color = Some(highlight_color);
        self
    }

    pub fn highlight_width(mut self, highlight_width: usize) -> PlaneContours {
        self.highlight_width = Some(highlight_width);
        self
    }
}

#[derive(Serialize, Debug)]
pub struct SurfaceContours {
    #[serde(skip_serializing_if = "Option::is_none")]
    x: Option<PlaneContours>,
    #[serde(skip_serializing_if = "Option::is_none")]
    y: Option<PlaneContours>,
    #[serde(skip_serializing_if = "Option::is_none")]
    z: Option<PlaneContours>,
}

impl SurfaceContours {
    pub fn new() -> SurfaceContours {
        SurfaceContours {
            x: None,
            y: None,
            z: None,
        }
    }

    pub fn x(mut self, x: PlaneContours) -> SurfaceContours {
        self.x = Some(x);
        self
    }

    pub fn y(mut self, y: PlaneContours) -> SurfaceContours {
        self.y = Some(y);
        self
    }

    pub fn z(mut self, z: PlaneContours) -> SurfaceContours {
        self.z = Some(z);
        self
    }
}

#[derive(Serialize, Debug)]
pub struct Surface<X, Y, Z>
where
    X: Serialize,
    Y: Serialize,
    Z: num::Num + Serialize,
{
    r#type: PlotType,
    #[serde(skip_serializing_if = "Option::is_none")]
    x: Option<Vec<X>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    y: Option<Vec<Y>>,
    z: Vec<Vec<Z>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    name: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    visible: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "showlegend")]
    show_legend: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "legendgroup")]
    legend_group: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    opacity: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "surfacecolor")]
    surface_color: Option<Vec<Color>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<Dim<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hovertext")]
    hover_text: Option<Dim<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverinfo")]
    hover_info: Option<HoverInfo>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hovertemplate")]
    hover_template: Option<Dim<String>>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "colorbar")]
    color_bar: Option<ColorBar>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "autocolorscale")]
    auto_color_scale: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "colorscale")]
    color_scale: Option<ColorScale>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "showscale")]
    show_scale: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "reversescale")]
    reverse_scale: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cauto: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cmin: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cmax: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cmid: Option<f64>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "connectgaps")]
    connect_gaps: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none")]
    contours: Option<SurfaceContours>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hidesurface")]
    hide_surface: Option<bool>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "hoverlabel")]
    hover_label: Option<Label>,
    #[serde(skip_serializing_if = "Option::is_none")]
    lighting: Option<Lighting>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "lightposition")]
    light_position: Option<Position>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "xcalendar")]
    x_calendar: Option<Calendar>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "ycalendar")]
    y_calendar: Option<Calendar>,
    #[serde(skip_serializing_if = "Option::is_none", rename = "zcalendar")]
    z_calendar: Option<Calendar>,
}

impl<X, Y, Z> Surface<X, Y, Z>
where
    X: Serialize,
    Y: Serialize,
    Z: num::Num + Serialize,
{
    pub fn new(z: Vec<Vec<Z>>) -> Box<Surface<X, Y, Z>> {
        Box::new(Surface {
            r#type: PlotType::Surface,
            x: None,
            y: None,
            z,
            name: None,
            visible: None,
            show_legend: None,
            legend_group: None,
            opacity: None,
            surface_color: None,
            text: None,
            hover_text: None,
            hover_info: None,
            hover_template: None,
            color_bar: None,
            auto_color_scale: None,
            color_scale: None,
            show_scale: None,
            reverse_scale: None,
            cauto: None,
            cmin: None,
            cmax: None,
            cmid: None,
            connect_gaps: None,
            contours: None,
            hide_surface: None,
            hover_label: None,
            lighting: None,
            light_position: None,
            x_calendar: None,
            y_calendar: None,
            z_calendar: None,
        })
    }

    pub fn x(mut self, x: Vec<X>) -> Box<Surface<X, Y, Z>> {
        self.x = Some(x);
        Box::new(self)
    }

    pub fn y(mut self, y: Vec<Y>) -> Box<Surface<X, Y, Z>> {
        self.y = Some(y);
        Box::new(self)
    }

    pub fn name(mut self, name: &str) -> Box<Surface<X, Y, Z>> {
        self.name = Some(name.to_owned());
        Box::new(self)
    }

    pub fn visible(mut self, visible: bool) -> Box<Surface<X, Y, Z>> {
        self.visible = Some(visible);
        Box::new(self)
    }

    pub fn show_legend(mut self, show_legend: bool) -> Box<Surface<X, Y, Z>> {
        self.show_legend = Some(show_legend);
        Box::new(self)
    }

    pub fn legend_group(mut self, legend_group: &str) -> Box<Surface<X, Y, Z>> {
        self.legend_group = Some(legend_group.to_owned());
        Box::new(self)
    }

    pub fn opacity(mut self, opacity: f64) -> Box<Surface<X, Y, Z>> {
        self.opacity = Some(opacity);
        Box::new(self)
    }

    pub fn surface_color(mut self, surface_color: Vec<Color>) -> Box<Surface<X, Y, Z>> {
        self.surface_color = Some(surface_color);
        Box::new(self)
    }

    pub fn text(mut self, text: &str) -> Box<Surface<X, Y, Z>> {
        self.text = Some(Dim::Scalar(text.to_owned()));
        Box::new(self)
    }

    pub fn text_array<S: AsRef<str>>(mut self, text: Vec<S>) -> Box<Surface<X, Y, Z>> {
        let text = owned_string_vector(text);
        self.text = Some(Dim::Vector(text));
        Box::new(self)
    }

    pub fn hover_text(mut self, hover_text: &str) -> Box<Surface<X, Y, Z>> {
        self.hover_text = Some(Dim::Scalar(hover_text.to_owned()));
        Box::new(self)
    }

    pub fn hover_text_array<S: AsRef<str>>(mut self, hover_text: Vec<S>) -> Box<Surface<X, Y, Z>> {
        let hover_text = owned_string_vector(hover_text);
        self.hover_text = Some(Dim::Vector(hover_text));
        Box::new(self)
    }

    pub fn hover_info(mut self, hover_info: HoverInfo) -> Box<Surface<X, Y, Z>> {
        self.hover_info = Some(hover_info);
        Box::new(self)
    }

    pub fn hover_template(mut self, hover_template: &str) -> Box<Surface<X, Y, Z>> {
        self.hover_template = Some(Dim::Scalar(hover_template.to_owned()));
        Box::new(self)
    }

    pub fn hover_template_array<S: AsRef<str>>(
        mut self,
        hover_template: Vec<S>,
    ) -> Box<Surface<X, Y, Z>> {
        let hover_template = owned_string_vector(hover_template);
        self.hover_template = Some(Dim::Vector(hover_template));
        Box::new(self)
    }

    pub fn color_bar(mut self, color_bar: ColorBar) -> Box<Surface<X, Y, Z>> {
        self.color_bar = Some(color_bar);
        Box::new(self)
    }

    pub fn auto_color_scale(mut self, auto_color_scale: bool) -> Box<Surface<X, Y, Z>> {
        self.auto_color_scale = Some(auto_color_scale);
        Box::new(self)
    }

    pub fn color_scale(mut self, color_scale: ColorScale) -> Box<Surface<X, Y, Z>> {
        self.color_scale = Some(color_scale);
        Box::new(self)
    }

    pub fn show_scale(mut self, show_scale: bool) -> Box<Surface<X, Y, Z>> {
        self.show_scale = Some(show_scale);
        Box::new(self)
    }

    pub fn reverse_scale(mut self, reverse_scale: bool) -> Box<Surface<X, Y, Z>> {
        self.reverse_scale = Some(reverse_scale);
        Box::new(self)
    }

    pub fn cauto(mut self, cauto: bool) -> Box<Surface<X, Y, Z>> {
        self.cauto = Some(cauto);
        Box::new(self)
    }

    pub fn cmin(mut self, cmin: f64) -> Box<Surface<X, Y, Z>> {
        self.cmin = Some(cmin);
        Box::new(self)
    }

    pub fn cmax(mut self, cmax: f64) -> Box<Surface<X, Y, Z>> {
        self.cmax = Some(cmax);
        Box::new(self)
    }

    pub fn cmid(mut self, cmid: f64) -> Box<Surface<X, Y, Z>> {
        self.cmid = Some(cmid);
        Box::new(self)
    }

    pub fn connect_gaps(mut self, connect_gaps: bool) -> Box<Surface<X, Y, Z>> {
        self.connect_gaps = Some(connect_gaps);
        Box::new(self)
    }

    pub fn contours(mut self, contours: SurfaceContours) -> Box<Surface<X, Y, Z>> {
        self.contours = Some(contours);
        Box::new(self)
    }

    pub fn hide_surface(mut self, hide_surface: bool) -> Box<Surface<X, Y, Z>> {
        self.hide_surface = Some(hide_surface);
        Box::new(self)
    }

    pub fn hover_label(mut self, hover_label: Label) -> Box<Surface<X, Y, Z>> {
        self.hover_label = Some(hover_label);
        Box::new(self)
    }

    pub fn lighting(mut self, lighting: Lighting) -> Box<Surface<X, Y, Z>> {
        self.lighting = Some(lighting);
        Box::new(self)
    }

    pub fn light_position(mut self, light_position: Position) -> Box<Surface<X, Y, Z>> {
        self.light_position = Some(light_position);
        Box::new(self)
    }

    pub fn x_calendar(mut self, x_calendar: Calendar) -> Box<Surface<X, Y, Z>> {
        self.x_calendar = Some(x_calendar);
        Box::new(self)
    }

    pub fn y_calendar(mut self, y_calendar: Calendar) -> Box<Surface<X, Y, Z>> {
        self.y_calendar = Some(y_calendar);
        Box::new(self)
    }

    pub fn z_calendar(mut self, z_calendar: Calendar) -> Box<Surface<X, Y, Z>> {
        self.z_calendar = Some(z_calendar);
        Box::new(self)
    }
}

impl<X, Y, Z> Trace for Surface<X, Y, Z>
where
    X: Serialize,
    Y: Serialize,
    Z: num::Num + Serialize,
{
    fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
