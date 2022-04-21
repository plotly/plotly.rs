use once_cell::sync::Lazy;

use crate::layout::{LayoutTemplate, Template};

static PLOTLY_DARK: Lazy<Template> = Lazy::new(|| {
    let layout_template = LayoutTemplate::new()
        .plot_background_color("#111111")
        .paper_background_color("#111111");
    let template = Template::new().layout(layout_template);
    template
});

#[cfg(test)]
mod tests {
    use super::*;
    use crate::*;

    #[test]
    fn test_plotly_dark() {
        let template = &*PLOTLY_DARK;
        let layout = Layout::new().template_ref(template);
        let mut plot = Plot::new();
        plot.set_layout(layout);
        plot.add_trace(Bar::new(vec![0], vec![1]));

        let expected = r##"{"data": [{"x":[0],"y":[1],"type":"bar"}], "layout": {"template":{"layout":{"paper_bgcolor":"#111111","plot_bgcolor":"#111111"}}}}"##;
        assert_eq!(plot.to_json(), expected);
    }
}
