//! Scatter trace

#[cfg(feature = "plotly_ndarray")]
use ndarray::{Array, Ix1, Ix2};
use plotly_derive::FieldSetter;
use serde::Serialize;

#[cfg(feature = "plotly_ndarray")]
use crate::ndarray::ArrayTraces;
use crate::{
    color::Color,
    common::{
        Calendar, Dim, ErrorData, Fill, Font, HoverInfo, HoverOn, Label, LegendGroupTitle, Line,
        Marker, Mode, Orientation, PlotType, Position, Visible,
    },
    private::{NumOrString, NumOrStringCollection},
    Trace,
};

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum GroupNorm {
    #[serde(rename = "")]
    Default,
    Fraction,
    Percent,
}

#[derive(Serialize, Clone, Debug)]
#[serde(rename_all = "lowercase")]
pub enum StackGaps {
    #[serde(rename = "infer zero")]
    InferZero,
    Interpolate,
}

/// Construct a scatter trace.
///
/// # Examples
///
/// ```
/// use plotly::Scatter;
///
/// let trace = Scatter::new(vec![0, 1, 2], vec![2, 1, 0]);
///
/// let expected = serde_json::json!({
///     "type": "scatter",
///     "x": [0, 1, 2],
///     "y": [2, 1, 0]
/// });
///
/// assert_eq!(serde_json::to_value(trace).unwrap(), expected);
/// ```
#[serde_with::skip_serializing_none]
#[derive(Serialize, Clone, Debug, FieldSetter)]
#[field_setter(box_self, kind = "trace")]
pub struct Scatter<X, Y>
where
    X: Serialize + Clone + 'static,
    Y: Serialize + Clone + 'static,
{
    #[field_setter(default = "PlotType::Scatter")]
    r#type: PlotType,
    /// Sets the trace name. The trace name appear as the legend item and on hover.
    name: Option<String>,
    /// Determines whether or not this trace is visible. If `Visible::LegendOnly`, the trace is not
    /// drawn, but can appear as a legend item (provided that the legend itself is visible).
    visible: Option<Visible>,
    /// Determines whether or not an item corresponding to this trace is shown in the legend.
    #[serde(rename = "showlegend")]
    show_legend: Option<bool>,
    /// Sets the legend group for this trace. Traces part of the same legend group hide/show at the
    /// same time when toggling legend items.
    #[serde(rename = "legendgroup")]
    legend_group: Option<String>,
    /// Set and style the title to appear for the legend group
    #[serde(rename = "legendgrouptitle")]
    legend_group_title: Option<LegendGroupTitle>,
    /// Sets the opacity of the trace.
    opacity: Option<f64>,
    /// Determines the drawing mode for this scatter trace. If the provided `Mode` includes
    /// "Text" then the `text` elements appear at the coordinates. Otherwise, the `text` elements
    /// appear on hover. If there are less than 20 points and the trace is not stacked then the
    /// default is `Mode::LinesMarkers`, otherwise it is `Mode::Lines`.
    mode: Option<Mode>,
    /// Assigns id labels to each datum. These ids for object constancy of data points during
    /// animation. Should be an array of strings, not numbers or any other type.
    ids: Option<Vec<String>>,
    x: Option<Vec<X>>,
    /// Alternate to `x`. Builds a linear space of x coordinates. Use with `dx` where `x0` is the
    /// starting coordinate and `dx` the step.
    x0: Option<NumOrString>,
    /// Sets the x coordinate step. See `x0` for more info.
    dx: Option<f64>,

    y: Option<Vec<Y>>,

    /// Alternate to `y`. Builds a linear space of y coordinates. Use with `dy` where `y0` is the
    /// starting coordinate and `dy` the step.
    y0: Option<NumOrString>,
    /// Sets the y coordinate step. See `y0` for more info.
    dy: Option<f64>,

    /// Sets text elements associated with each (x,y) pair. If a single string, the same string
    /// appears over all the data points. If an array of string, the items are mapped in order to
    /// the this trace's (x,y) coordinates. If the trace `HoverInfo` contains a "text" flag and
    /// `hover_text` is not set, these elements will be seen in the hover labels.
    text: Option<Dim<String>>,
    /// Sets the positions of the `text` elements with respects to the (x,y) coordinates.
    #[serde(rename = "textposition")]
    text_position: Option<Dim<Position>>,
    /// Template string used for rendering the information text that appear on points. Note that
    /// this will override `textinfo`. Variables are inserted using %{variable}, for example
    /// "y: %{y}". Numbers are formatted using d3-format's syntax %{variable:d3-format}, for example
    /// "Price: %{y:$.2f}". See [format](https://github.com/d3/d3-3.x-api-reference/blob/master/Formatting.md#d3)
    /// for details on the formatting syntax. Dates are formatted using d3-time-format's syntax
    /// %{variable|d3-time-format}, for example "Day: %{2019-01-01|%A}".
    /// See [format](https://github.com/d3/d3-3.x-api-reference/blob/master/Time-Formatting.md#format) for details
    /// on the date formatting syntax. Every attributes that can be specified per-point (the ones
    /// that are `arrayOk: true`) are available.
    #[serde(rename = "texttemplate")]
    text_template: Option<Dim<String>>,
    /// Sets hover text elements associated with each (x,y) pair. If a single string, the same
    /// string appears over all the data points. If an array of string, the items are mapped in
    /// order to the this trace's (x,y) coordinates. To be seen, trace `HoverInfo` must contain a
    /// "Text" flag.
    #[serde(rename = "hovertext")]
    hover_text: Option<Dim<String>>,
    /// Determines which trace information appear on hover. If `HoverInfo::None` or `HoverInfo::Skip`
    /// are set, no information is displayed upon hovering. But, if `HoverInfo::None` is set, click
    /// and hover events are still fired.
    #[serde(rename = "hoverinfo")]
    hover_info: Option<HoverInfo>,
    /// Template string used for rendering the information that appear on hover box. Note that this
    /// will override `HoverInfo`. Variables are inserted using %{variable}, for example "y: %{y}".
    /// Numbers are formatted using d3-format's syntax %{variable:d3-format}, for example
    /// "Price: %{y:$.2f}".
    /// https://github.com/d3/d3-3.x-api-reference/blob/master/Formatting.md#d3_format for details
    /// on the formatting syntax. Dates are formatted using d3-time-format's syntax
    /// %{variable|d3-time-format}, for example "Day: %{2019-01-01|%A}".
    /// https://github.com/d3/d3-3.x-api-reference/blob/master/Time-Formatting.md#format for details
    /// on the date formatting syntax. The variables available in `hovertemplate` are the ones
    /// emitted as event data described at this link https://plotly.com/javascript/plotlyjs-events/#event-data.
    /// Additionally, every attributes that can be specified per-point (the ones that are
    /// `arrayOk: true`) are available. Anything contained in tag `<extra>` is displayed in the
    /// secondary box, for example "<extra>{fullData.name}</extra>". To hide the secondary box
    /// completely, use an empty tag `<extra></extra>`.
    #[serde(rename = "hovertemplate")]
    hover_template: Option<Dim<String>>,
    /// Assigns extra meta information associated with this trace that can be used in various text
    /// attributes. Attributes such as trace `name`, graph, axis and colorbar `title.text`,
    /// annotation `text` `rangeselector`, `updatemenues` and `sliders` `label` text all support
    /// `meta`. To access the trace `meta` values in an attribute in the same trace, simply use
    /// `%{meta[i]}` where `i` is the index or key of the `meta` item in question. To access trace
    /// `meta` in layout attributes, use `%{data[n[.meta[i]}` where `i` is the index or key of the
    /// `meta` and `n` is the trace index.
    meta: Option<NumOrString>,
    /// Assigns extra data each datum. This may be useful when listening to hover, click and
    /// selection events. Note that, "scatter" traces also appends customdata items in the markers
    /// DOM elements
    #[serde(rename = "customdata")]
    custom_data: Option<NumOrStringCollection>,

    /// Sets a reference between this trace's x coordinates and a 2D cartesian x axis. If "x" (
    /// the default value), the x coordinates refer to `Layout::x_axis`. If "x2", the x coordinates
    /// refer to `Layout::x_axis2`, and so on.
    #[serde(rename = "xaxis")]
    x_axis: Option<String>,
    /// Sets a reference between this trace's y coordinates and a 2D cartesian y axis. If "y"
    /// (the default value), the y coordinates refer to `Layout::y_axis`. If "y2", the y coordinates
    /// refer to `Layout::y_axis2`, and so on.
    #[serde(rename = "yaxis")]
    y_axis: Option<String>,
    /// Only relevant when `stackgroup` is used, and only the first `orientation` found in the
    /// `stackgroup` will be used - including if `visible` is "legendonly" but not if it is `false`.
    /// Sets the stacking direction. With "v" ("h"), the y (x) values of subsequent traces are
    /// added. Also affects the default value of `fill`.
    orientation: Option<Orientation>,
    /// Only relevant when `stackgroup` is used, and only the first `groupnorm` found in the
    /// `stackgroup` will be used - including if `visible` is "legendonly" but not if it is `false`.
    /// Sets the normalization for the sum of this `stackgroup`. With "fraction", the value of each
    /// trace at each location is divided by the sum of all trace values at that location. "percent"
    /// is the same but multiplied by 100 to show percentages. If there are multiple subplots, or
    /// multiple `stackgroup`s on one subplot, each will be normalized within its own set.
    #[serde(rename = "groupnorm")]
    group_norm: Option<GroupNorm>,
    /// Set several scatter traces (on the same subplot) to the same stackgroup in order to add
    /// their y values (or their x values if `orientation` is "h"). If blank or omitted this trace
    /// will not be stacked. Stacking also turns `fill` on by default, using "tonexty" ("tonextx")
    /// if `orientation` is "h" ("v") and sets the default `mode` to "lines" irrespective of point
    /// count. You can only stack on a numeric (linear or log) axis. Traces in a `stackgroup` will
    /// only fill to (or be filled to) other traces in the same group. With multiple `stackgroup`s
    /// or some traces stacked and some not, if fill-linked traces are not already consecutive, the
    /// later ones will be pushed down in the drawing order.
    #[serde(rename = "stackgroup")]
    stack_group: Option<String>,
    /// Determines how points are displayed and joined.
    marker: Option<Marker>,
    /// Line display properties.
    line: Option<Line>,
    /// Sets the text font.
    #[serde(rename = "textfont")]
    text_font: Option<Font>,
    /// x-axis error display properties
    error_x: Option<ErrorData>,
    /// y-axis error display properties.
    error_y: Option<ErrorData>,
    /// Determines whether or not markers and text nodes are clipped about the subplot axes. To show
    /// markers and text nodes above axis lines and tick labels, make sure to set `xaxis.layer` and
    /// `yaxis.layer` to "below traces".
    #[serde(rename = "cliponaxis")]
    clip_on_axis: Option<bool>,
    /// Determines whether or not gaps (i.e. {nan} or missing values) in the provided data arrays
    /// are connected.
    #[serde(rename = "connectgaps")]
    connect_gaps: Option<bool>,
    /// Sets the area to fill with a solid color. Defaults to "none" unless this trace is stacked,
    /// then it gets "tonexty" ("tonextx") if `orientation` is "v" ("h") Use with `fillcolor` if not
    /// "none". "tozerox" and "tozeroy" fill to x=0 and y=0 respectively. "tonextx" and "tonexty"
    /// fill between the endpoints of this trace and the endpoints of the trace before it,
    /// connecting those endpoints with straight lines (to make a stacked area graph); if there is
    /// no trace before it, they behave like "tozerox" and "tozeroy". "toself" connects the
    /// endpoints of the trace (or each segment of the trace if it has gaps) into a closed shape.
    /// "tonext" fills the space between two traces if one completely encloses the other
    /// (eg consecutive contour lines), and behaves like "toself" if there is no trace before it.
    /// "tonext" should not be used if one trace does not enclose the other. Traces in a
    /// `stackgroup` will only fill to (or be filled to) other traces in the same group. With
    /// multiple `stackgroup`s or some traces stacked and some not, if fill-linked traces are not
    /// already consecutive, the later ones will be pushed down in the drawing order.
    fill: Option<Fill>,
    /// Sets the fill color. Defaults to a half-transparent variant of the line color, marker color,
    /// or marker line color, whichever is available.
    #[serde(rename = "fillcolor")]
    fill_color: Option<Box<dyn Color>>,
    /// Properties of label displayed on mouse hover.
    #[serde(rename = "hoverlabel")]
    hover_label: Option<Label>,
    /// Do the hover effects highlight individual points (markers or line points) or do they
    /// highlight filled regions? If the fill is "toself" or "tonext" and there are no markers or
    /// text, then the default is "fills", otherwise it is "points".
    #[serde(rename = "hoveron")]
    hover_on: Option<HoverOn>,
    /// Only relevant when `stack_group` is used, and only the first `stack_gaps` found in the
    /// `stackgroup` will be used - including if `visible` is set to `Visible::LegendOnly` but not
    /// if it is set to `Visible::False`.
    /// Determines how we handle locations at which other traces in this group have data but this
    /// one does not. With "infer zero" we insert a zero at these locations. With "interpolate" we
    /// linearly interpolate between existing values, and extrapolate a constant beyond the existing
    /// values.
    #[serde(rename = "stackgaps")]
    stack_gaps: Option<StackGaps>,
    /// Sets the calendar system to use with `x` date data.
    #[serde(rename = "xcalendar")]
    x_calendar: Option<Calendar>,
    /// Sets the calendar system to use with `y` date data.
    #[serde(rename = "ycalendar")]
    y_calendar: Option<Calendar>,
}

impl<X, Y> Scatter<X, Y>
where
    X: Serialize + Clone + 'static,
    Y: Serialize + Clone + 'static,
{
    pub fn new(x: Vec<X>, y: Vec<Y>) -> Box<Self> {
        Box::new(Self {
            x: Some(x),
            y: Some(y),
            ..Default::default()
        })
    }

    #[cfg(feature = "plotly_ndarray")]
    pub fn from_array(x: Array<X, Ix1>, y: Array<Y, Ix1>) -> Box<Self> {
        Box::new(Scatter {
            x: Some(x.to_vec()),
            y: Some(y.to_vec()),
            ..Default::default()
        })
    }

    /// Produces `Scatter` traces from a 2 dimensional tensor (`traces_matrix`) indexed by `x`. This
    /// function requires the `ndarray` feature.
    ///
    /// # Arguments
    /// * `x`             - One dimensional array (or view) that represents the `x` axis coordinates.
    /// * `traces_matrix` - Two dimensional array (or view) containing the `y` axis coordinates of
    /// the traces.
    /// * `array_traces`  - Determines whether the traces are arranged in the matrix over the
    /// columns (`ArrayTraces::OverColumns`) or over the rows (`ArrayTraces::OverRows`).
    ///
    /// # Examples
    ///
    /// ```
    /// use plotly::common::Mode;
    /// use plotly::{Plot, Scatter, ArrayTraces};
    /// use ndarray::{Array, Ix1, Ix2};
    ///
    /// fn ndarray_to_traces() {
    ///     let n: usize = 11;
    ///     let t: Array<f64, Ix1> = Array::range(0., 10., 10. / n as f64);
    ///     let mut ys: Array<f64, Ix2> = Array::zeros((11, 11));
    ///     let mut count = 0.;
    ///     for mut row in ys.columns_mut() {
    ///         for index in 0..row.len() {
    ///             row[index] = count + (index as f64).powf(2.);
    ///         }
    ///         count += 1.;
    ///     }
    ///
    ///     let traces = Scatter::default()
    ///         .mode(Mode::LinesMarkers)
    ///         .to_traces(t, ys, ArrayTraces::OverColumns);
    ///
    ///     let mut plot = Plot::new();
    ///     plot.add_traces(traces);
    ///
    ///     # if false { // Skip running this line in doctests.
    ///     plot.show();
    ///     # }
    /// }
    /// fn main() -> std::io::Result<()> {
    ///     ndarray_to_traces();
    ///     Ok(())
    /// }
    /// ```
    #[cfg(feature = "plotly_ndarray")]
    pub fn to_traces(
        &self,
        x: Array<X, Ix1>,
        traces_matrix: Array<Y, Ix2>,
        array_traces: ArrayTraces,
    ) -> Vec<Box<dyn Trace>> {
        let mut traces: Vec<Box<dyn Trace>> = Vec::new();
        let mut trace_vectors = crate::private::trace_vectors_from(traces_matrix, array_traces);
        trace_vectors.reverse();
        while !trace_vectors.is_empty() {
            let mut sc = Box::new(self.clone());
            sc.x = Some(x.to_vec());
            let data = trace_vectors.pop();
            if let Some(d) = data {
                sc.y = Some(d);
                traces.push(sc);
            }
        }

        traces
    }

    /// Enables WebGL.
    pub fn web_gl_mode(mut self, on: bool) -> Box<Self> {
        self.r#type = if on {
            PlotType::ScatterGL
        } else {
            PlotType::Scatter
        };
        Box::new(self)
    }
}

impl<X, Y> Trace for Scatter<X, Y>
where
    X: Serialize + Clone + 'static,
    Y: Serialize + Clone + 'static,
{
    fn to_json(&self) -> String {
        serde_json::to_string(self).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use serde_json::{json, to_value};

    use super::*;

    #[test]
    fn test_serialize_group_norm() {
        assert_eq!(to_value(GroupNorm::Default).unwrap(), json!(""));
        assert_eq!(to_value(GroupNorm::Fraction).unwrap(), json!("fraction"));
        assert_eq!(to_value(GroupNorm::Percent).unwrap(), json!("percent"));
    }

    #[test]
    #[rustfmt::skip]
    fn test_serialize_stack_gaps() {
        assert_eq!(to_value(StackGaps::InferZero).unwrap(), json!("infer zero"));
        assert_eq!(to_value(StackGaps::Interpolate).unwrap(), json!("interpolate"));
    }

    #[test]
    fn test_serialize_default_scatter() {
        let trace = Scatter::<u32, u32>::default();
        let expected = json!({"type": "scatter"});

        assert_eq!(to_value(trace).unwrap(), expected);
    }

    #[test]
    fn test_serialize_scatter() {
        use crate::common::ErrorType;

        let trace = Scatter::new(vec![0, 1], vec![2, 3])
            .clip_on_axis(true)
            .connect_gaps(false)
            .custom_data(vec!["custom_data"])
            .error_x(ErrorData::new(ErrorType::Percent))
            .error_y(ErrorData::new(ErrorType::Data))
            .dx(1.0)
            .dy(4.0)
            .fill(Fill::ToNext)
            .fill_color("#789456")
            .group_norm(GroupNorm::Default)
            .hover_info(HoverInfo::Name)
            .hover_label(Label::new())
            .hover_on(HoverOn::Points)
            .hover_text("hover_text")
            .hover_text_array(vec!["hover_text"])
            .hover_template("hover_template")
            .hover_template_array(vec!["hover_template"])
            .ids(vec!["1"])
            .legend_group("legend_group")
            .legend_group_title(LegendGroupTitle::new("Legend Group Title"))
            .line(Line::new())
            .marker(Marker::new())
            .meta("meta")
            .mode(Mode::LinesMarkers)
            .name("scatter_trace")
            .opacity(0.6)
            .orientation(Orientation::Horizontal)
            .show_legend(false)
            .stack_gaps(StackGaps::InferZero)
            .stack_group("stack_group")
            .text("text")
            .text_array(vec!["text"])
            .text_font(Font::new())
            .text_position(Position::MiddleCenter)
            .text_position_array(vec![Position::MiddleLeft])
            .text_template("text_template")
            .text_template_array(vec!["text_template"])
            .visible(Visible::True)
            .x_axis("x_axis")
            .x_calendar(Calendar::Chinese)
            .x0(0)
            .y_axis("y_axis")
            .y_calendar(Calendar::Coptic)
            .y0(2)
            .web_gl_mode(true);

        let expected = json!({
            "type": "scattergl",
            "x": [0, 1],
            "y": [2, 3],
            "cliponaxis": true,
            "connectgaps": false,
            "customdata": ["custom_data"],
            "error_x": {"type": "percent"},
            "error_y": {"type": "data"},
            "dx": 1.0,
            "dy": 4.0,
            "fill": "tonext",
            "fillcolor": "#789456",
            "groupnorm": "",
            "hoverinfo": "name",
            "hoverlabel": {},
            "hoveron": "points",
            "hovertext": ["hover_text"],
            "hovertemplate": ["hover_template"],
            "ids": ["1"],
            "legendgroup": "legend_group",
            "legendgrouptitle": {"text": "Legend Group Title"},
            "line": {},
            "marker": {},
            "meta": "meta",
            "mode": "lines+markers",
            "name": "scatter_trace",
            "opacity": 0.6,
            "orientation": "h",
            "showlegend": false,
            "stackgaps": "infer zero",
            "stackgroup": "stack_group",
            "text": ["text"],
            "textfont": {},
            "textposition": ["middle left"],
            "texttemplate": ["text_template"],
            "visible": true,
            "xaxis": "x_axis",
            "xcalendar": "chinese",
            "x0": 0,
            "yaxis": "y_axis",
            "ycalendar": "coptic",
            "y0": 2
        });

        assert_eq!(to_value(trace).unwrap(), expected);
    }
}
