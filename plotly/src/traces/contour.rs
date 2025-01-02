//! Contour trace

use plotly_derive::FieldSetter;
use serde::Serialize;

use crate::{
    color::Color,
    common::{
        Calendar, ColorBar, ColorScale, Dim, Font, HoverInfo, Label, LegendGroupTitle, Line,
        PlotType, Visible,
    },
    private, Trace,
};

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum ContoursType {
    Levels,
    Constraint,
}

#[derive(Serialize, Debug, Clone)]
#[serde(rename_all = "lowercase")]
pub enum Coloring {
    Fill,
    HeatMap,
    Lines,
    None,
}

#[derive(Serialize, Debug, Clone)]
pub enum Operation {
    #[serde(rename = "=")]
    Equals,
    #[serde(rename = "<")]
    LessThan,
    #[serde(rename = "<=")]
    LessThanOrEqual,
    #[serde(rename = ">")]
    GreaterThan,
    #[serde(rename = ">=")]
    GreaterThanOrEqual,
    #[serde(rename = "[]")]
    Inside,
    #[serde(rename = "][")]
    Outside,
}

#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone, FieldSetter)]
pub struct Contours {
    #[field_setter(skip)]
    r#type: Option<ContoursType>,
    start: Option<f64>,
    end: Option<f64>,
    size: Option<f64>,
    coloring: Option<Coloring>,
    #[serde(rename = "showlines")]
    show_lines: Option<bool>,
    #[serde(rename = "showlabels")]
    show_labels: Option<bool>,
    #[serde(rename = "labelfont")]
    label_font: Option<Font>,
    #[serde(rename = "labelformat")]
    label_format: Option<String>,
    operation: Option<Operation>,
    value: Option<f64>,
}

impl Contours {
    pub fn new() -> Self {
        Default::default()
    }

    pub fn type_(mut self, t: ContoursType) -> Self {
        self.r#type = Some(t);
        self
    }
}

/// Construct a contour trace.
///
/// # Examples
///
/// ```
/// use plotly::Contour;
///
/// let trace = Contour::new(
///     vec![0.0, 2.5, 5.0],
///     vec![0.0, 7.5, 15.0],
///     vec![
///         vec![0.0, 5.0, 10.0],
///         vec![5.0, 2.5, 0.0],
///         vec![10.0, 5.0, 0.0],
///     ],
/// );
///
/// let expected = serde_json::json!({
///     "type": "contour",
///     "x": [0.0, 2.5, 5.0],
///     "y": [0.0, 7.5, 15.0],
///     "z": [[0.0, 5.0, 10.0], [5.0, 2.5, 0.0], [10.0, 5.0, 0.0]]
/// });
///
/// assert_eq!(serde_json::to_value(trace).unwrap(), expected);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone)]
pub struct Contour<Z, X = f64, Y = f64>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
    Z: Serialize + Clone,
{
    r#type: PlotType,
    name: Option<String>,
    visible: Option<Visible>,
    #[serde(rename = "showlegend")]
    show_legend: Option<bool>,
    #[serde(rename = "legendgroup")]
    legend_group: Option<String>,
    #[serde(rename = "legendgrouptitle")]
    legend_group_title: Option<LegendGroupTitle>,
    opacity: Option<f64>,
    x: Option<Vec<X>>,
    x0: Option<X>,
    dx: Option<X>,
    y: Option<Vec<Y>>,
    y0: Option<Y>,
    dy: Option<Y>,
    z: Option<Vec<Z>>,
    text: Option<Vec<String>>,
    #[serde(rename = "hovertext")]
    hover_text: Option<Vec<String>>,
    #[serde(rename = "hoverinfo")]
    hover_info: Option<HoverInfo>,
    #[serde(rename = "hovertemplate")]
    hover_template: Option<Dim<String>>,
    #[serde(rename = "xaxis")]
    x_axis: Option<String>,
    #[serde(rename = "yaxis")]
    y_axis: Option<String>,
    line: Option<Line>,
    #[serde(rename = "colorbar")]
    color_bar: Option<ColorBar>,
    #[serde(rename = "autocolorscale")]
    auto_color_scale: Option<bool>,
    #[serde(rename = "colorscale")]
    color_scale: Option<ColorScale>,
    #[serde(rename = "showscale")]
    show_scale: Option<bool>,
    #[serde(rename = "reversescale")]
    reverse_scale: Option<bool>,
    zauto: Option<bool>,
    #[serde(rename = "zhoverformat")]
    zhover_format: Option<String>,
    zmax: Option<Z>,
    zmid: Option<Z>,
    zmin: Option<Z>,
    #[serde(rename = "autocontour")]
    auto_contour: Option<bool>,
    #[serde(rename = "connectgaps")]
    connect_gaps: Option<bool>,
    contours: Option<Contours>,
    #[serde(rename = "fillcolor")]
    fill_color: Option<Box<dyn Color>>,
    #[serde(rename = "hoverlabel")]
    hover_label: Option<Label>,
    #[serde(rename = "hoverongaps")]
    hover_on_gaps: Option<bool>,
    #[serde(rename = "ncontours")]
    n_contours: Option<usize>,
    transpose: Option<bool>,
    #[serde(rename = "xcalendar")]
    x_calendar: Option<Calendar>,
    #[serde(rename = "ycalendar")]
    y_calendar: Option<Calendar>,
}

impl<Z, X, Y> Default for Contour<Z, X, Y>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
    Z: Serialize + Clone,
{
    fn default() -> Self {
        Self {
            r#type: PlotType::Contour,
            name: None,
            visible: None,
            show_legend: None,
            legend_group: None,
            legend_group_title: None,
            opacity: None,
            x: None,
            x0: None,
            dx: None,
            y: None,
            y0: None,
            dy: None,
            z: None,
            text: None,
            hover_text: None,
            hover_info: None,
            hover_template: None,
            x_axis: None,
            y_axis: None,
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
            auto_contour: None,
            connect_gaps: None,
            contours: None,
            fill_color: None,
            hover_label: None,
            hover_on_gaps: None,
            n_contours: None,
            transpose: None,
            x_calendar: None,
            y_calendar: None,
        }
    }
}

impl<Z> Contour<Z, f64, f64>
where
    Z: Serialize + Clone,
{
    pub fn new_z(z: Vec<Z>) -> Box<Self> {
        Box::new(Contour {
            z: Some(z),
            ..Default::default()
        })
    }
}

impl<Z, X, Y> Contour<Z, X, Y>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
    Z: Serialize + Clone,
{
    pub fn new(x: Vec<X>, y: Vec<Y>, z: Vec<Z>) -> Box<Self> {
        Box::new(Contour {
            x: Some(x),
            y: Some(y),
            z: Some(z),
            ..Default::default()
        })
    }

    pub fn auto_color_scale(mut self, auto_color_scale: bool) -> Box<Self> {
        self.auto_color_scale = Some(auto_color_scale);
        Box::new(self)
    }

    pub fn auto_contour(mut self, auto_contour: bool) -> Box<Self> {
        self.auto_contour = Some(auto_contour);
        Box::new(self)
    }

    pub fn color_bar(mut self, color_bar: ColorBar) -> Box<Self> {
        self.color_bar = Some(color_bar);
        Box::new(self)
    }

    pub fn color_scale(mut self, color_scale: ColorScale) -> Box<Self> {
        self.color_scale = Some(color_scale);
        Box::new(self)
    }

    pub fn connect_gaps(mut self, connect_gaps: bool) -> Box<Self> {
        self.connect_gaps = Some(connect_gaps);
        Box::new(self)
    }

    pub fn contours(mut self, contours: Contours) -> Box<Self> {
        self.contours = Some(contours);
        Box::new(self)
    }

    pub fn dx(mut self, dx: X) -> Box<Self> {
        self.dx = Some(dx);
        Box::new(self)
    }

    pub fn dy(mut self, dy: Y) -> Box<Self> {
        self.dy = Some(dy);
        Box::new(self)
    }

    pub fn fill_color<C: Color>(mut self, fill_color: C) -> Box<Self> {
        self.fill_color = Some(Box::new(fill_color));
        Box::new(self)
    }

    pub fn hover_info(mut self, hover_info: HoverInfo) -> Box<Self> {
        self.hover_info = Some(hover_info);
        Box::new(self)
    }

    pub fn hover_label(mut self, hover_label: Label) -> Box<Self> {
        self.hover_label = Some(hover_label);
        Box::new(self)
    }

    pub fn hover_on_gaps(mut self, hover_on_gaps: bool) -> Box<Self> {
        self.hover_on_gaps = Some(hover_on_gaps);
        Box::new(self)
    }

    pub fn hover_template(mut self, hover_template: &str) -> Box<Self> {
        self.hover_template = Some(Dim::Scalar(hover_template.to_string()));
        Box::new(self)
    }

    pub fn hover_template_array<S: AsRef<str>>(mut self, hover_template: Vec<S>) -> Box<Self> {
        let hover_template = private::owned_string_vector(hover_template);
        self.hover_template = Some(Dim::Vector(hover_template));
        Box::new(self)
    }

    pub fn hover_text<S: AsRef<str>>(mut self, hover_text: Vec<S>) -> Box<Self> {
        let hover_text = private::owned_string_vector(hover_text);
        self.hover_text = Some(hover_text);
        Box::new(self)
    }

    pub fn legend_group(mut self, legend_group: &str) -> Box<Self> {
        self.legend_group = Some(legend_group.to_string());
        Box::new(self)
    }

    pub fn legend_group_title(
        mut self,
        legend_group_title: impl Into<LegendGroupTitle>,
    ) -> Box<Self> {
        self.legend_group_title = Some(legend_group_title.into());
        Box::new(self)
    }

    pub fn line(mut self, line: Line) -> Box<Self> {
        self.line = Some(line);
        Box::new(self)
    }

    pub fn name(mut self, name: &str) -> Box<Self> {
        self.name = Some(name.to_string());
        Box::new(self)
    }

    pub fn n_contours(mut self, n_contours: usize) -> Box<Self> {
        self.n_contours = Some(n_contours);
        Box::new(self)
    }

    pub fn opacity(mut self, opacity: f64) -> Box<Self> {
        self.opacity = Some(opacity);
        Box::new(self)
    }

    pub fn reverse_scale(mut self, reverse_scale: bool) -> Box<Self> {
        self.reverse_scale = Some(reverse_scale);
        Box::new(self)
    }

    pub fn show_legend(mut self, show_legend: bool) -> Box<Self> {
        self.show_legend = Some(show_legend);
        Box::new(self)
    }

    pub fn show_scale(mut self, show_scale: bool) -> Box<Self> {
        self.show_scale = Some(show_scale);
        Box::new(self)
    }

    pub fn text<S: AsRef<str>>(mut self, text: Vec<S>) -> Box<Self> {
        let text = private::owned_string_vector(text);
        self.text = Some(text);
        Box::new(self)
    }

    pub fn transpose(mut self, transpose: bool) -> Box<Self> {
        self.transpose = Some(transpose);
        Box::new(self)
    }

    pub fn visible(mut self, visible: Visible) -> Box<Self> {
        self.visible = Some(visible);
        Box::new(self)
    }

    pub fn x(mut self, x: Vec<X>) -> Box<Self> {
        self.x = Some(x);
        Box::new(self)
    }

    pub fn x_axis(mut self, axis: &str) -> Box<Self> {
        self.x_axis = Some(axis.to_string());
        Box::new(self)
    }

    pub fn x_calendar(mut self, x_calendar: Calendar) -> Box<Self> {
        self.x_calendar = Some(x_calendar);
        Box::new(self)
    }

    pub fn x0(mut self, x0: X) -> Box<Self> {
        self.x0 = Some(x0);
        Box::new(self)
    }

    pub fn y_axis(mut self, axis: &str) -> Box<Self> {
        self.y_axis = Some(axis.to_string());
        Box::new(self)
    }

    pub fn y(mut self, y: Vec<Y>) -> Box<Self> {
        self.y = Some(y);
        Box::new(self)
    }

    pub fn y_calendar(mut self, y_calendar: Calendar) -> Box<Self> {
        self.y_calendar = Some(y_calendar);
        Box::new(self)
    }

    pub fn y0(mut self, y0: Y) -> Box<Self> {
        self.y0 = Some(y0);
        Box::new(self)
    }

    pub fn zauto(mut self, zauto: bool) -> Box<Self> {
        self.zauto = Some(zauto);
        Box::new(self)
    }

    pub fn zhover_format(mut self, zhover_format: &str) -> Box<Self> {
        self.zhover_format = Some(zhover_format.to_string());
        Box::new(self)
    }

    pub fn zmax(mut self, zmax: Z) -> Box<Self> {
        self.zmax = Some(zmax);
        Box::new(self)
    }

    pub fn zmid(mut self, zmid: Z) -> Box<Self> {
        self.zmid = Some(zmid);
        Box::new(self)
    }

    pub fn zmin(mut self, zmin: Z) -> Box<Self> {
        self.zmin = Some(zmin);
        Box::new(self)
    }
}

impl<X, Y, Z> Trace for Contour<X, Y, Z>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
    Z: Serialize + Clone,
{
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};

    use super::*;
    use crate::common::ColorScalePalette;

    #[test]
    #[rustfmt::skip]
    fn serialize_contours_type() {
        assert_eq!(to_value(ContoursType::Levels).unwrap(), json!("levels"));
        assert_eq!(to_value(ContoursType::Constraint).unwrap(), json!("constraint"));
    }

    #[test]
    fn serialize_coloring() {
        assert_eq!(to_value(Coloring::Fill).unwrap(), json!("fill"));
        assert_eq!(to_value(Coloring::HeatMap).unwrap(), json!("heatmap"));
        assert_eq!(to_value(Coloring::Lines).unwrap(), json!("lines"));
        assert_eq!(to_value(Coloring::None).unwrap(), json!("none"));
    }

    #[test]
    #[rustfmt::skip]
    fn serialize_operation() {
        assert_eq!(to_value(Operation::Equals).unwrap(), json!("="));
        assert_eq!(to_value(Operation::LessThan).unwrap(), json!("<"));
        assert_eq!(to_value(Operation::LessThanOrEqual).unwrap(), json!("<="));
        assert_eq!(to_value(Operation::GreaterThan).unwrap(), json!(">"));
        assert_eq!(to_value(Operation::GreaterThanOrEqual).unwrap(), json!(">="));
        assert_eq!(to_value(Operation::Inside).unwrap(), json!("[]"));
        assert_eq!(to_value(Operation::Outside).unwrap(), json!("]["));
    }

    #[test]
    fn serialize_default_contours() {
        let contours = Contours::new();
        let expected = json!({});

        assert_eq!(to_value(contours).unwrap(), expected);
    }
    #[test]
    fn serialize_contours() {
        let contours = Contours::new()
            .type_(ContoursType::Levels)
            .start(0.0)
            .end(10.0)
            .size(5.0)
            .coloring(Coloring::HeatMap)
            .show_lines(true)
            .show_labels(false)
            .label_font(Font::new())
            .label_format("fmt")
            .operation(Operation::GreaterThan)
            .value(6.0);

        let expected = json!({
            "type": "levels",
            "start": 0.0,
            "end": 10.0,
            "size": 5.0,
            "coloring": "heatmap",
            "showlines": true,
            "showlabels": false,
            "labelformat": "fmt",
            "labelfont": {},
            "operation": ">",
            "value": 6.0
        });

        assert_eq!(to_value(contours).unwrap(), expected)
    }

    #[test]
    fn serialize_default_contour() {
        let trace: Contour<f64, f64, f64> = Contour::default();
        let expected = json!({"type": "contour"}).to_string();

        assert_eq!(trace.to_json(), expected);
    }

    #[test]
    fn new_z_contour() {
        let trace = Contour::new_z(vec![1.0]);
        let expected = json!({
            "type": "contour",
            "z": [1.0]
        });

        assert_eq!(to_value(trace).unwrap(), expected);
    }

    #[test]
    fn serialize_contour() {
        let trace = Contour::new(vec![0., 1.], vec![2., 3.], vec![4., 5.])
            .auto_color_scale(true)
            .auto_contour(true)
            .color_bar(ColorBar::new())
            .color_scale(ColorScale::Palette(ColorScalePalette::Blackbody))
            .connect_gaps(true)
            .contours(Contours::new())
            .dx(1.)
            .dy(1.)
            .fill_color("#123456")
            .hover_info(HoverInfo::XAndYAndZ)
            .hover_label(Label::new())
            .hover_on_gaps(false)
            .hover_template("ok {1}")
            .hover_template_array(vec!["ok {1}", "ok {2}"])
            .hover_text(vec!["p3", "p4"])
            .legend_group("group_1")
            .legend_group_title("Legend Group Title")
            .line(Line::new())
            .n_contours(5)
            .name("contour trace")
            .opacity(0.6)
            .reverse_scale(true)
            .show_scale(true)
            .show_legend(false)
            .text(vec!["p1", "p2"])
            .transpose(true)
            .visible(Visible::True)
            .x(vec![0.0, 1.0])
            .x_axis("x0")
            .x_calendar(Calendar::Ethiopian)
            .x0(0.)
            .y(vec![2.0, 3.0])
            .y_axis("y0")
            .y_calendar(Calendar::Gregorian)
            .y0(0.)
            .zauto(false)
            .zhover_format("fmt")
            .zmax(10.)
            .zmid(5.)
            .zmin(0.);

        let expected = json!({
            "type": "contour",
            "x": [0.0, 1.0],
            "y": [2.0, 3.0],
            "z": [4.0, 5.0],
            "x0": 0.0,
            "dx": 1.0,
            "y0": 0.0,
            "dy": 1.0,
            "name": "contour trace",
            "visible": true,
            "showlegend": false,
            "legendgroup": "group_1",
            "legendgrouptitle": {"text": "Legend Group Title"},
            "opacity": 0.6,
            "text": ["p1", "p2"],
            "hovertext": ["p3", "p4"],
            "hoverinfo": "x+y+z",
            "hovertemplate": ["ok {1}", "ok {2}"],
            "xaxis": "x0",
            "yaxis": "y0",
            "line": {},
            "colorbar": {},
            "autocolorscale": true,
            "colorscale": "Blackbody",
            "showscale": true,
            "reversescale": true,
            "zauto": false,
            "zhoverformat": "fmt",
            "zmax": 10.0,
            "zmid": 5.0,
            "zmin": 0.0,
            "autocontour": true,
            "connectgaps": true,
            "contours": {},
            "fillcolor": "#123456",
            "hoverlabel": {},
            "hoverongaps": false,
            "ncontours": 5,
            "transpose": true,
            "xcalendar": "ethiopian",
            "ycalendar": "gregorian"
        });

        assert_eq!(to_value(trace).unwrap(), expected);
    }
}
