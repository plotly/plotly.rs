use once_cell::sync::Lazy;

use crate::{
    common::{ColorBar, ColorScale, ColorScaleElement, Font, Label, Title},
    layout::{Axis, ColorAxis, HoverMode, LayoutColorScale, LayoutTemplate, Template},
};

pub static DEFAULT: Lazy<Template> = Lazy::new(|| {
    let layout_template = LayoutTemplate::new();
    Template::new().layout(layout_template)
});

pub static PLOTLY_WHITE: Lazy<Template> = Lazy::new(|| {
    let layout_template = LayoutTemplate::new()
        .color_axis(ColorAxis::new().color_bar(ColorBar::new().outline_width(0)))
        .color_scale(
            LayoutColorScale::new()
                .sequential(ColorScale::Vector(vec![
                    ColorScaleElement(0., "#0d0887".to_string()),
                    ColorScaleElement(0.1111111111111111, "#46039f".to_string()),
                    ColorScaleElement(0.2222222222222222, "#7201a8".to_string()),
                    ColorScaleElement(0.3333333333333333, "#9c179e".to_string()),
                    ColorScaleElement(0.4444444444444444, "#bd3786".to_string()),
                    ColorScaleElement(0.5555555555555556, "#d8576b".to_string()),
                    ColorScaleElement(0.6666666666666666, "#ed7953".to_string()),
                    ColorScaleElement(0.7777777777777778, "#fb9f3a".to_string()),
                    ColorScaleElement(0.8888888888888888, "#fdca26".to_string()),
                    ColorScaleElement(1., "#f0f921".to_string()),
                ]))
                .sequential_minus(ColorScale::Vector(vec![
                    ColorScaleElement(0., "#0d0887".to_string()),
                    ColorScaleElement(0.1111111111111111, "#46039f".to_string()),
                    ColorScaleElement(0.2222222222222222, "#7201a8".to_string()),
                    ColorScaleElement(0.3333333333333333, "#9c179e".to_string()),
                    ColorScaleElement(0.4444444444444444, "#bd3786".to_string()),
                    ColorScaleElement(0.5555555555555556, "#d8576b".to_string()),
                    ColorScaleElement(0.6666666666666666, "#ed7953".to_string()),
                    ColorScaleElement(0.7777777777777778, "#fb9f3a".to_string()),
                    ColorScaleElement(0.8888888888888888, "#fdca26".to_string()),
                    ColorScaleElement(1., "#f0f921".to_string()),
                ]))
                .diverging(ColorScale::Vector(vec![
                    ColorScaleElement(0., "#8e0152".to_string()),
                    ColorScaleElement(0.1, "#c51b7d".to_string()),
                    ColorScaleElement(0.2, "#de77ae".to_string()),
                    ColorScaleElement(0.3, "#f1b6da".to_string()),
                    ColorScaleElement(0.4, "#fde0ef".to_string()),
                    ColorScaleElement(0.5, "#f7f7f7".to_string()),
                    ColorScaleElement(0.6, "#e6f5d0".to_string()),
                    ColorScaleElement(0.7, "#b8e186".to_string()),
                    ColorScaleElement(0.8, "#7fbc41".to_string()),
                    ColorScaleElement(0.9, "#4d9221".to_string()),
                    ColorScaleElement(1., "#276419".to_string()),
                ])),
        )
        .colorway(vec![
            "#636efa", "#EF553B", "#00cc96", "#ab63fa", "#FFA15A", "#19d3f3", "#FF6692", "#B6E880",
            "#FF97FF", "#FECB52",
        ])
        .font(Font::new().color("#2a3f5f"))
        .hover_label(Label::new().align("left"))
        .hover_mode(HoverMode::Closest)
        .paper_background_color("#ffffff")
        .plot_background_color("#ffffff")
        .title(Title::new().x(0.05))
        .x_axis(
            Axis::new()
                .auto_margin(true)
                .grid_color("#EBF0F8")
                .line_color("#EBF0F8")
                // missing title.standoff = 15
                .zero_line_color("#EBF0F8")
                .zero_line_width(2),
        )
        .y_axis(
            Axis::new()
                .auto_margin(true)
                .grid_color("#EBF0F8")
                .line_color("#EBF0F8")
                // missing title.standoff = 15
                .zero_line_color("#EBF0F8")
                .zero_line_width(2),
        );
    Template::new().layout(layout_template)
});

pub static PLOTLY_DARK: Lazy<Template> = Lazy::new(|| {
    // the following are unimplemented: layout.autotypenumbers, layout.polar,
    // layout.ternary, layout.scene, layout.geo, layout.mapbox, layout.*defaults
    let layout_template = LayoutTemplate::new()
        .color_axis(ColorAxis::new().color_bar(ColorBar::new().outline_width(0)))
        .color_scale(
            LayoutColorScale::new()
                .sequential(ColorScale::Vector(vec![
                    ColorScaleElement(0., "#0d0887".to_string()),
                    ColorScaleElement(0.1111111111111111, "#46039f".to_string()),
                    ColorScaleElement(0.2222222222222222, "#7201a8".to_string()),
                    ColorScaleElement(0.3333333333333333, "#9c179e".to_string()),
                    ColorScaleElement(0.4444444444444444, "#bd3786".to_string()),
                    ColorScaleElement(0.5555555555555556, "#d8576b".to_string()),
                    ColorScaleElement(0.6666666666666666, "#ed7953".to_string()),
                    ColorScaleElement(0.7777777777777778, "#fb9f3a".to_string()),
                    ColorScaleElement(0.8888888888888888, "#fdca26".to_string()),
                    ColorScaleElement(1., "#f0f921".to_string()),
                ]))
                .sequential_minus(ColorScale::Vector(vec![
                    ColorScaleElement(0., "#0d0887".to_string()),
                    ColorScaleElement(0.1111111111111111, "#46039f".to_string()),
                    ColorScaleElement(0.2222222222222222, "#7201a8".to_string()),
                    ColorScaleElement(0.3333333333333333, "#9c179e".to_string()),
                    ColorScaleElement(0.4444444444444444, "#bd3786".to_string()),
                    ColorScaleElement(0.5555555555555556, "#d8576b".to_string()),
                    ColorScaleElement(0.6666666666666666, "#ed7953".to_string()),
                    ColorScaleElement(0.7777777777777778, "#fb9f3a".to_string()),
                    ColorScaleElement(0.8888888888888888, "#fdca26".to_string()),
                    ColorScaleElement(1., "#f0f921".to_string()),
                ]))
                .diverging(ColorScale::Vector(vec![
                    ColorScaleElement(0., "#8e0152".to_string()),
                    ColorScaleElement(0.1, "#c51b7d".to_string()),
                    ColorScaleElement(0.2, "#de77ae".to_string()),
                    ColorScaleElement(0.3, "#f1b6da".to_string()),
                    ColorScaleElement(0.4, "#fde0ef".to_string()),
                    ColorScaleElement(0.5, "#f7f7f7".to_string()),
                    ColorScaleElement(0.6, "#e6f5d0".to_string()),
                    ColorScaleElement(0.7, "#b8e186".to_string()),
                    ColorScaleElement(0.8, "#7fbc41".to_string()),
                    ColorScaleElement(0.9, "#4d9221".to_string()),
                    ColorScaleElement(1., "#276419".to_string()),
                ])),
        )
        .colorway(vec![
            "#636efa", "#EF553B", "#00cc96", "#ab63fa", "#FFA15A", "#19d3f3", "#FF6692", "#B6E880",
            "#FF97FF", "#FECB52",
        ])
        .font(Font::new().color("#f2f5fa"))
        .hover_label(Label::new().align("left"))
        .hover_mode(HoverMode::Closest)
        .paper_background_color("#111111")
        .plot_background_color("#111111")
        .title(Title::new().x(0.05))
        .x_axis(
            Axis::new()
                .auto_margin(true)
                .grid_color("#283442")
                .line_color("#506784")
                // missing title.standoff = 15
                .zero_line_color("#283442")
                .zero_line_width(2),
        )
        .y_axis(
            Axis::new()
                .auto_margin(true)
                .grid_color("#283442")
                .line_color("#506784")
                // missing title.standoff = 15
                .zero_line_color("#283442")
                .zero_line_width(2),
        );
    Template::new().layout(layout_template)
});

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn plotly_default() {
        let template = &*DEFAULT;
        let layout = Layout::new().template(template);
        let mut plot = Plot::new();
        plot.set_layout(layout);
        plot.add_trace(Bar::new(vec![0], vec![1]));

        let expected = r##"{"template":{"layout":{}}}"##; // etc...
        assert!(plot.to_json().contains(expected));
    }

    #[test]
    fn plotly_white() {
        let template = &*PLOTLY_WHITE;
        let layout = Layout::new().template(template);
        let mut plot = Plot::new();
        plot.set_layout(layout);
        plot.add_trace(Bar::new(vec![0], vec![1]));
        dbg!(plot.to_json());

        let expected = r##"{"template":{"layout":{"title":{"x":0.05},"font":{"color":"#2a3f5f"}"##; // etc...
        assert!(plot.to_json().contains(expected));
    }

    #[test]
    fn plotly_dark() {
        let template = &*PLOTLY_DARK;
        let layout = Layout::new().template(template);
        let mut plot = Plot::new();
        plot.set_layout(layout);
        plot.add_trace(Bar::new(vec![0], vec![1]));

        let expected = r##"{"template":{"layout":{"title":{"x":0.05},"font":{"color":"#f2f5fa"}"##; // etc...
        assert!(plot.to_json().contains(expected));
    }
}
