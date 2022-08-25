//! Heat map trace

use serde::Serialize;

use crate::{
    common::{Calendar, ColorBar, ColorScale, Dim, HoverInfo, Label, PlotType, Visible},
    private, Trace,
};

#[derive(Debug, Clone)]
pub enum Smoothing {
    Fast,
    Best,
    False,
}

impl Serialize for Smoothing {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match *self {
            Self::Fast => serializer.serialize_str("fast"),
            Self::Best => serializer.serialize_str("best"),
            Self::False => serializer.serialize_bool(false),
        }
    }
}

/// Construct a heat map trace.
///
/// # Examples
///
/// ```
/// use plotly::HeatMap;
///
/// let trace = HeatMap::new(
///     vec![0.0, 1.0],
///     vec![2.0, 3.0],
///     vec![vec![0.25, 0.75], vec![0.0, 0.5]]
/// );
///
/// let expected = serde_json::json!({
///     "type": "heatmap",
///     "x": [0.0, 1.0],
///     "y": [2.0, 3.0],
///     "z": [[0.25, 0.75], [0.0, 0.5]]
/// });
///
/// assert_eq!(serde_json::to_value(trace).unwrap(), expected);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Serialize, Debug, Clone)]
pub struct HeatMap<X, Y, Z>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
    Z: Serialize + Clone,
{
    r#type: PlotType,
    #[serde(rename = "autocolorscale")]
    auto_color_scale: Option<bool>,
    #[serde(rename = "colorbar")]
    color_bar: Option<ColorBar>,
    #[serde(rename = "colorscale")]
    color_scale: Option<ColorScale>,
    #[serde(rename = "connectgaps")]
    connect_gaps: Option<bool>,
    #[serde(rename = "hoverinfo")]
    hover_info: Option<HoverInfo>,
    #[serde(rename = "hoverlabel")]
    hover_label: Option<Label>,
    #[serde(rename = "hoverongaps")]
    hover_on_gaps: Option<bool>,
    #[serde(rename = "hovertemplate")]
    hover_template: Option<Dim<String>>,
    #[serde(rename = "hovertext")]
    hover_text: Option<Vec<String>>,
    #[serde(rename = "legendgroup")]
    legend_group: Option<String>,
    name: Option<String>,
    opacity: Option<f64>,
    #[serde(rename = "reversescale")]
    reverse_scale: Option<bool>,
    #[serde(rename = "showlegend")]
    show_legend: Option<bool>,
    #[serde(rename = "showscale")]
    show_scale: Option<bool>,
    text: Option<Vec<String>>,
    transpose: Option<bool>,
    visible: Option<Visible>,
    x: Option<Vec<X>>,
    #[serde(rename = "xaxis")]
    x_axis: Option<String>,
    #[serde(rename = "xcalendar")]
    x_calendar: Option<Calendar>,
    y: Option<Vec<Y>>,
    #[serde(rename = "yaxis")]
    y_axis: Option<String>,
    #[serde(rename = "ycalendar")]
    y_calendar: Option<Calendar>,
    z: Option<Vec<Z>>,
    zauto: Option<bool>,
    #[serde(rename = "zhoverformat")]
    zhover_format: Option<String>,
    zmax: Option<Z>,
    zmid: Option<Z>,
    zmin: Option<Z>,
    zsmooth: Option<Smoothing>,
}

impl<X, Y, Z> Default for HeatMap<X, Y, Z>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
    Z: Serialize + Clone,
{
    fn default() -> Self {
        Self {
            r#type: PlotType::HeatMap,
            auto_color_scale: None,
            color_bar: None,
            color_scale: None,
            connect_gaps: None,
            hover_info: None,
            hover_label: None,
            hover_on_gaps: None,
            hover_template: None,
            hover_text: None,
            legend_group: None,
            name: None,
            opacity: None,
            reverse_scale: None,
            show_legend: None,
            show_scale: None,
            text: None,
            transpose: None,
            visible: None,
            x: None,
            x_axis: None,
            x_calendar: None,
            y: None,
            y_axis: None,
            y_calendar: None,
            z: None,
            zauto: None,
            zhover_format: None,
            zmax: None,
            zmid: None,
            zmin: None,
            zsmooth: None,
        }
    }
}

impl<Z> HeatMap<f64, f64, Z>
where
    Z: Serialize + Clone,
{
    pub fn new_z(z: Vec<Z>) -> Box<Self> {
        Box::new(Self {
            z: Some(z),
            ..Default::default()
        })
    }
}

impl<X, Y, Z> HeatMap<X, Y, Z>
where
    X: Serialize + Clone,
    Y: Serialize + Clone,
    Z: Serialize + Clone,
{
    pub fn new(x: Vec<X>, y: Vec<Y>, z: Vec<Z>) -> Box<Self> {
        Box::new(Self {
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

    pub fn name(mut self, name: &str) -> Box<Self> {
        self.name = Some(name.to_string());
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

    pub fn x_axis(mut self, axis: &str) -> Box<Self> {
        self.x_axis = Some(axis.to_string());
        Box::new(self)
    }

    pub fn x_calendar(mut self, calendar: Calendar) -> Box<Self> {
        self.x_calendar = Some(calendar);
        Box::new(self)
    }

    pub fn y_axis(mut self, axis: &str) -> Box<Self> {
        self.y_axis = Some(axis.to_string());
        Box::new(self)
    }

    pub fn y_calendar(mut self, calendar: Calendar) -> Box<Self> {
        self.y_calendar = Some(calendar);
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

    pub fn zsmooth(mut self, zsmooth: Smoothing) -> Box<Self> {
        self.zsmooth = Some(zsmooth);
        Box::new(self)
    }
}

impl<X, Y, Z> Trace for HeatMap<X, Y, Z>
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
    fn test_serialize_smoothing() {
        assert_eq!(to_value(Smoothing::Fast).unwrap(), json!("fast"));
        assert_eq!(to_value(Smoothing::Best).unwrap(), json!("best"));
        assert_eq!(to_value(Smoothing::False).unwrap(), json!(false));
    }

    #[test]
    fn test_serialize_default_heat_map() {
        let trace = HeatMap::<f64, f64, f64>::default();
        let expected = json!({"type": "heatmap"}).to_string();

        assert_eq!(trace.to_json(), expected);
    }

    #[test]
    fn test_serialize_heat_map_z() {
        let trace = HeatMap::new_z(vec![1.0]);
        let expected = json!({
            "type": "heatmap",
            "z": [1.0],
        });

        assert_eq!(to_value(trace).unwrap(), expected);
    }

    #[test]
    fn test_serialize_heat_map() {
        let trace = HeatMap::new(vec![0.0, 1.0], vec![2.0, 3.0], vec![4.0, 5.0])
            .auto_color_scale(true)
            .color_bar(ColorBar::new())
            .color_scale(ColorScale::Palette(ColorScalePalette::Picnic))
            .connect_gaps(false)
            .hover_info(HoverInfo::None)
            .hover_label(Label::new())
            .hover_on_gaps(true)
            .hover_template("tmpl")
            .hover_template_array(vec!["tmpl1", "tmpl2"])
            .hover_text(vec!["hov", "er"])
            .legend_group("1")
            .name("name")
            .opacity(0.99)
            .reverse_scale(false)
            .show_legend(true)
            .show_scale(false)
            .text(vec!["te", "xt"])
            .transpose(true)
            .visible(Visible::LegendOnly)
            .x_axis("x")
            .x_calendar(Calendar::Hebrew)
            .y_axis("y")
            .y_calendar(Calendar::Islamic)
            .zauto(true)
            .zhover_format("fmt")
            .zmax(10.0)
            .zmid(5.0)
            .zmin(0.0)
            .zsmooth(Smoothing::Fast);

        let expected = json!({
            "type": "heatmap",
            "autocolorscale": true,
            "colorbar": {},
            "colorscale": "Picnic",
            "connectgaps": false,
            "hoverinfo": "none",
            "hoverlabel": {},
            "hoverongaps": true,
            "hovertemplate": ["tmpl1", "tmpl2"],
            "hovertext": ["hov", "er"],
            "legendgroup": "1",
            "name": "name",
            "opacity": 0.99,
            "reversescale": false,
            "showlegend": true,
            "showscale": false,
            "text": ["te", "xt"],
            "transpose": true,
            "visible": "legendonly",
            "x": [0.0, 1.0],
            "xcalendar": "hebrew",
            "xaxis": "x",
            "y": [2.0, 3.0],
            "yaxis": "y",
            "ycalendar": "islamic",
            "z": [4.0, 5.0],
            "zauto": true,
            "zhoverformat": "fmt",
            "zmax": 10.0,
            "zmid": 5.0,
            "zmin": 0.0,
            "zsmooth": "fast"
        });

        assert_eq!(to_value(trace).unwrap(), expected);
    }
}
