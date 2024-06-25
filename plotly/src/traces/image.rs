//! Image plot

#[cfg(feature = "plotly_image")]
use image::{Pixel, RgbImage, RgbaImage};
#[cfg(feature = "plotly_ndarray")]
use ndarray::{Array, Ix2};
use plotly_derive::FieldSetter;
use serde::Serialize;

use crate::color::{Rgb, Rgba};
use crate::common::{Dim, HoverInfo, Label, LegendGroupTitle, PlotType, Visible};
use crate::private::{NumOrString, NumOrStringCollection};
use crate::Trace;

#[derive(Serialize, Clone, Copy, Debug)]
#[serde(untagged)]
pub enum PixelColor {
    Color3(u8, u8, u8),
    Color4(u8, u8, u8, f64),
}

/// A marker trait allowing several ways to describe the pixel data for an
/// `Image` trace.
pub trait ImageData {
    fn to_image_data(&self) -> Vec<Vec<PixelColor>>;
}

impl ImageData for Vec<Vec<Rgb>> {
    fn to_image_data(&self) -> Vec<Vec<PixelColor>> {
        let mut out = Vec::with_capacity(self.len());
        for row in self {
            let mut new_row = Vec::with_capacity(row.len());
            for pixel in row {
                new_row.push(PixelColor::Color3(pixel.r, pixel.g, pixel.b))
            }
            out.push(new_row);
        }
        out
    }
}

impl ImageData for Vec<Vec<Rgba>> {
    fn to_image_data(&self) -> Vec<Vec<PixelColor>> {
        let mut out = Vec::with_capacity(self.len());
        for row in self {
            let mut new_row = Vec::with_capacity(row.len());
            for pixel in row {
                new_row.push(PixelColor::Color4(pixel.r, pixel.g, pixel.b, pixel.a))
            }
            out.push(new_row);
        }
        out
    }
}

#[cfg(feature = "plotly_image")]
impl ImageData for RgbImage {
    fn to_image_data(&self) -> Vec<Vec<PixelColor>> {
        let mut out = Vec::with_capacity(self.height() as usize);
        for row in self.rows() {
            let mut new_row = Vec::with_capacity(row.len());
            for pixel in row {
                let ch = pixel.channels();
                new_row.push(PixelColor::Color3(ch[0], ch[1], ch[2]))
            }
            out.push(new_row);
        }
        out
    }
}

#[cfg(feature = "plotly_image")]
impl ImageData for RgbaImage {
    fn to_image_data(&self) -> Vec<Vec<PixelColor>> {
        let mut out = Vec::with_capacity(self.height() as usize);
        for row in self.rows() {
            let mut new_row = Vec::with_capacity(row.len());
            for pixel in row {
                let ch = pixel.channels();
                new_row.push(PixelColor::Color4(
                    ch[0],
                    ch[1],
                    ch[2],
                    ch[3] as f64 / 255.0,
                ))
            }
            out.push(new_row);
        }
        out
    }
}

#[cfg(feature = "plotly_ndarray")]
impl ImageData for Array<(u8, u8, u8), Ix2> {
    fn to_image_data(&self) -> Vec<Vec<PixelColor>> {
        let height = self.shape()[0];
        let mut out = Vec::with_capacity(height);
        let pixels = self.map(|p| PixelColor::Color3(p.0, p.1, p.2));
        for row in pixels.rows() {
            out.push(row.to_vec());
        }
        out
    }
}

#[cfg(feature = "plotly_ndarray")]
impl ImageData for Array<(u8, u8, u8, f64), Ix2> {
    fn to_image_data(&self) -> Vec<Vec<PixelColor>> {
        let height = self.shape()[0];
        let mut out = Vec::with_capacity(height);
        let pixels = self.map(|p| PixelColor::Color4(p.0, p.1, p.2, p.3));
        for row in pixels.rows() {
            out.push(row.to_vec());
        }
        out
    }
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum ColorModel {
    RGB,
    RGBA,
    RGBA256,
    HSL,
    HSLA,
}

#[derive(Clone, Debug)]
pub enum ZSmooth {
    Fast,
    False,
}

impl Serialize for ZSmooth {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        match self {
            Self::Fast => serializer.serialize_str("fast"),
            Self::False => serializer.serialize_bool(false),
        }
    }
}

/// Construct an image trace.
///
/// # Examples
///
/// ```
/// use plotly::{color::Rgb, image::ColorModel, Image};
///
/// let b = Rgb::new(0, 0, 0);
/// let w = Rgb::new(255, 255, 255);
///
/// let z = vec![
///     vec![b, w],
///     vec![w, b],
/// ];
///
/// let trace = Image::new(z).color_model(ColorModel::RGB);
///
/// let expected = serde_json::json!({
///     "type": "image",
///     "z": [[[0, 0, 0], [255, 255, 255]], [[255, 255, 255], [0, 0, 0]]],
///     "colormodel": "rgb"
/// });
///
/// assert_eq!(serde_json::to_value(trace).unwrap(), expected);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
#[field_setter(box_self, kind = "trace")]
pub struct Image {
    #[field_setter(default = "PlotType::Image")]
    r#type: PlotType,

    #[field_setter(skip)]
    z: Option<Vec<Vec<PixelColor>>>,

    /// Sets the trace name. The trace name appear as the legend item and on
    /// hover.
    name: Option<String>,
    /// Determines whether or not this trace is visible. If
    /// `Visible::LegendOnly`, the trace is not drawn, but can appear as a
    /// legend item (provided that the legend itself is visible).
    visible: Option<Visible>,

    /// Sets the legend rank for this trace. Items and groups with smaller ranks
    /// are presented on top/left side while with `"reversed"
    /// `legend.trace_order` they are on bottom/right side. The default
    /// legendrank is 1000, so that you can use ranks less than 1000 to
    /// place certain items before all unranked items, and ranks greater
    /// than 1000 to go after all unranked items.
    #[serde(rename = "legendrank")]
    legend_rank: Option<usize>,
    /// Set and style the title to appear for the legend group.
    #[serde(rename = "legendgrouptitle")]
    legend_group_title: Option<LegendGroupTitle>,

    /// Sets the opacity of the trace.
    opacity: Option<f64>,
    /// Assigns id labels to each datum. These ids for object constancy of data
    /// points during animation. Should be an array of strings, not numbers
    /// or any other type.
    ids: Option<Vec<String>>,

    /// Set the image's x position.
    x0: Option<NumOrString>,
    /// Set the pixel's horizontal size.
    dx: Option<f64>,

    /// Set the image's y position.
    y0: Option<NumOrString>,
    /// Set the pixel's vertical size.
    dy: Option<f64>,

    /// Specifies the data URI of the image to be visualized. The URI consists
    /// of "data:image/[<media subtype>][;base64],<data>".
    source: Option<String>,

    /// Sets text elements associated with each (x,y) pair. If a single string,
    /// the same string appears over all the data points. If an array of
    /// strings the items are mapped in order to the this trace's (x,y)
    /// coordinates. If the trace `HoverInfo` contains a "text" flag and
    /// `hover_text` is not set, these elements will be seen in the hover
    /// labels.
    text: Option<Dim<String>>,
    /// Sets hover text elements associated with each (x,y) pair. If a single
    /// string, the same string appears over all the data points. If an
    /// array of strings, the items are mapped in order to the this trace's
    /// (x,y) coordinates. To be seen, trace `HoverInfo` must contain a
    /// "Text" flag.
    #[serde(rename = "hovertext")]
    hover_text: Option<Dim<String>>,
    /// Determines which trace information appear on hover. If `HoverInfo::None`
    /// or `HoverInfo::Skip` are set, no information is displayed upon
    /// hovering. But, if `HoverInfo::None` is set, click and hover events
    /// are still fired.
    #[serde(rename = "hoverinfo")]
    hover_info: Option<HoverInfo>,
    /// Template string used for rendering the information that appear on hover
    /// box. Note that this will override `HoverInfo`. Variables are
    /// inserted using %{variable}, for example "y: %{y}". Numbers are
    /// formatted using d3-format's syntax %{variable:d3-format}, for example
    /// "Price: %{y:$.2f}".
    /// https://github.com/d3/d3-3.x-api-reference/blob/master/Formatting.md#d3_format for details
    /// on the formatting syntax. Dates are formatted using d3-time-format's
    /// syntax %{variable|d3-time-format}, for example "Day:
    /// %{2019-01-01|%A}". https://github.com/d3/d3-3.x-api-reference/blob/master/Time-Formatting.md#format for details
    /// on the date formatting syntax. The variables available in
    /// `hovertemplate` are the ones emitted as event data described at this link https://plotly.com/javascript/plotlyjs-events/#event-data.
    /// Additionally, every attributes that can be specified per-point (the ones
    /// that are `arrayOk: true`) are available. Anything contained in tag
    /// `<extra>` is displayed in the secondary box, for example
    /// "<extra>{fullData.name}</extra>". To hide the secondary box
    /// completely, use an empty tag `<extra></extra>`.
    #[serde(rename = "hovertemplate")]
    hover_template: Option<Dim<String>>,

    /// Assigns extra meta information associated with this trace that can be
    /// used in various text attributes. Attributes such as trace `name`,
    /// graph, axis and colorbar `title.text`, annotation `text`
    /// `rangeselector`, `updatemenues` and `sliders` `label` text all support
    /// `meta`. To access the trace `meta` values in an attribute in the same
    /// trace, simply use `%{meta[i]}` where `i` is the index or key of the
    /// `meta` item in question. To access trace `meta` in layout
    /// attributes, use `%{data[n[.meta[i]}` where `i` is the index or key of
    /// the `meta` and `n` is the trace index.
    meta: Option<NumOrString>,
    /// Assigns extra data each datum. This may be useful when listening to
    /// hover, click and selection events. Note that, "scatter" traces also
    /// appends customdata items in the markers DOM elements
    #[serde(rename = "customdata")]
    custom_data: Option<NumOrStringCollection>,

    /// Sets a reference between this trace's x coordinates and a 2D cartesian x
    /// axis. If "x" ( the default value), the x coordinates refer to
    /// `Layout::x_axis`. If "x2", the x coordinates
    /// refer to `Layout::x_axis2`, and so on.
    #[serde(rename = "xaxis")]
    x_axis: Option<String>,
    /// Sets a reference between this trace's y coordinates and a 2D cartesian y
    /// axis. If "y" (the default value), the y coordinates refer to
    /// `Layout::y_axis`. If "y2", the y coordinates
    /// refer to `Layout::y_axis2`, and so on.
    #[serde(rename = "yaxis")]
    y_axis: Option<String>,

    /// Color model used to map the numerical color components described in `z`
    /// into colors. If `source` is specified, this attribute will be set to
    /// `rgba256` otherwise it defaults to `rgb`.
    #[serde(rename = "colormodel")]
    color_model: Option<ColorModel>,

    #[field_setter(skip)]
    #[serde(rename = "zmax")]
    z_max: Option<Vec<Vec<PixelColor>>>,

    #[field_setter(skip)]
    #[serde(rename = "zmin")]
    z_min: Option<Vec<Vec<PixelColor>>>,

    /// Picks a smoothing algorithm used to smooth `z` data. This only applies
    /// for image traces that use the `source` attribute.
    #[serde(rename = "zsmooth")]
    z_smooth: Option<ZSmooth>,

    /// Properties of label displayed on mouse hover.
    #[serde(rename = "hoverlabel")]
    hover_label: Option<Label>,

    /// Controls persistence of some user-driven changes to the trace:
    /// `constraintrange` in `parcoords` traces, as well as some `editable:
    /// True` modifications such as `name` and `colorbar.title`. Defaults to
    /// `layout.uirevision`. Note that other user-driven trace attribute changes
    /// are controlled by `layout` attributes: `trace.visible` is controlled
    /// by `layout.legend.uirevision`, `selectedpoints` is controlled
    /// by `layout.selectionrevision`, and `colorbar.(x|y)` (accessible with
    /// `config: {editable: True}`) is controlled by `layout.editrevision`.
    /// Trace changes are tracked by `uid`, which only falls back on trace
    /// index if no `uid` is provided. So if your app can add/remove traces
    /// before the end of the `data` array, such that the same trace has a
    /// different index, you can still preserve user-driven changes if you give
    /// each trace a `uid` that stays with it as it moves.
    #[serde(rename = "uirevision")]
    ui_revision: Option<NumOrString>,
}

impl Image {
    /// A 2-dimensional array in which each element is an array of 3 or 4
    /// numbers representing a color.
    pub fn new(z: impl ImageData) -> Box<Self> {
        Box::new(Self {
            z: Some(z.to_image_data()),
            ..Default::default()
        })
    }

    /// Array defining the higher bound for each color component. Note that the
    /// default value will depend on the colormodel. For the `rgb`
    /// colormodel, it is [255, 255, 255]. For the `rgba` colormodel, it is
    /// [255, 255, 255, 1]. For the `rgba256` colormodel, it is [255, 255, 255,
    /// 255]. For the `hsl` colormodel, it is [360, 100, 100]. For the
    /// `hsla` colormodel, it is [360, 100, 100, 1].
    pub fn z_max(mut self, z_max: impl ImageData) -> Box<Self> {
        self.z_max = Some(z_max.to_image_data());
        Box::new(self)
    }

    /// Array defining the lower bound for each color component. Note that the
    /// default value will depend on the colormodel. For the `rgb`
    /// colormodel, it is [0, 0, 0]. For the `rgba` colormodel, it is [0, 0, 0,
    /// 0]. For the `rgba256` colormodel, it is [0, 0, 0, 0]. For the `hsl`
    /// colormodel, it is [0, 0, 0]. For the `hsla` colormodel, it is [0, 0,
    /// 0, 0].
    pub fn z_min(mut self, z_min: impl ImageData) -> Box<Self> {
        self.z_min = Some(z_min.to_image_data());
        Box::new(self)
    }
}

impl Trace for Image {
    fn to_json(&self) -> String {
        serde_json::to_string(&self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};

    use super::*;

    #[test]
    fn test_serialize_pixel_color() {
        assert_eq!(
            to_value(PixelColor::Color3(255, 100, 150)).unwrap(),
            json!([255, 100, 150])
        );
        assert_eq!(
            to_value(PixelColor::Color4(150, 140, 190, 0.5)).unwrap(),
            json!([150, 140, 190, 0.5])
        );
    }

    #[test]
    fn test_serialize_color_model() {
        assert_eq!(to_value(ColorModel::RGB).unwrap(), json!("rgb"));
        assert_eq!(to_value(ColorModel::RGBA).unwrap(), json!("rgba"));
        assert_eq!(to_value(ColorModel::RGBA256).unwrap(), json!("rgba256"));
        assert_eq!(to_value(ColorModel::HSL).unwrap(), json!("hsl"));
        assert_eq!(to_value(ColorModel::HSLA).unwrap(), json!("hsla"));
    }

    #[test]
    fn test_serialize_z_smooth() {
        assert_eq!(to_value(ZSmooth::Fast).unwrap(), json!("fast"));
        assert_eq!(to_value(ZSmooth::False).unwrap(), json!(false));
    }

    #[test]
    fn test_serialize_image() {
        let b = Rgba::new(0, 0, 0, 0.5);
        let w = Rgba::new(255, 255, 255, 1.0);
        let image = Image::new(vec![vec![b, w, b, w, b], vec![w, b, w, b, w]])
            .name("image name")
            .visible(Visible::True)
            .legend_rank(1000)
            .legend_group_title("Legend Group Title")
            .opacity(0.5)
            .ids(vec!["one"])
            .x0(0.0)
            .dx(1.0)
            .y0(2.0)
            .dy(3.0)
            .source("https://raw.githubusercontent.com/michaelbabyn/plot_data/master/bridge.jpg")
            .text("text")
            .text_array(vec!["text"])
            .hover_text("hover_text")
            .hover_text_array(vec!["hover_text"])
            .hover_info(HoverInfo::XAndYAndZ)
            .hover_template("hover_template")
            .hover_template_array(vec!["hover_template"])
            .meta("meta")
            .custom_data(vec!["custom_data"])
            .x_axis("x2")
            .y_axis("y2")
            .color_model(ColorModel::RGBA)
            .z_max(vec![vec![w, w, w, w, w], vec![w, w, w, w, w]])
            .z_min(vec![vec![b, b, b, b, b], vec![b, b, b, b, b]])
            .z_smooth(ZSmooth::Fast)
            .hover_label(Label::new())
            .ui_revision(6);

        let b = (0, 0, 0, 0.5);
        let w = (255, 255, 255, 1.0);
        let expected = json!({
            "type": "image",
            "z": [[b, w, b, w, b], [w, b, w, b, w]],
            "name": "image name",
            "visible": true,
            "legendrank": 1000,
            "legendgrouptitle": {"text": "Legend Group Title"},
            "opacity": 0.5,
            "ids": ["one"],
            "x0": 0.0,
            "dx": 1.0,
            "y0": 2.0,
            "dy": 3.0,
            "source": "https://raw.githubusercontent.com/michaelbabyn/plot_data/master/bridge.jpg",
            "text": ["text"],
            "hovertext": ["hover_text"],
            "hoverinfo": "x+y+z",
            "hovertemplate": ["hover_template"],
            "meta": "meta",
            "customdata": ["custom_data"],
            "xaxis": "x2",
            "yaxis": "y2",
            "colormodel": "rgba",
            "zmax": [[w, w, w, w, w], [w, w, w, w, w]],
            "zmin": [[b, b, b, b, b], [b, b, b, b, b]],
            "zsmooth": "fast",
            "hoverlabel": {},
            "uirevision": 6,
        });

        assert_eq!(to_value(image).unwrap(), expected);
    }
}
