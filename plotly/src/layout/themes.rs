use once_cell::sync::Lazy;

use crate::{
    common::{ColorBar, ColorScale, ColorScaleElement, Font, Label, Title},
    layout::{Axis, ColorAxis, HoverMode, LayoutColorScale, LayoutTemplate, Template, TicksDirection},
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
    // layout.ternary, layout.scene, layout.*defaults
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

pub static SEABORN: Lazy<Template> = Lazy::new(|| {
    // Based on seaborn's "darkgrid" style and "deep" color palette
    let layout_template = LayoutTemplate::new()
        .font(Font::new().family("DejaVu Sans").size(12).color("#333333"))
        .colorway(vec![
            "#4c72b0", "#55a868", "#c44e52", "#8172b2", "#ccb974", "#64b5cd",
        ])
        .paper_background_color("#EAEAF2")
        .plot_background_color("#EAEAF2")
        .hover_label(Label::new().align("left"))
        .hover_mode(HoverMode::Closest)
        .x_axis(
            Axis::new()
                .auto_margin(true)
                .grid_color("#D3D3D3")
                .line_color("#CCCCCC")
                .zero_line_color("#D3D3D3")
                .zero_line_width(1),
        )
        .y_axis(
            Axis::new()
                .auto_margin(true)
                .grid_color("#D3D3D3")
                .line_color("#CCCCCC")
                .zero_line_color("#D3D3D3")
                .zero_line_width(1),
        );
    Template::new().layout(layout_template)
});

pub static SEABORN_WHITEGRID: Lazy<Template> = Lazy::new(|| {
    // Seaborn "whitegrid" style
    let layout_template = LayoutTemplate::new()
        .font(Font::new().family("DejaVu Sans").size(12).color("#333333"))
        .colorway(vec![
            "#4c72b0", "#55a868", "#c44e52", "#8172b2", "#ccb974", "#64b5cd",
        ])
        .paper_background_color("#FFFFFF")
        .plot_background_color("#FFFFFF")
        .hover_label(Label::new().align("left"))
        .hover_mode(HoverMode::Closest)
        .x_axis(
            Axis::new()
                .auto_margin(true)
                .grid_color("#E5E5E5")
                .line_color("#CCCCCC")
                .zero_line_color("#E5E5E5")
                .zero_line_width(1),
        )
        .y_axis(
            Axis::new()
                .auto_margin(true)
                .grid_color("#E5E5E5")
                .line_color("#CCCCCC")
                .zero_line_color("#E5E5E5")
                .zero_line_width(1),
        );
    Template::new().layout(layout_template)
});

pub static SEABORN_DARK: Lazy<Template> = Lazy::new(|| {
    // Seaborn "dark" style
    let layout_template = LayoutTemplate::new()
        .font(Font::new().family("DejaVu Sans").size(12).color("#eaeaf2"))
        .colorway(vec![
            "#4c72b0", "#55a868", "#c44e52", "#8172b2", "#ccb974", "#64b5cd",
        ])
        .paper_background_color("#222222")
        .plot_background_color("#222222")
        .hover_label(Label::new().align("left"))
        .hover_mode(HoverMode::Closest)
        .x_axis(
            Axis::new()
                .auto_margin(true)
                .grid_color("#444444")
                .line_color("#888888")
                .zero_line_color("#444444")
                .zero_line_width(1),
        )
        .y_axis(
            Axis::new()
                .auto_margin(true)
                .grid_color("#444444")
                .line_color("#888888")
                .zero_line_color("#444444")
                .zero_line_width(1),
        );
    Template::new().layout(layout_template)
});

pub static MATPLOTLIB: Lazy<Template> = Lazy::new(|| {
    // Based on matplotlib's classic style with publication-ready formatting
    // Inspired by: https://medium.com/swlh/formatting-a-plotly-figure-with-matplotlib-style-fa56ddd97539
    let layout_template = LayoutTemplate::new()
        .font(Font::new().family("Arial").size(26).color("black"))
        .colorway(vec![
            "#1f77b4", "#ff7f0e", "#2ca02c", "#d62728", "#9467bd", "#8c564b",
            "#e377c2", "#7f7f7f", "#bcbd22", "#17becf",
        ])
        .paper_background_color("white")
        .plot_background_color("white")
        .hover_label(Label::new().align("left"))
        .hover_mode(HoverMode::Closest)
        .x_axis(
            Axis::new()
                .auto_margin(true)
                .show_line(true)
                .line_color("black")
                .line_width(2)
                .ticks(TicksDirection::Outside)
                .tick_width(2)
                .tick_color("black")
                .mirror(true)
                .grid_color("#e5e5e5")
                .zero_line_color("#e5e5e5")
                .zero_line_width(1),
        )
        .y_axis(
            Axis::new()
                .auto_margin(true)
                .show_line(true)
                .line_color("black")
                .line_width(2)
                .ticks(TicksDirection::Outside)
                .tick_width(2)
                .tick_color("black")
                .mirror(true)
                .grid_color("#e5e5e5")
                .zero_line_color("#e5e5e5")
                .zero_line_width(1),
        );
    Template::new().layout(layout_template)
});

pub static PLOTNINE: Lazy<Template> = Lazy::new(|| {
    // Based on plotnine's default theme (ggplot2-inspired)
    let layout_template = LayoutTemplate::new()
        .font(Font::new().family("DejaVu Sans").size(12).color("#525252"))
        .colorway(vec![
            "#F8766D", "#7CAE00", "#00BFC4", "#C77CFF", "#E58700", "#00B0F6", "#FF61C3",
        ])
        .paper_background_color("#EBEBEB")
        .plot_background_color("#EBEBEB")
        .hover_label(Label::new().align("left"))
        .hover_mode(HoverMode::Closest)
        .x_axis(
            Axis::new()
                .auto_margin(true)
                .grid_color("#FFFFFF")
                .line_color("#CCCCCC")
                .zero_line_color("#FFFFFF")
                .zero_line_width(1),
        )
        .y_axis(
            Axis::new()
                .auto_margin(true)
                .grid_color("#FFFFFF")
                .line_color("#CCCCCC")
                .zero_line_color("#FFFFFF")
                .zero_line_width(1),
        );
    Template::new().layout(layout_template)
});

/// Enum for all built-in themes.
#[derive(Clone, Copy, Debug)]
pub enum BuiltinTheme {
    Default,
    PlotlyWhite,
    PlotlyDark,
    Seaborn,
    SeabornWhitegrid,
    SeabornDark,
    Matplotlib,
    Plotnine,
}

impl BuiltinTheme {
    /// Get a reference to the template for this theme.
    pub fn build(&self) -> &'static Template {
        match self {
            BuiltinTheme::Default => &DEFAULT,
            BuiltinTheme::PlotlyWhite => &PLOTLY_WHITE,
            BuiltinTheme::PlotlyDark => &PLOTLY_DARK,
            BuiltinTheme::Seaborn => &SEABORN,
            BuiltinTheme::SeabornWhitegrid => &SEABORN_WHITEGRID,
            BuiltinTheme::SeabornDark => &SEABORN_DARK,
            BuiltinTheme::Matplotlib => &MATPLOTLIB,
            BuiltinTheme::Plotnine => &PLOTNINE,
        }
    }
}

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

    #[test]
    fn seaborn() {
        let template = &*SEABORN;
        let layout = Layout::new().template(template);
        let mut plot = Plot::new();
        plot.set_layout(layout);
        plot.add_trace(Bar::new(vec![0], vec![1]));

        let expected = r##""paper_bgcolor":"#EAEAF2""##;
        assert!(plot.to_json().contains(expected));
    }

    #[test]
    fn seaborn_whitegrid() {
        let template = &*SEABORN_WHITEGRID;
        let layout = Layout::new().template(template);
        let mut plot = Plot::new();
        plot.set_layout(layout);
        plot.add_trace(Bar::new(vec![0], vec![1]));

        let expected = r##""paper_bgcolor":"#FFFFFF""##;
        assert!(plot.to_json().contains(expected));
    }

    #[test]
    fn seaborn_dark() {
        let template = &*SEABORN_DARK;
        let layout = Layout::new().template(template);
        let mut plot = Plot::new();
        plot.set_layout(layout);
        plot.add_trace(Bar::new(vec![0], vec![1]));

        let expected = r##""paper_bgcolor":"#222222""##;
        assert!(plot.to_json().contains(expected));
    }

    #[test]
    fn matplotlib() {
        let template = &*MATPLOTLIB;
        let layout = Layout::new().template(template);
        let mut plot = Plot::new();
        plot.set_layout(layout);
        plot.add_trace(Bar::new(vec![0], vec![1]));

        let expected = r##""paper_bgcolor":"#ffffff""##;
        assert!(plot.to_json().contains(expected));
    }

    #[test]
    fn plotnine() {
        let template = &*PLOTNINE;
        let layout = Layout::new().template(template);
        let mut plot = Plot::new();
        plot.set_layout(layout);
        plot.add_trace(Bar::new(vec![0], vec![1]));

        let expected = r##""paper_bgcolor":"#EBEBEB""##;
        assert!(plot.to_json().contains(expected));
    }

    #[test]
    fn theme_enum() {
        for theme in [
            BuiltinTheme::Default,
            BuiltinTheme::PlotlyWhite,
            BuiltinTheme::PlotlyDark,
            BuiltinTheme::Seaborn,
            BuiltinTheme::SeabornWhitegrid,
            BuiltinTheme::SeabornDark,
            BuiltinTheme::Matplotlib,
            BuiltinTheme::Plotnine,
        ] {
            let template = theme.build();
            let layout = Layout::new().template(template);
            let mut plot = Plot::new();
            plot.set_layout(layout);
            plot.add_trace(Bar::new(vec![0], vec![1]));
            assert!(!plot.to_json().is_empty());
        }
    }
}
