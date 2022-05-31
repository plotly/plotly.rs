# Sankey Diagrams

The following imports have been used to produce the plots below:

```rust
use itertools_num::linspace;
use plotly::common::{
    ColorScale, ColorScalePalette, DashType, Fill, Font, Line, LineShape, Marker, Mode, Title,
};
use plotly::layout::{Axis, BarMode, Layout, Legend, TicksDirection};
use plotly::Sankey;
use rand_distr::{Distribution, Normal, Uniform};
```

The `to_inline_html` method is used to produce the html plot displayed in this page.

## Constructing a basic Sankey diagram
```rust
let trace = Sankey::new()
        .orientation(Orientation::Horizontal)
        .node(
            Node::new()
                .pad(15)
                .thickness(30)
                .line(SankeyLine::new().color(NamedColor::Black).width(0.5))
                .label(vec!["A1", "A2", "B1", "B2", "C1", "C2"])
                .color_array(vec![
                    NamedColor::Blue,
                    NamedColor::Blue,
                    NamedColor::Blue,
                    NamedColor::Blue,
                    NamedColor::Blue,
                    NamedColor::Blue,
                ]),
        )
        .link(
            Link::new()
                .value(vec![8, 4, 2, 8, 4, 2])
                .source(vec![0, 1, 0, 2, 3, 3])
                .target(vec![2, 3, 3, 4, 4, 5]),
        );

    let layout = Layout::new()
        .title("Basic Sankey".into())
        .font(Font::new().size(10));

    let mut plot = Plot::new();
    plot.add_trace(trace);
    plot.set_layout(layout);

    if show {
        plot.show();
    }

    println!("{}", plot.to_inline_html(Some("basic_sankey_diagram")));
}
```
<div id="basic_sankey_diagram" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("basic_sankey_diagram")) {
        var image_element = document.getElementById('image-export')

        Plotly.newPlot('basic_sankey_diagram', {
  "data": [
    {
      "type": "sankey",
      "orientation": "h",
      "node": {
        "color": [
          "blue",
          "blue",
          "blue",
          "blue",
          "blue",
          "blue"
        ],
        "label": [
          "A1",
          "A2",
          "B1",
          "B2",
          "C1",
          "C2"
        ],
        "line": {
          "color": "black",
          "width": 0.5
        },
        "pad": 15,
        "thickness": 30
      },
      "link": {
        "source": [
          0,
          1,
          0,
          2,
          3,
          3
        ],
        "target": [
          2,
          3,
          3,
          4,
          4,
          5
        ],
        "value": [
          8,
          4,
          2,
          8,
          4,
          2
        ]
      }
    }
  ],
  "layout": {
    "title": {
      "text": "Basic Sankey"
    },
    "font": {
      "size": 10
    }
  },
  "config": {}
});
    };
</script>