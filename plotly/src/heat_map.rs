//! Heat-map plot

use crate::common::{Calendar, ColorBar, ColorScale, Dim, HoverInfo, Label, PlotType};
use crate::private;
use crate::Trace;
use serde::Serialize;

#[derive(Serialize, Debug)]
pub struct HeatMap<Z, X, Y>
where
    X: Serialize,
    Y: Serialize,
    Z: Serialize,
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
    x: Option<Vec<X>>,
    y: Option<Vec<Y>>,
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

impl<Z> HeatMap<Z, f64, f64>
where
    Z: Serialize,
{
    pub fn new_z(z: Vec<Z>) -> HeatMap<Z, f64, f64> {
        HeatMap {
            x: None,
            y: None,
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
        }
    }
}

impl<X, Y, Z> HeatMap<Z, X, Y>
where
    X: Serialize,
    Y: Serialize,
    Z: Serialize,
{
    pub fn new(x: Vec<X>, y: Vec<Y>, z: Vec<Z>) -> HeatMap<Z, X, Y> {
        HeatMap {
            x: Some(x),
            y: Some(y),
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
        }
    }

    pub fn name(mut self, name: &str) -> HeatMap<Z, X, Y> {
        self.name = Some(name.to_owned());
        self
    }

    pub fn visible(mut self, visible: bool) -> HeatMap<Z, X, Y> {
        self.visible = Some(visible);
        self
    }

    pub fn show_legend(mut self, show_legend: bool) -> HeatMap<Z, X, Y> {
        self.show_legend = Some(show_legend);
        self
    }

    pub fn legend_group(mut self, legend_group: &str) -> HeatMap<Z, X, Y> {
        self.legend_group = Some(legend_group.to_owned());
        self
    }

    pub fn opacity(mut self, opacity: f64) -> HeatMap<Z, X, Y> {
        self.opacity = Some(opacity);
        self
    }

    pub fn text<S: AsRef<str>>(mut self, text: Vec<S>) -> HeatMap<Z, X, Y> {
        let text = private::owned_string_vector(text);
        self.text = Some(text);
        self
    }

    pub fn hover_text<S: AsRef<str>>(mut self, hover_text: Vec<S>) -> HeatMap<Z, X, Y> {
        let hover_text = private::owned_string_vector(hover_text);
        self.hover_text = Some(hover_text);
        self
    }

    pub fn hover_info(mut self, hover_info: HoverInfo) -> HeatMap<Z, X, Y> {
        self.hover_info = Some(hover_info);
        self
    }

    pub fn hover_template(mut self, hover_template: &str) -> HeatMap<Z, X, Y> {
        self.hover_template = Some(Dim::Scalar(hover_template.to_owned()));
        self
    }

    pub fn hover_template_array<S: AsRef<str>>(
        mut self,
        hover_template: Vec<S>,
    ) -> HeatMap<Z, X, Y> {
        let hover_template = private::owned_string_vector(hover_template);
        self.hover_template = Some(Dim::Vector(hover_template));
        self
    }

    pub fn color_bar(mut self, color_bar: ColorBar) -> HeatMap<Z, X, Y> {
        self.color_bar = Some(color_bar);
        self
    }

    pub fn auto_color_scale(mut self, auto_color_scale: bool) -> HeatMap<Z, X, Y> {
        self.auto_color_scale = Some(auto_color_scale);
        self
    }

    pub fn color_scale(mut self, color_scale: ColorScale) -> HeatMap<Z, X, Y> {
        self.color_scale = Some(color_scale);
        self
    }

    pub fn show_scale(mut self, show_scale: bool) -> HeatMap<Z, X, Y> {
        self.show_scale = Some(show_scale);
        self
    }

    pub fn reverse_scale(mut self, reverse_scale: bool) -> HeatMap<Z, X, Y> {
        self.reverse_scale = Some(reverse_scale);
        self
    }

    pub fn zauto(mut self, zauto: bool) -> HeatMap<Z, X, Y> {
        self.zauto = Some(zauto);
        self
    }

    pub fn zhover_format(mut self, zhover_format: &str) -> HeatMap<Z, X, Y> {
        self.zhover_format = Some(zhover_format.to_owned());
        self
    }

    pub fn zmax(mut self, zmax: Z) -> HeatMap<Z, X, Y> {
        self.zmax = Some(zmax);
        self
    }

    pub fn zmid(mut self, zmid: Z) -> HeatMap<Z, X, Y> {
        self.zmid = Some(zmid);
        self
    }

    pub fn zmin(mut self, zmin: Z) -> HeatMap<Z, X, Y> {
        self.zmin = Some(zmin);
        self
    }

    pub fn zsmooth(mut self, zsmooth: &str) -> HeatMap<Z, X, Y> {
        self.zsmooth = Some(zsmooth.to_owned());
        self
    }

    pub fn connect_gaps(mut self, connect_gaps: bool) -> HeatMap<Z, X, Y> {
        self.connect_gaps = Some(connect_gaps);
        self
    }

    pub fn hover_label(mut self, hover_label: Label) -> HeatMap<Z, X, Y> {
        self.hover_label = Some(hover_label);
        self
    }

    pub fn hover_on_gaps(mut self, hover_on_gaps: bool) -> HeatMap<Z, X, Y> {
        self.hover_on_gaps = Some(hover_on_gaps);
        self
    }

    pub fn transpose(mut self, transpose: bool) -> HeatMap<Z, X, Y> {
        self.transpose = Some(transpose);
        self
    }

    pub fn x_calendar(mut self, calendar: Calendar) -> HeatMap<Z, X, Y> {
        self.x_calendar = Some(calendar);
        self
    }

    pub fn y_calendar(mut self, calendar: Calendar) -> HeatMap<Z, X, Y> {
        self.y_calendar = Some(calendar);
        self
    }
}

impl<X, Y, Z> Trace for HeatMap<Z, X, Y>
where
    X: Serialize,
    Y: Serialize,
    Z: Serialize,
{
    fn serialize(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}
