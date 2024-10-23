# Shapes

The following imports have been used to produce the plots below:

```rust
use itertools_num::linspace;
use plotly::common::{
    Fill, Font, Mode,
};
use plotly::layout::{
    Axis, GridPattern, Layout, LayoutGrid, Margin, Shape, ShapeLayer, ShapeLine,
    ShapeType,
};
use plotly::{Bar, color::NamedColor, Plot, Scatter};
use rand::thread_rng;
use rand_distr::{Distribution, Normal};
```

The `to_inline_html` method is used to produce the html plot displayed in this page.


## Filled Area Chart
```rust
fn filled_area_chart(show: bool) {
    let trace1 = Scatter::new(vec![0, 1, 2, 0], vec![0, 2, 0, 0]).fill(Fill::ToSelf);
    let trace2 =
        Scatter::new(vec![3, 3, 5, 5, 3], vec![0.5, 1.5, 1.5, 0.5, 0.5]).fill(Fill::ToSelf);
    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    if show {
        plot.show();
    }
    println!("{}", plot.to_inline_html(Some("filled_area_chart")));
}
```
<div id="filled_area_chart" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("filled_area_chart")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"type":"scatter","x":[0,1,2,0],"x0":null,"dx":null,"y":[0,2,0,0],"y0":null,"dy":null,"meta":null,"custom_data":null,"fill":"toself"};
var trace_1 = {"type":"scatter","x":[3,3,5,5,3],"x0":null,"dx":null,"y":[0.5,1.5,1.5,0.5,0.5],"y0":null,"dy":null,"meta":null,"custom_data":null,"fill":"toself"};
var data = [trace_0,trace_1];
var layout = {};
        Plotly.newPlot('filled_area_chart', data, layout, {"responsive": true});
    };
</script>


## Vertical and Horizontal Lines Positioned Relative to Axes
```rust
fn vertical_and_horizontal_lines_positioned_relative_to_axes(show: bool) {
    let trace = Scatter::new(vec![2.0, 3.5, 6.0], vec![1.0, 1.5, 1.0])
        .text_array(vec![
            "Vertical Line",
            "Horizontal Dashed Line",
            "Diagonal dotted Line",
        ])
        .mode(Mode::Text);
    let mut plot = Plot::new();
    plot.add_trace(trace);

    let mut layout = Layout::new()
        .x_axis(Axis::new().range(vec![0.0, 7.0]))
        .y_axis(Axis::new().range(vec![0.0, 2.5]));

    layout.add_shape(
        Shape::new()
            .shape_type(ShapeType::Line)
            .x0(1)
            .y0(0)
            .x1(1)
            .y1(2)
            .line(ShapeLine::new().color(NamedColor::RoyalBlue).width(3.)),
    );
    layout.add_shape(
        Shape::new()
            .shape_type(ShapeType::Line)
            .x0(2)
            .y0(2)
            .x1(5)
            .y1(2)
            .line(
                ShapeLine::new()
                    .color(NamedColor::LightSeaGreen)
                    .width(3.)
                    .dash("dashdot"),
            ),
    );
    layout.add_shape(
        Shape::new()
            .shape_type(ShapeType::Line)
            .x0(4)
            .y0(0)
            .x1(6)
            .y1(2)
            .line(
                ShapeLine::new()
                    .color(NamedColor::MediumPurple)
                    .width(3.)
                    .dash("dot"),
            ),
    );

    plot.set_layout(layout);
    if show {
        plot.show();
    }
    println!(
        "{}",
        plot.to_inline_html(Some(
            "vertical_and_horizontal_lines_positioned_relative_to_axes"
        ))
    );
}
```
<div id="vertical_and_horizontal_lines_positioned_relative_to_axes" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("vertical_and_horizontal_lines_positioned_relative_to_axes")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"type":"scatter","mode":"text","x":[2.0,3.5,6.0],"x0":null,"dx":null,"y":[1.0,1.5,1.0],"y0":null,"dy":null,"text":["Vertical Line","Horizontal Dashed Line","Diagonal dotted Line"],"meta":null,"custom_data":null};
var data = [trace_0];
var layout = {"xaxis":{"range":[0.0,7.0]},"yaxis":{"range":[0.0,2.5]},"shapes":[{"type":"line","x0":1,"x1":1,"y0":0,"y1":2,"line":{"color":"royalblue","width":3.0}},{"type":"line","x0":2,"x1":5,"y0":2,"y1":2,"line":{"color":"lightseagreen","width":3.0,"dash":"dashdot"}},{"type":"line","x0":4,"x1":6,"y0":0,"y1":2,"line":{"color":"mediumpurple","width":3.0,"dash":"dot"}}]};
        Plotly.newPlot('vertical_and_horizontal_lines_positioned_relative_to_axes', data, layout, {"responsive": true});
    };
</script>


## Lines Positioned Relative to the Plot and to the Axes
```rust
fn lines_positioned_relative_to_the_plot_and_to_the_axes(show: bool) {
    let trace = Scatter::new(vec![2.0, 6.0], vec![1.0, 1.0])
        .text_array(vec![
            "Line positioned relative to the plot",
            "Line positioned relative to the axes",
        ])
        .mode(Mode::Text);
    let mut plot = Plot::new();
    plot.add_trace(trace);

    let mut layout = Layout::new()
        .x_axis(Axis::new().range(vec![0.0, 8.0]))
        .y_axis(Axis::new().range(vec![0.0, 2.]));

    layout.add_shape(
        Shape::new()
            .x_ref("x")
            .y_ref("y")
            .shape_type(ShapeType::Line)
            .x0(4)
            .y0(0)
            .x1(8)
            .y1(1)
            .line(ShapeLine::new().color(NamedColor::LightSeaGreen).width(3.)),
    );
    layout.add_shape(
        Shape::new()
            .x_ref("paper")
            .y_ref("paper")
            .shape_type(ShapeType::Line)
            .x0(0.0)
            .y0(0.0)
            .x1(0.5)
            .y1(0.5)
            .line(ShapeLine::new().color(NamedColor::DarkOrange).width(3.)),
    );

    plot.set_layout(layout);
    if show {
        plot.show();
    }
    println!(
        "{}",
        plot.to_inline_html(Some(
            "lines_positioned_relative_to_the_plot_and_to_the_axes"
        ))
    );
}
```
<div id="lines_positioned_relative_to_the_plot_and_to_the_axes" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("lines_positioned_relative_to_the_plot_and_to_the_axes")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"type":"scatter","mode":"text","x":[2.0,6.0],"x0":null,"dx":null,"y":[1.0,1.0],"y0":null,"dy":null,"text":["Line positioned relative to the plot","Line positioned relative to the axes"],"meta":null,"custom_data":null};
var data = [trace_0];
var layout = {"xaxis":{"range":[0.0,8.0]},"yaxis":{"range":[0.0,2.0]},"shapes":[{"type":"line","xref":"x","x0":4,"x1":8,"yref":"y","y0":0,"y1":1,"line":{"color":"lightseagreen","width":3.0}},{"type":"line","xref":"paper","x0":0.0,"x1":0.5,"yref":"paper","y0":0.0,"y1":0.5,"line":{"color":"darkorange","width":3.0}}]};
        Plotly.newPlot('lines_positioned_relative_to_the_plot_and_to_the_axes', data, layout, {"responsive": true});
    };
</script>


## Creating Tangent Lines with Shapes
```rust
fn creating_tangent_lines_with_shapes(show: bool) {
    let x0: Vec<f64> = linspace(1.0, 3.0, 200).collect();
    let y0: Vec<f64> = x0.iter().map(|v| *v * (v.powf(2.)).sin() + 1.).collect();

    let trace = Scatter::new(x0, y0);
    let mut plot = Plot::new();
    plot.add_trace(trace);

    let mut layout =
        Layout::new().title("$f(x)=x\\sin(x^2)+1\\\\ f\'(x)=\\sin(x^2)+2x^2\\cos(x^2)$".into());

    layout.add_shape(
        Shape::new()
            .x_ref("x")
            .y_ref("y")
            .opacity(0.7)
            .shape_type(ShapeType::Line)
            .x0(1.)
            .y0(2.30756)
            .x1(1.75)
            .y1(2.30756)
            .line(ShapeLine::new().color(NamedColor::Crimson).width(2.5)),
    );
    layout.add_shape(
        Shape::new()
            .x_ref("x")
            .y_ref("y")
            .opacity(0.7)
            .shape_type(ShapeType::Line)
            .x0(2.5)
            .y0(3.80796)
            .x1(3.05)
            .y1(3.80796)
            .line(ShapeLine::new().color(NamedColor::Crimson).width(2.5)),
    );
    layout.add_shape(
        Shape::new()
            .x_ref("x")
            .y_ref("y")
            .opacity(0.7)
            .shape_type(ShapeType::Line)
            .x0(1.90)
            .y0(-1.1827)
            .x1(2.5)
            .y1(-1.1827)
            .line(ShapeLine::new().color(NamedColor::Crimson).width(2.5)),
    );

    plot.set_layout(layout);
    if show {
        plot.show();
    }
    println!(
        "{}",
        plot.to_inline_html(Some("creating_tangent_lines_with_shapes"))
    );
}
```
<div id="creating_tangent_lines_with_shapes" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("creating_tangent_lines_with_shapes")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"type":"scatter","x":[1.0,1.0100502512562815,1.020100502512563,1.0301507537688441,1.0402010050251256,1.050251256281407,1.0603015075376885,1.07035175879397,1.0804020100502512,1.0904522613065326,1.100502512562814,1.1105527638190955,1.120603015075377,1.1306532663316582,1.1407035175879396,1.150753768844221,1.1608040201005025,1.170854271356784,1.1809045226130652,1.1909547738693467,1.2010050251256281,1.2110552763819096,1.221105527638191,1.2311557788944723,1.2412060301507537,1.2512562814070352,1.2613065326633166,1.271356783919598,1.2814070351758793,1.2914572864321607,1.3015075376884422,1.3115577889447236,1.321608040201005,1.3316582914572863,1.3417085427135678,1.3517587939698492,1.3618090452261307,1.3718592964824121,1.3819095477386933,1.391959798994975,1.4020100502512562,1.4120603015075377,1.4221105527638191,1.4321608040201004,1.442211055276382,1.4522613065326633,1.4623115577889447,1.4723618090452262,1.4824120603015074,1.492462311557789,1.5025125628140703,1.5125628140703518,1.5226130653266332,1.5326633165829144,1.542713567839196,1.5527638190954773,1.5628140703517588,1.5728643216080402,1.5829145728643215,1.5929648241206031,1.6030150753768844,1.6130653266331658,1.6231155778894473,1.6331658291457285,1.6432160804020102,1.6532663316582914,1.6633165829145728,1.6733668341708543,1.6834170854271355,1.6934673366834172,1.7035175879396984,1.7135678391959799,1.7236180904522613,1.7336683417085426,1.7437185929648242,1.7537688442211055,1.763819095477387,1.7738693467336684,1.7839195979899498,1.7939698492462313,1.8040201005025125,1.814070351758794,1.8241206030150754,1.8341708542713568,1.8442211055276383,1.8542713567839195,1.864321608040201,1.8743718592964824,1.8844221105527639,1.8944723618090453,1.9045226130653266,1.914572864321608,1.9246231155778895,1.934673366834171,1.9447236180904524,1.9547738693467336,1.964824120603015,1.9748743718592965,1.984924623115578,1.9949748743718594,2.0050251256281406,2.015075376884422,2.0251256281407035,2.035175879396985,2.0452261306532664,2.0552763819095476,2.065326633165829,2.0753768844221105,2.085427135678392,2.0954773869346734,2.1055276381909547,2.115577889447236,2.1256281407035176,2.1356783919597992,2.1457286432160805,2.1557788944723617,2.165829145728643,2.1758793969849246,2.1859296482412063,2.1959798994974875,2.2060301507537687,2.21608040201005,2.2261306532663316,2.2361809045226133,2.2462311557788945,2.2562814070351758,2.266331658291457,2.2763819095477387,2.2864321608040203,2.2964824120603016,2.306532663316583,2.316582914572864,2.3266331658291457,2.3366834170854274,2.3467336683417086,2.35678391959799,2.366834170854271,2.3768844221105527,2.3869346733668344,2.3969849246231156,2.407035175879397,2.417085427135678,2.4271356783919598,2.4371859296482414,2.4472361809045227,2.457286432160804,2.467336683417085,2.477386934673367,2.4874371859296485,2.4974874371859297,2.507537688442211,2.5175879396984926,2.527638190954774,2.5376884422110555,2.5477386934673367,2.557788944723618,2.5678391959798996,2.577889447236181,2.5879396984924625,2.5979899497487438,2.608040201005025,2.6180904522613067,2.628140703517588,2.6381909547738696,2.648241206030151,2.658291457286432,2.6683417085427137,2.678391959798995,2.6884422110552766,2.698492462311558,2.708542713567839,2.7185929648241207,2.728643216080402,2.7386934673366836,2.748743718592965,2.758793969849246,2.7688442211055277,2.778894472361809,2.7889447236180906,2.798994974874372,2.809045226130653,2.819095477386935,2.829145728643216,2.8391959798994977,2.849246231155779,2.85929648241206,2.869346733668342,2.879396984924623,2.8894472361809047,2.899497487437186,2.909547738693467,2.919597989949749,2.92964824120603,2.9396984924623117,2.949748743718593,2.959798994974874,2.969849246231156,2.9798994974874375,2.9899497487437188,3.0],"x0":null,"dx":null,"y":[1.8414709848078965,1.8607784276328483,1.8800512620305623,1.8992666593306657,1.9184009697996496,1.9374297235235876,1.9563276328292907,1.9750685963225296,1.993625704622662,2.0119712478735226,2.0300767251107716,2.0479128555660724,2.0654495919884006,2.082656136062539,2.099500956004328,2.11595180641149,2.131975750447878,2.1475391844377323,2.1626078649449854,2.1771469384108335,2.1911209734206265,2.204493995668672,2.217229525686742,2.229290619398891,2.240639911561702,2.251239662145147,2.261051805704984,2.270038003792901,2.2781597004455323,2.285378180787932,2.2916546327811407,2.2969502121370633,2.301226110417035,2.3044436263231254,2.3065642401834543,2.3075496916245353,2.3073620604149334,2.3059638504553153,2.303318076880271,2.299388356227129,2.2941389996163064,2.2875351088766402,2.2795426755375203,2.270128682597586,2.2592612089672324,2.2469095364692047,2.2330442592681603,2.217637395586279,2.2006625015477894,2.182094786980696,2.161911232989092,2.140090711094122,2.1166141037262145,2.0914644258352757,2.064626947369609,2.03608931635803,2.0058416823133074,1.9738768196586123,1.9401902508621034,1.9047803689482916,1.8676485590383842,1.828799318555415,1.788240375713885,1.7459828058976647,1.7020411455143418,1.6564335028989972,1.6091816658256142,1.5603112051701853,1.509851574255961,1.4578362033984422,1.4043025891557113,1.3492923777784434,1.2928514423438935,1.2350299530489595,1.175882440129598,1.1154678488672447,1.0538495861376218,0.9910955579536863,0.9272781974522257,0.8624744827732753,0.7967659442828741,0.7302386605929723,0.6629832428376667,0.5950948066723359,0.5266729314719791,0.45782160621704193,0.3886491615694512,0.3192681876585809,0.24979543711639252,0.18035171292331498,0.111061740651485,0.04205402471986164,-0.026539311693365875,-0.094582703401898,-0.16193733690983914,-0.22846136130155337,-0.29401011652370657,-0.3584363792140739,-0.42159062618236476,-0.48332131559567215,-0.5434751858655098,-0.6018975721745512,-0.6584327405192352,-0.712924239079507,-0.7652152666590397,-0.8151490578684908,-0.8625692846508299,-0.9073204736715978,-0.9492484390182874,-0.9882007295720208,-1.0240270903315563,-1.0565799368845332,-1.0857148421340703,-1.1112910343005078,-1.1331719051285911,-1.1512255271399638,-1.165325178679792,-1.1753498754150722,-1.1811849068509312,-1.1827223763404904,-1.1798617429739653,-1.1725103636440855,-1.1605840334980275,-1.1440075229013837,-1.122715108957657,-1.0966510995479317,-1.0657703477802034,-1.0300387546669039,-0.9894337577829553,-0.9439448035958211,-0.8935738011040155,-0.8383355543720372,-0.77825817150815,-0.71338344759762,-0.6437672190783159,-0.5694796870287797,-0.4906057068313754,-0.40724504167557884,-0.31951257737953553,-0.22753849603193776,-0.13146840599204235,-0.03146342583329775,0.07229978012362959,0.17962901696929856,0.2903166272495399,0.4041396139886223,0.5208598206703122,0.6402241700264706,0.761964963338897,0.8858002418015467,1.0114342113164576,1.1385577319080311,1.2668488727366554,1.3959735334740548,1.5255861325691806,1.6553303626858777,1.7848400133312639,1.9137398604181137,2.041646622215663,2.168169980841419,2.2929136681334708,2.415476614418012,2.535454158352352,2.6524393156800237,2.76602410438268,2.8758009233554755,2.981363981368525,3.082310772709632,3.178243595533468,3.2687711085716074,3.3535099214885222,3.4320862138018904,3.5041373769240614,3.5693136735269486,3.627279908087007,3.6777171021329975,3.720324167398606,3.754819569777558,3.7809429766927183,3.7984568802253,3.80714818810836,3.8068297744724098,3.797341982043058,3.778554067333325,3.7503655802492997,3.7127076694393977,3.665544304667209,3.608873407477859,3.542727881460324,3.4671765334853744,3.382324877422479,3.2883158120114144,3.1853301647865897,3.073587094226088,2.9533443426245354,2.824898332569492,2.688584100337109,2.5447750600135532,2.3938825926954994,2.2363554557252696],"y0":null,"dy":null,"meta":null,"custom_data":null};
var data = [trace_0];
var layout = {"title":{"text":"$f(x)=x\\sin(x^2)+1\\\\ f'(x)=\\sin(x^2)+2x^2\\cos(x^2)$"},"shapes":[{"type":"line","xref":"x","x0":1.0,"x1":1.75,"yref":"y","y0":2.30756,"y1":2.30756,"opacity":0.7,"line":{"color":"crimson","width":2.5}},{"type":"line","xref":"x","x0":2.5,"x1":3.05,"yref":"y","y0":3.80796,"y1":3.80796,"opacity":0.7,"line":{"color":"crimson","width":2.5}},{"type":"line","xref":"x","x0":1.9,"x1":2.5,"yref":"y","y0":-1.1827,"y1":-1.1827,"opacity":0.7,"line":{"color":"crimson","width":2.5}}]};
        Plotly.newPlot('creating_tangent_lines_with_shapes', data, layout, {"responsive": true});
    };
</script>


## Rectangles Positioned Relative to the Axes
```rust
fn rectangles_positioned_relative_to_the_axes(show: bool) {
    let trace = Scatter::new(vec![1.5, 4.5], vec![0.75, 0.75])
        .text_array(vec!["Unfilled Rectangle", "Filled Rectangle"])
        .mode(Mode::Text);
    let mut plot = Plot::new();
    plot.add_trace(trace);

    let mut layout = Layout::new()
        .x_axis(Axis::new().range(vec![0.0, 7.0]).show_grid(false))
        .y_axis(Axis::new().range(vec![0.0, 3.5]));

    layout.add_shape(
        Shape::new()
            .x_ref("x")
            .y_ref("y")
            .shape_type(ShapeType::Rect)
            .x0(1.)
            .y0(1.)
            .x1(2.)
            .y1(3.)
            .line(ShapeLine::new().color(NamedColor::RoyalBlue)),
    );
    layout.add_shape(
        Shape::new()
            .x_ref("x")
            .y_ref("y")
            .shape_type(ShapeType::Rect)
            .x0(3.)
            .y0(1.)
            .x1(6.)
            .y1(2.)
            .line(ShapeLine::new().color(NamedColor::RoyalBlue).width(2.))
            .fill_color(NamedColor::LightSkyBlue),
    );

    plot.set_layout(layout);
    if show {
        plot.show();
    }
    println!(
        "{}",
        plot.to_inline_html(Some("rectangles_positioned_relative_to_the_axes"))
    );
}
```
<div id="rectangles_positioned_relative_to_the_axes" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("rectangles_positioned_relative_to_the_axes")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"type":"scatter","mode":"text","x":[1.5,4.5],"x0":null,"dx":null,"y":[0.75,0.75],"y0":null,"dy":null,"text":["Unfilled Rectangle","Filled Rectangle"],"meta":null,"custom_data":null};
var data = [trace_0];
var layout = {"xaxis":{"range":[0.0,7.0],"showgrid":false},"yaxis":{"range":[0.0,3.5]},"shapes":[{"type":"rect","xref":"x","x0":1.0,"x1":2.0,"yref":"y","y0":1.0,"y1":3.0,"line":{"color":"royalblue"}},{"type":"rect","xref":"x","x0":3.0,"x1":6.0,"yref":"y","y0":1.0,"y1":2.0,"line":{"color":"royalblue","width":2.0},"fillcolor":"lightskyblue"}]};
        Plotly.newPlot('rectangles_positioned_relative_to_the_axes', data, layout, {"responsive": true});
    };
</script>


## Rectangle Positioned Relative to the Plot and to the Axes
```rust
fn rectangle_positioned_relative_to_the_plot_and_to_the_axes(show: bool) {
    let trace = Scatter::new(vec![1.5, 3.], vec![2.5, 2.5])
        .text_array(vec![
            "Rectangle reference to the plot",
            "Rectangle reference to the axes",
        ])
        .mode(Mode::Text);
    let mut plot = Plot::new();
    plot.add_trace(trace);

    let mut layout = Layout::new()
        .x_axis(Axis::new().range(vec![0.0, 4.0]).show_grid(false))
        .y_axis(Axis::new().range(vec![0.0, 4.0]));

    layout.add_shape(
        Shape::new()
            .x_ref("x")
            .y_ref("y")
            .shape_type(ShapeType::Rect)
            .x0(2.5)
            .y0(0.0)
            .x1(3.5)
            .y1(2.0)
            .line(ShapeLine::new().color(NamedColor::RoyalBlue).width(3.))
            .fill_color(NamedColor::LightSkyBlue),
    );
    layout.add_shape(
        Shape::new()
            .x_ref("paper")
            .y_ref("paper")
            .shape_type(ShapeType::Rect)
            .x0(0.25)
            .y0(0.0)
            .x1(0.5)
            .y1(0.5)
            .line(ShapeLine::new().color(NamedColor::LightSeaGreen).width(3.))
            .fill_color(NamedColor::PaleTurquoise),
    );

    plot.set_layout(layout);
    if show {
        plot.show();
    }
    println!(
        "{}",
        plot.to_inline_html(Some(
            "rectangle_positioned_relative_to_the_plot_and_to_the_axes"
        ))
    );
}
```
<div id="rectangle_positioned_relative_to_the_plot_and_to_the_axes" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("rectangle_positioned_relative_to_the_plot_and_to_the_axes")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"type":"scatter","mode":"text","x":[1.5,3.0],"x0":null,"dx":null,"y":[2.5,2.5],"y0":null,"dy":null,"text":["Rectangle reference to the plot","Rectangle reference to the axes"],"meta":null,"custom_data":null};
var data = [trace_0];
var layout = {"xaxis":{"range":[0.0,4.0],"showgrid":false},"yaxis":{"range":[0.0,4.0]},"shapes":[{"type":"rect","xref":"x","x0":2.5,"x1":3.5,"yref":"y","y0":0.0,"y1":2.0,"line":{"color":"royalblue","width":3.0},"fillcolor":"lightskyblue"},{"type":"rect","xref":"paper","x0":0.25,"x1":0.5,"yref":"paper","y0":0.0,"y1":0.5,"line":{"color":"lightseagreen","width":3.0},"fillcolor":"paleturquoise"}]};
        Plotly.newPlot('rectangle_positioned_relative_to_the_plot_and_to_the_axes', data, layout, {"responsive": true});
    };
</script>


## Highlighting Time Series Regions with Rectangle Shapes
```rust
fn highlighting_time_series_regions_with_rectangle_shapes(show: bool) {
    let x = vec![
        "2015-02-01",
        "2015-02-02",
        "2015-02-03",
        "2015-02-04",
        "2015-02-05",
        "2015-02-06",
        "2015-02-07",
        "2015-02-08",
        "2015-02-09",
        "2015-02-10",
        "2015-02-11",
        "2015-02-12",
        "2015-02-13",
        "2015-02-14",
        "2015-02-15",
        "2015-02-16",
        "2015-02-17",
        "2015-02-18",
        "2015-02-19",
        "2015-02-20",
        "2015-02-21",
        "2015-02-22",
        "2015-02-23",
        "2015-02-24",
        "2015-02-25",
        "2015-02-26",
        "2015-02-27",
        "2015-02-28",
    ];
    let y = vec![
        -14, -17, -8, -4, -7, -10, -12, -14, -12, -7, -11, -7, -18, -14, -14, -16, -13, -7, -8,
        -14, -8, -3, -9, -9, -4, -13, -9, -6,
    ];
    let trace = Scatter::new(x, y).mode(Mode::Lines).name("temperature");
    let mut plot = Plot::new();
    plot.add_trace(trace);

    let mut layout = Layout::new();

    layout.add_shape(
        Shape::new()
            .x_ref("x")
            .y_ref("paper")
            .shape_type(ShapeType::Rect)
            .x0("2015-02-04")
            .y0(0)
            .x1("2015-02-06")
            .y1(1)
            .fill_color(NamedColor::LightSalmon)
            .opacity(0.5)
            .layer(ShapeLayer::Below)
            .line(ShapeLine::new().width(0.)),
    );
    layout.add_shape(
        Shape::new()
            .x_ref("x")
            .y_ref("paper")
            .shape_type(ShapeType::Rect)
            .x0("2015-02-20")
            .y0(0)
            .x1("2015-02-22")
            .y1(1)
            .fill_color(NamedColor::LightSalmon)
            .opacity(0.5)
            .layer(ShapeLayer::Below)
            .line(ShapeLine::new().width(0.)),
    );

    plot.set_layout(layout);
    if show {
        plot.show();
    }
    println!(
        "{}",
        plot.to_inline_html(Some(
            "highlighting_time_series_regions_with_rectangle_shapes"
        ))
    );
}
```
<div id="highlighting_time_series_regions_with_rectangle_shapes" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("highlighting_time_series_regions_with_rectangle_shapes")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"type":"scatter","name":"temperature","mode":"lines","x":["2015-02-01","2015-02-02","2015-02-03","2015-02-04","2015-02-05","2015-02-06","2015-02-07","2015-02-08","2015-02-09","2015-02-10","2015-02-11","2015-02-12","2015-02-13","2015-02-14","2015-02-15","2015-02-16","2015-02-17","2015-02-18","2015-02-19","2015-02-20","2015-02-21","2015-02-22","2015-02-23","2015-02-24","2015-02-25","2015-02-26","2015-02-27","2015-02-28"],"x0":null,"dx":null,"y":[-14,-17,-8,-4,-7,-10,-12,-14,-12,-7,-11,-7,-18,-14,-14,-16,-13,-7,-8,-14,-8,-3,-9,-9,-4,-13,-9,-6],"y0":null,"dy":null,"meta":null,"custom_data":null};
var data = [trace_0];
var layout = {"shapes":[{"type":"rect","layer":"below","xref":"x","x0":"2015-02-04","x1":"2015-02-06","yref":"paper","y0":0,"y1":1,"opacity":0.5,"line":{"width":0.0},"fillcolor":"lightsalmon"},{"type":"rect","layer":"below","xref":"x","x0":"2015-02-20","x1":"2015-02-22","yref":"paper","y0":0,"y1":1,"opacity":0.5,"line":{"width":0.0},"fillcolor":"lightsalmon"}]};
        Plotly.newPlot('highlighting_time_series_regions_with_rectangle_shapes', data, layout, {"responsive": true});
    };
</script>


## Circles Positioned Relative to the Axes
```rust
fn circles_positioned_relative_to_the_axes(show: bool) {
    let trace = Scatter::new(vec![1.5, 3.5], vec![0.75, 2.5])
        .text_array(vec!["Unfilled Circle", "Filled Circle"])
        .mode(Mode::Text);
    let mut plot = Plot::new();
    plot.add_trace(trace);

    let mut layout = Layout::new()
        .x_axis(Axis::new().range(vec![0.0, 4.5]).zero_line(false))
        .y_axis(Axis::new().range(vec![0.0, 4.5]))
        .width(800)
        .height(800);

    layout.add_shape(
        Shape::new()
            .x_ref("x")
            .y_ref("y")
            .shape_type(ShapeType::Circle)
            .x0(1)
            .y0(1)
            .x1(3)
            .y1(3)
            .line(ShapeLine::new().color(NamedColor::LightSeaGreen)),
    );
    layout.add_shape(
        Shape::new()
            .x_ref("x")
            .y_ref("y")
            .shape_type(ShapeType::Circle)
            .x0(3)
            .y0(3)
            .x1(4)
            .y1(4)
            .line(ShapeLine::new().color(NamedColor::LightSeaGreen))
            .fill_color(NamedColor::PaleTurquoise),
    );

    plot.set_layout(layout);
    if show {
        plot.show();
    }
    println!(
        "{}",
        plot.to_inline_html(Some("circles_positioned_relative_to_the_axes"))
    );
}
```
<div id="circles_positioned_relative_to_the_axes" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("circles_positioned_relative_to_the_axes")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"type":"scatter","mode":"text","x":[1.5,3.5],"x0":null,"dx":null,"y":[0.75,2.5],"y0":null,"dy":null,"text":["Unfilled Circle","Filled Circle"],"meta":null,"custom_data":null};
var data = [trace_0];
var layout = {"width":800,"height":800,"xaxis":{"range":[0.0,4.5],"zeroline":false},"yaxis":{"range":[0.0,4.5]},"shapes":[{"type":"circle","xref":"x","x0":1,"x1":3,"yref":"y","y0":1,"y1":3,"line":{"color":"lightseagreen"}},{"type":"circle","xref":"x","x0":3,"x1":4,"yref":"y","y0":3,"y1":4,"line":{"color":"lightseagreen"},"fillcolor":"paleturquoise"}]};
        Plotly.newPlot('circles_positioned_relative_to_the_axes', data, layout, {"responsive": true});
    };
</script>


## Highlighting Clusters of Scatter Points with Circle Shapes
```rust
fn highlighting_clusters_of_scatter_points_with_circle_shapes(show: bool) {
    let rng = thread_rng();
    let x0 = Normal::new(2., 0.45)
        .unwrap()
        .sample_iter(rng)
        .take(300)
        .collect::<Vec<f64>>();
    let y0 = Normal::new(2., 0.45)
        .unwrap()
        .sample_iter(rng)
        .take(300)
        .collect::<Vec<f64>>();
    let x1 = Normal::new(6., 0.4)
        .unwrap()
        .sample_iter(rng)
        .take(300)
        .collect::<Vec<f64>>();
    let y1 = Normal::new(6., 0.4)
        .unwrap()
        .sample_iter(rng)
        .take(300)
        .collect::<Vec<f64>>();
    let x2 = Normal::new(4., 0.3)
        .unwrap()
        .sample_iter(rng)
        .take(300)
        .collect::<Vec<f64>>();
    let y2 = Normal::new(4., 0.3)
        .unwrap()
        .sample_iter(rng)
        .take(300)
        .collect::<Vec<f64>>();

    let x0min = x0.iter().copied().fold(f64::NAN, f64::min);
    let x0max = x0.iter().copied().fold(f64::NAN, f64::max);
    let y0min = y0.iter().copied().fold(f64::NAN, f64::min);
    let y0max = y0.iter().copied().fold(f64::NAN, f64::max);

    let x1min = x1.iter().copied().fold(f64::NAN, f64::min);
    let x1max = x1.iter().copied().fold(f64::NAN, f64::max);
    let y1min = y1.iter().copied().fold(f64::NAN, f64::min);

    let x2min = x2.iter().copied().fold(f64::NAN, f64::min);
    let x2max = x2.iter().copied().fold(f64::NAN, f64::max);
    let y2min = y2.iter().copied().fold(f64::NAN, f64::min);

    let mut plot = Plot::new();
    plot.add_trace(Scatter::new(x0, y0.clone()).mode(Mode::Markers));
    plot.add_trace(Scatter::new(x1.clone(), y1).mode(Mode::Markers));
    plot.add_trace(Scatter::new(x2, y2).mode(Mode::Markers));
    plot.add_trace(Scatter::new(x1, y0).mode(Mode::Markers));

    let mut layout = Layout::new().show_legend(false);

    layout.add_shape(
        Shape::new()
            .x_ref("x")
            .y_ref("y")
            .shape_type(ShapeType::Circle)
            .x0(x0min)
            .y0(y0min)
            .x1(x0max)
            .y1(y0max)
            .opacity(0.2)
            .fill_color(NamedColor::Blue)
            .line(ShapeLine::new().color(NamedColor::Blue)),
    );
    layout.add_shape(
        Shape::new()
            .x_ref("x")
            .y_ref("y")
            .shape_type(ShapeType::Circle)
            .x0(x1min)
            .y0(y1min)
            .x1(x1max)
            .y1(x1max)
            .opacity(0.2)
            .fill_color(NamedColor::Orange)
            .line(ShapeLine::new().color(NamedColor::Orange)),
    );
    layout.add_shape(
        Shape::new()
            .x_ref("x")
            .y_ref("y")
            .shape_type(ShapeType::Circle)
            .x0(x2min)
            .y0(y2min)
            .x1(x2max)
            .y1(x2max)
            .opacity(0.2)
            .fill_color(NamedColor::Green)
            .line(ShapeLine::new().color(NamedColor::Green)),
    );
    layout.add_shape(
        Shape::new()
            .x_ref("x")
            .y_ref("y")
            .shape_type(ShapeType::Circle)
            .x0(x1min)
            .y0(y0min)
            .x1(x1max)
            .y1(x0max)
            .opacity(0.2)
            .fill_color(NamedColor::Red)
            .line(ShapeLine::new().color(NamedColor::Red)),
    );

    plot.set_layout(layout);
    if show {
        plot.show();
    }
    println!(
        "{}",
        plot.to_inline_html(Some(
            "highlighting_clusters_of_scatter_points_with_circle_shapes"
        ))
    );
}
```
<div id="highlighting_clusters_of_scatter_points_with_circle_shapes" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("highlighting_clusters_of_scatter_points_with_circle_shapes")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"type":"scatter","mode":"markers","x":[2.3040629343622174,2.201452068628335,2.3561044135565923,1.635742812844836,1.9714359962009151,2.0281452972823373,2.181017041029233,2.032950054901583,2.4456556513283196,2.2943148821137362,1.4771582625686777,1.5511576289992672,1.3036501838869463,1.458456017870282,1.7234272446646821,1.5500900506344315,2.027417087417031,1.7395469687285756,2.6241637567344998,2.3750716168304393,2.001315255139244,1.4979511096337015,1.9326433614512752,1.993688226987977,1.8622575139259685,1.4952964107573887,2.3963333857217233,1.8279314735058,1.7794688055845005,1.570893276682536,2.264647209954227,1.7365641556222984,2.6100397614834767,1.5137384373463438,1.7068116227384058,2.220612073142193,2.155875625123354,1.9675117593466374,2.4146881629215797,1.704380578084932,2.6113944118315437,2.7397993070057884,1.6485255412417321,2.5108019882670094,2.0472660447936537,1.9670353909893439,2.4522206667260007,1.932125020659041,2.1660618355012558,1.691290414057371,2.3418133604829947,1.3563200371577862,1.8804654270434313,2.1322055047125263,1.4270035778747094,1.993966109282429,1.8278347150263403,1.6856693867617267,2.0015603214750857,2.202475822288695,1.886586482018346,1.9441702746222589,1.5786739607396563,1.6843329277689314,1.7725237811906267,1.1697438622330378,2.183696713685776,1.226603873126436,2.490601195713728,3.192673283827587,1.8385693644304775,1.2982428424083,2.0557377807074286,2.0160837700471865,2.727118202041087,1.6227141047633933,1.2886863697516961,1.4933702447997854,2.506913658814897,1.8151854253930655,2.1504271223373697,1.9018747438006012,2.085036451542614,2.423584301346584,2.2816926136295885,1.7155509641715279,2.246048909653354,1.825272004710993,2.7389852745697687,2.204978747755174,2.7126599457633693,2.1070291360649134,1.5384768642768576,2.607062874827738,2.0754449941902497,2.0943468722899707,1.7106687715264615,1.7513577606560253,2.434143238770282,1.7776586989439334,1.60938694542796,2.34991808137918,2.7234575871686126,1.6200111172547815,1.0525696087163876,2.1256512810955273,2.6649334116243253,1.706534577168671,2.3597555476962713,2.321281810282906,2.5131625501707204,1.8529729812798184,2.071196277087004,1.6638546730850838,0.9743260639316429,1.1073113219247481,1.6170208782653912,1.957533232396348,2.1172156517950484,1.5907610039009643,2.2359880616704095,1.629079618413452,1.9601462821786981,1.8605076021427662,2.108586495754939,1.9991452734763526,1.5747929214788434,2.831301189521951,2.557109726526643,1.9006052580859154,2.5847988609461754,2.1379772855965404,2.3227726296846996,2.2012408880958967,2.2866000730563805,2.3116211474123816,1.8157956404546536,2.1929269068696953,2.48828148310997,2.5279515878407466,1.6249184498729428,2.490505611150601,2.278388693517533,1.455942957412271,1.1683688641732601,1.058038405819901,3.078167810521676,1.8402877706194845,1.7182715249736114,2.18396160861856,1.7160024180051616,1.786047076234579,2.163544220517942,2.085684516380123,1.8587186983674258,2.9145146067123844,2.7573000333265676,2.6474325647069885,1.6258549732841816,2.079089958860958,2.2939888443786827,2.2133187061047974,1.349089197754017,1.0111361885725993,1.6386825020950873,1.4128024872666218,1.6277250352197523,1.4260375382670651,1.6244325532824961,2.5393557720436353,1.0348215190674435,2.3836066901034956,1.5881798996109764,1.7498006556349996,2.209991548456768,2.6913917738398707,1.468796640026926,1.4987535669740604,2.294072027191456,1.9135752403574011,2.298633810254906,1.9944110714463539,1.9546862814340937,2.76470436955663,1.4054633610549474,1.4090559299278396,1.7256759660784582,2.0978018616581657,2.411185385044081,2.027815984187851,2.3593563144299075,2.1251055808390307,1.5978100438491942,1.9415581127499582,1.9401626120200939,2.0536678538296522,1.783548911172051,2.19299649558047,2.1246910938482695,2.165694340913262,2.0248863731912436,2.103305210084109,2.1922485084375696,1.9455685022715776,1.8673733846474792,1.6684891968383284,1.5998928582483172,1.9807464813315863,2.3392891385019436,2.407217936613424,2.151795760751027,1.914037299041313,1.119469241212093,1.3104982518262902,2.825835379746117,1.782184088687865,1.9314942132604593,2.5128664389124844,2.2343371723927605,1.9606255408467372,1.855981110723373,1.4602246911789147,1.572982960085768,2.0351730593618687,2.538556683120473,2.7135195255436377,2.209091049601606,1.0321834543607045,1.641783709962169,2.9518536732048783,2.7172571011139834,2.342792502478302,2.2296763373904565,2.836840273383675,1.2424671597258516,1.7078477061229773,2.0295022435526304,2.404568853304047,2.148486285690808,1.9995997106433674,2.6887099161039063,1.3216351260146122,1.6990039917181792,2.485314272175076,2.6748230813239475,1.8311305217278782,1.7513881645340947,2.209000933791112,2.177818925952731,1.561196388143859,1.9948084088488522,1.872865148743326,2.276453369862166,2.2480311389195378,1.6918103567006044,2.2941453664676543,2.0598827408451603,1.5762972197021559,1.6546404755922177,1.93797575121412,2.3067262438293867,1.4687786851599736,2.681734632523946,1.8098684520921993,2.490558654174525,2.417954404402883,2.533685924807237,3.215663120996151,2.17039934488487,1.382419908347041,2.5606456977768337,1.5827859566506952,1.3776499432633766,2.359334007411967,1.3671677637949187,1.9704743614999538,2.166704946306173,3.429682751302849,1.6829015295796736,2.0487647637621653,2.3534974086469576,2.427163820858005,2.4768810691301684,2.106180445653672,2.2093914142279574,2.724499759461387,2.3488529320825116,2.6271239669666446,2.041283498270173,2.0860544914571406,1.9649458768316928,2.099383841967959,2.319593785260881,2.209497559422404,1.4498992903759287,1.99790969295641,2.0172750070536614,2.468437149000648,1.8246081554390883,1.7770656887061744],"x0":null,"dx":null,"y":[2.2866433232538994,2.3165526536190617,1.6574449362396473,1.3120128623045648,2.0924773449068264,1.8248987549194116,2.0240096752178234,2.096841985392658,2.5963340590897985,1.7232615553122237,1.6634099164184004,1.3685140018554645,0.9552721092363732,1.7539685787887271,2.240644507159723,1.395636774828422,1.949619022420305,1.7608560601662056,2.7510203934732864,1.9509042743498834,2.3097060533883305,2.5505743736031854,2.4057068756418567,0.9010091723793237,2.052820353031891,1.3795669007047437,2.027303690801306,1.3946008943457078,2.2200363072408367,2.157872269011385,1.495545875782697,1.4641978309491095,2.38899008416983,1.6755418541958433,2.3925135916990414,1.9701372591087218,1.8797631786520963,2.0996016851904895,2.2159565761633266,2.0403673207921815,1.436543163636959,2.450712880356252,1.8781225411323004,2.000245686348314,2.1803289454128865,2.2582573738325418,1.6839867086957363,1.186773998362829,1.3116911507260451,1.7753944996968505,2.3636638516231674,1.5242491263253453,2.4489801088726897,2.034968325243443,2.478315314073553,1.352019376025513,2.239648235614438,1.2648168488772114,1.8255955204083267,2.3935605929817365,2.5879061437953377,2.4340583821407584,1.3913026821002585,1.5275626622241079,1.0762741660919128,2.0396784136919397,1.5280366081690544,1.6331714349919693,1.5392470178852,1.404561582927423,1.8942601237479204,1.761442292500591,1.7612850309619548,2.145809144806645,1.9625105353298233,2.3404917155547635,1.4957228624851602,1.9143889738694566,2.2314424628802243,2.4444609685948087,2.6930901482066494,2.799820746330015,1.9493803765696276,2.119158158322922,2.3629904032352806,1.8974559313265627,1.0757410769995792,1.7234644890214867,2.099612880142534,2.3264104150103377,1.720511498612364,2.5238922206569083,2.176643473866142,1.280630434987171,2.0806663387287667,1.7779654905286493,1.0355392104590804,2.175786728744568,2.178551306727988,2.3614081079365525,2.1587977052905596,2.1777386683981903,1.654326069327369,1.6962439539095888,1.8207126556246405,1.5295483667416665,1.9843034326499838,2.220563872744388,2.4089545254230744,2.3065410199116636,2.079912757911088,1.4739078796481755,2.5585738731571976,2.283159816256541,2.0902090602478305,1.997409113251978,1.9577914796658966,2.6068868598190402,2.2715304952503725,2.348942691834095,2.187108013723824,1.796708790638894,2.2837768188996104,2.468109806118198,2.6579670027793547,1.9862401615777991,2.0037526601274744,2.302008993875127,1.4422664533503802,2.0716270853776964,2.1892479658099404,2.2633975384654192,2.0742278337275435,2.5134485125598656,2.9763036242960275,2.687657679554767,2.590341336141898,1.9137488151198159,1.6848894056701618,1.8585195919003636,1.9700764098197034,0.7650323254368607,2.3335584521310633,1.4042332113239522,1.368674725043177,2.1971649084656057,2.2717948949957645,2.787851674068224,1.925267170653974,2.6957027746805595,2.2419693077448692,1.5064136527348535,2.489085245663414,1.7445829849826595,2.0786913927943633,1.7228228117644262,1.8101984455141376,2.2212314331673104,1.9374165012600715,1.8423035041705698,2.2443953721006875,2.3824768987828446,1.9007490065108057,1.7410729244249972,2.5533589820583416,1.711640307905543,1.1500033530205858,1.9414600530459496,2.0021184109444805,2.045159226194435,1.758414472418954,1.3030467074685697,1.9681044427947827,2.902633465780605,1.88711129090728,2.4281504297843988,2.414827382626718,1.9718178683532328,2.214327131425496,1.4488104512224556,2.397766921822291,1.9694883027987382,1.700487841646637,2.807783514336518,1.9829578227189162,2.42459052976409,1.691653085417637,1.3397656714535875,2.3333907882292935,2.409380205591783,1.7157801429563548,2.379063509876681,1.4738710486181992,1.9752461719059433,2.118405204127554,2.3247805612085672,1.8526572447639702,2.8152520824555567,2.7273715716500373,1.9834267200879918,2.794850116637215,1.8670558385334508,2.2742785813236406,1.7458801664196013,1.7626168945818683,2.9466703497458497,1.7323648778419618,1.386793584566445,2.05160359213328,2.5060617354208503,1.6903420184303424,2.160494677512552,0.8949609921105413,1.8969869806398412,1.9797704546369717,2.166845556403765,1.534245991344163,2.2364893385675635,1.7665694456743053,2.2013057636499433,1.4600814534973254,2.0240709325986863,1.2228001662883003,2.259782970100123,2.3191428392551123,1.4650199502826249,2.461771470608095,2.1263049365481845,2.314805618362206,1.8943147419938204,1.1296263663547992,1.9446065497946434,1.9998031074158378,2.126967665243623,2.74232816379702,1.9488027715465277,1.6841288078484264,1.2594501868877575,2.385148729550052,1.8841813072151952,1.932883618369558,1.8897225012948686,2.243729976245335,1.3909162619528719,1.6079621987708825,1.6921934065400461,1.9854288373962028,2.1603605838493585,2.2366881873326037,2.532061943118065,2.5636661869273842,2.052940051488067,1.699404835888048,2.0866324501722064,1.5593579327243092,2.2459787174301433,1.4985251017520915,2.383974298050996,2.833627742835363,1.4773261165739848,2.778493445095284,2.369844643548506,2.1753440898552885,0.7502587426790785,1.8746770625085407,2.2836111761563274,2.5763461152273552,2.012550252920226,1.8500761734226523,1.931231236455906,1.9037813610961107,2.337076538713727,1.8166593243687805,2.457591973668897,1.9278941468137691,1.8141807071736211,1.9058050330093668,2.6704650754313546,2.286461449232473,2.3382427562704837,2.0156569803642563,2.3376774592171814,2.099362539319688,2.7488964443996466,1.802149627998307,1.9485328119422196,1.9293811563629144,2.0528974406144394,1.9939976567245579,1.9178142081848717,2.134579633840884,2.3570116427156127,1.9536862321909387,2.1598548343396433,1.3405530926146616,2.3838295092514015,2.4683149792938543,2.3846975710146396,1.6245988447658277,2.708869249753759],"y0":null,"dy":null,"meta":null,"custom_data":null};
var trace_1 = {"type":"scatter","mode":"markers","x":[6.308722685997051,6.502251789365028,5.88918697344314,5.651045834227919,6.097581753231741,6.220203840664942,5.741856692584831,5.6988376703307475,5.854233018795669,6.6923002834326795,6.165868186482678,5.898812899353039,6.009690242032855,6.5214743056825135,5.643092843421815,6.197933817403902,5.589966381206928,5.68737177510806,5.661385355771732,5.5294741525505176,5.989337935960583,6.573482724009413,5.578440692674201,5.983834156152197,5.470593428042466,5.506866010284034,5.619120940615113,5.429848785186272,6.372989614484155,5.975549735211117,6.199982488338754,5.041959030524709,6.177286991587404,5.687061373363964,5.9613186526958994,6.695217178545767,5.688229533347136,5.883164986043324,6.171396726735961,5.561740516297644,5.933524641996358,5.8851360802484,6.317884140143491,5.949238761054221,5.9556934222125415,6.845240853946498,6.119311311980994,5.67303404116566,6.01820340925983,6.019193845631682,5.853062804505309,6.177233898485447,6.05275650649308,6.4027588152123736,5.872759595086831,5.568178449852205,5.574113864385116,6.545417289992046,5.982492519973942,6.14022679438385,5.307052722693939,6.024056581894562,5.885426993401917,5.604920465203809,6.478203833623834,5.5801193878225535,5.964181620089111,5.825038448881022,6.470969727252356,6.690275961430597,6.340261323695891,6.1680314815094155,5.712511007366311,6.029727534382732,5.950189827785453,5.7533163735724075,5.619394089314227,6.658475102045628,5.617433738599531,6.079711160572003,6.051219136873816,6.406829263746278,6.122545978589449,6.139919383228536,5.640994520423226,6.455268577007872,5.3524715900501345,6.191373710110422,6.412616910849637,5.794115473022224,5.723721656523387,5.418821151097957,6.312671542346808,6.0984919858218545,6.457804822847217,5.990265220187552,5.2895427083348485,5.60325351674261,6.279694917010403,5.273123647771782,5.782603881006111,6.473875811930135,6.854128743667056,5.823769507808381,6.302010565065575,6.22393795475252,5.4476931821083845,5.851677380921418,6.333134788963646,5.97350479501587,5.835206484396524,6.015444889459258,5.6692014284727215,6.3245288499844445,6.133386130012535,6.465609258327318,5.68415035128529,5.5951479514257025,6.349283687684004,5.536659574109649,6.119715002021932,6.177215514474675,5.16524622848211,5.927596410514266,5.274687489945289,5.880428834838878,5.7399825804178946,5.865439350647876,6.0336212358680745,5.844939950233772,5.9719340643325864,5.830889700875347,5.6563215646854585,5.692858145996881,6.179169273913054,5.012057280409836,5.693439908126542,5.628666946941944,5.389484452947532,5.884504716651758,6.433159528076751,5.737738033259539,6.1613234296688075,5.604525474134623,6.582290493595416,6.043984611718284,6.721373815272867,5.807394091443582,6.060674084551221,5.937577917779946,5.994860706070712,5.934318398939375,6.0648023905058,6.132211717184896,5.534134161455575,5.733618684720557,5.907635151605134,5.521100768151793,5.902429219232239,5.805571737144912,6.018755084436062,6.156389151088956,6.201563379337708,5.839795321195474,5.731855046168489,6.054619299351211,5.956596823391647,6.200899906296633,5.613893059661445,5.4283135604642805,5.914841584081607,5.894444240318546,5.572541197452341,7.001639062120788,6.426081726078035,6.194984839176526,5.358746325518118,6.139572525611783,6.118089281597266,6.2448632287778985,6.114730205275098,6.0213564729009486,6.0867724427161605,6.087393290461635,5.857736519377893,5.9865763400731415,5.472014404533665,5.216595847527413,5.870331396362686,5.94917927113241,5.696477339150072,6.525879922866427,6.306346741056638,6.625487192371658,5.783995568325572,6.184980474556657,4.8316473991582,6.246586920219787,5.629838970595543,6.624316406527843,6.215176336037242,5.841436892709522,5.885702061165831,6.0444102687161845,6.217877661017253,5.5464419232686835,5.341427619964165,5.861192858159126,6.32646899210915,6.4538420242274865,5.937669250134014,6.415423390057996,6.140202998964434,5.253424456447398,6.14698760975651,6.415105058832625,5.403609618239983,4.965865892661018,5.918692140335063,5.534681144105795,6.456555998982255,5.75660813469196,6.025193419049279,6.449178770634854,5.555060086714305,5.969903452284985,6.869115281840231,5.243203486534008,5.945020850506689,5.544631383066248,6.568811741019184,6.10609407060747,6.281520288615711,5.993327257556314,6.325968859724003,5.783996371313243,5.635529191281052,6.1234819749791125,6.285378558398641,5.85982614015494,6.0050454097600845,5.643430009382203,5.720310729019959,6.197657754203956,5.274478683033324,5.874874983034057,5.463022689466365,5.953084123166693,6.2272642352691605,5.75296229435315,6.083502775963532,6.241405545217791,6.3687897564574305,5.77587345255501,6.67539828288354,6.138979339161767,5.888684035282223,4.951480244819417,5.7013293723570095,6.26775529194008,6.4619989638540325,6.056236566755767,6.758017743024892,6.24790801092764,5.953279214294294,5.891211591786086,5.729572671555567,6.458476572793394,6.169677946903115,5.8287577759772535,5.814961127543257,6.225349746530574,5.571183316373119,6.208981386773526,5.576363646846891,6.5465053050926265,5.9265475787937945,6.396021216372474,5.912883842370145,5.891135935648004,6.062484637606801,6.040807480275322,5.923737038429863,5.944849330372214,6.374450253226401,5.799030112605967,5.993786895634824,6.409030279833619,6.294496542715948,6.37159596275001,5.975559201473465,5.968732332914294,5.40203521956152,5.769234301943315,6.194514943660014,6.0614819760006595,5.8631658686949875,7.13299104924515,5.959185913935083,6.318803391207237],"x0":null,"dx":null,"y":[5.562693698110651,5.8806815760150135,6.347952471667173,5.6166059238858415,5.564276060104032,5.88143963119038,6.268164394420809,5.772610925775068,5.791506909360152,5.102300403870554,5.7225073857640485,5.444423868342472,6.189168744450049,5.979445681825634,5.886637140668922,6.609176764774235,6.798085988269334,6.407232397439213,6.920625406442092,5.821770950204982,5.356931863967583,6.153033398256987,6.360848566120914,5.567853201740942,6.089099288129309,5.6495057084429625,6.254876359536037,6.1652508632706535,5.985583589823475,6.857780519170896,6.030176092145813,6.417323087853047,6.351382403779591,5.873833185088708,6.121256172443999,5.784306476147933,5.548825660599939,6.258544359931933,6.345347623758158,6.453001031331005,6.299262062000514,6.3413421009447,5.608524881475139,5.6119632207806145,6.16014446612056,5.2225196460490935,6.4046731845179,5.8372298266604234,5.851245748436585,6.18578518797609,6.175649548448101,5.771773642793043,6.310152009439109,5.5838090069431106,6.203144923963364,6.249757981337,6.336702406967028,6.600091628291618,4.808787226539741,5.7399681548095725,6.0122233552919875,6.345676383365527,6.674318028689541,5.291669392833461,6.131782403899925,5.903830972673923,6.017805095551078,5.871568808672938,5.727026343087347,6.223120035459455,6.722439722950402,6.216572087012311,6.455939032658496,5.574897507930258,5.830652422189045,5.817707771864333,5.8869403019104665,5.669980303107302,6.191827272604268,6.314272775799567,5.7671171664145255,6.132088830969682,6.088953904366817,5.759371687087032,5.965974041319024,6.035452819945509,5.551781003005335,6.353621977027929,5.660856118264957,6.187550731744355,5.661819367619566,6.321452388071147,6.330305473949492,5.985433484706843,5.708907514152258,5.747891345580331,6.7070423820717,6.4763733636068626,6.562954542376427,6.183746978715694,6.1186894033137555,5.937188397063719,6.508535430838168,5.97159504458563,5.821585606697669,6.137497273520271,5.916461431966222,6.131682903644446,6.532628684102791,5.862844604690934,6.263387272773225,5.7686685552053145,6.1864684486303645,6.435172889779393,6.21451445009381,5.848335912181133,6.09941723962401,5.597540169759409,5.851121562904538,6.170086464471225,5.847498363092168,6.116570619078597,5.7518139264537576,6.195764916630021,6.050618580594501,5.535183146278744,5.369701830452334,5.696004666714477,6.121627441349143,5.184241400436852,6.598557295473257,5.720971821755952,6.357565375916571,5.873885936335471,5.777956458734501,7.096931988821421,6.417552651742981,5.523073676008621,6.370172022116721,5.6353422981159955,5.794508041481185,5.792692810148052,6.3337811327062425,5.252472689374919,5.688715002007803,6.439314514996491,6.5032399213234395,6.31816490890332,6.035232681805377,6.529834443904436,6.333552312868034,5.711500789163795,5.578930269290793,6.303567590844048,5.726960064002421,6.132158921095654,5.640655972544894,5.769915552751827,5.952172756976068,6.1428047600253315,6.396788845395694,6.1523082271951814,6.065506184910217,6.925896323174321,6.222755665968611,6.080758893671847,5.780333024460961,6.417726614956882,6.0704013703030535,5.8333498922474485,6.005826068943267,5.806255235555517,6.388241688971709,6.703091408855101,5.3435824295899454,6.307872342527595,6.5370932014833025,6.1215424139975,5.840542076486424,5.910136433639158,5.7377449436021974,6.29584230245788,5.457947118177441,6.044442557937299,5.84977826208899,6.127861447291887,5.88457277297606,5.980864555836258,5.933099562761137,6.178656203525912,5.700091785591213,6.2508823144621495,6.556958978405009,5.556971292687553,5.2223327518769285,5.676471276332977,6.791398279794034,5.907456791361106,6.290267422832057,5.895812931094804,6.357654494783234,5.953065332005018,5.045810027941309,5.93022667337635,5.791396263415782,6.175775412044528,6.455130688717615,6.597076241240986,6.733570515403739,6.715486525773787,5.967261271488715,6.071224836209708,5.837220144914023,6.179624247502417,5.917672642048402,5.91800713306871,5.155797866470633,5.7920433710201715,6.400213770901619,6.246461788974567,5.9242826336436485,5.986422076911196,6.180554733957763,6.133570326714844,5.061591678452558,5.581633969747974,5.075069698808297,6.386649197838186,5.8499698650390375,5.428817935408543,6.315353225348193,6.688946634887605,6.34780787112196,5.79397559409841,5.459230902424422,5.611511294016631,6.113861054473075,6.496351187517037,5.937297592703089,6.094868689119651,5.951685755937649,5.919889480982956,5.838181187810471,6.280375934371303,5.792001492900899,5.280305881240723,6.273665742910862,5.656265196027373,5.562920582918366,6.17652031357404,5.907614247282705,5.72760895035934,6.153811137035917,6.222464995113738,6.425613248867623,5.789508758096844,5.869134586564087,5.951351971287631,6.156218136149074,5.889684138784742,6.604107549346987,6.0312298063706065,4.970374098645252,6.0040219695463986,5.573323772200726,5.675835518082607,5.311381514843326,6.3362585896672226,6.249948150379814,5.36635226108182,6.030456256216539,6.01131759021939,6.047845627666505,5.747568769917577,6.330333749732801,5.499007722176053,6.396367185016521,5.418535695451455,6.1116965570682,6.577719750227766,5.945506263509842,5.889483014022686,6.073525100053058,6.214048152562865,6.626648660743588,6.158713206894228,5.871280861027809,6.3884497531033935,5.399884821662493,5.766575987818742,7.0450938433938175,5.39085213544675,5.739962527979618,5.728885810693576,6.090104203282471,6.545668761137673,5.92819566984616,5.399597187705076,5.779068530144187,5.7283404327759575],"y0":null,"dy":null,"meta":null,"custom_data":null};
var trace_2 = {"type":"scatter","mode":"markers","x":[3.8739558121958892,4.067950899504839,4.457203575250138,3.2204485829688485,3.6818459011963287,3.795986089478982,4.315394879932056,3.78163506345316,4.051556805873293,4.120284756347192,4.0221320552407995,3.8187521147499717,4.470817631496074,3.4594173617916057,3.4860305018455313,4.043825960712708,4.065051551288806,3.9560364695084016,3.6370431940957966,3.973902921304352,4.277833411301159,3.5628480770304045,4.559971841670569,3.7375287971273767,3.577720205009207,3.8503334120224877,4.0664824344374315,4.050945262173468,4.746439423117746,3.805557139762267,3.909558743704649,4.082697568982501,4.099004699776758,3.4924438415485546,4.2114084256689885,4.209712876736113,3.4875409189450353,4.1200019853290994,4.071436787571552,3.452548752562932,3.833638323251584,4.102206333823765,3.9303617984061887,3.2040411755679994,4.099807772995804,3.6838327971666853,4.236715370649733,3.6431512933112113,4.053146502065247,4.518862068474323,4.115415989084488,4.217472784688266,4.019233886821063,4.482953752905605,4.184670195116994,4.257956191470031,4.129661177184978,3.6236730189045554,4.059881185984125,4.555244035362777,4.257811780954301,4.536659827808619,3.7006067686611064,3.997455627851511,4.3114539523607505,4.178123982162425,3.9560108719917992,4.034955003010947,3.5265616742251935,3.7039230183804768,4.081826247729549,3.975738397679142,3.734578987395068,3.9607691102531786,4.075810488665334,4.053270417654452,3.679980359953505,3.555209653492414,3.8442550303176026,4.399608728963771,4.16675797546146,4.431801732739712,3.967648848673782,4.511042260401941,4.036503816644899,3.7506649591011385,4.146315984463921,4.296754502086059,4.6072163388467775,4.278071199068832,3.7714416667252335,3.5330501600419337,3.829027591680078,3.706819905181444,4.191773153216965,4.374849663862805,3.8839318968834937,4.495854897330654,3.6485504001601683,3.909172219365618,4.322473073316474,4.260907639517174,3.435828596678698,4.2506595641953,4.367815156998373,3.7226544157564208,4.337765274674302,4.004138505489969,3.911911385633973,4.1468362708947275,4.124921725842056,4.883685062937754,3.8428694122623415,3.566694981438427,3.923563713080099,3.8719234015291457,4.291414188030587,3.918459819569454,3.632639980134938,4.301423099038291,4.545913674583775,3.9083529080627395,3.669939159088221,3.7315973240127405,3.56309088470645,3.561936431961695,4.282984690234738,4.194516939310843,4.570165133693505,3.5498396480541037,3.732463463408755,3.7796195167261577,4.5379891786285445,3.6529487340874804,3.533910057319484,3.932861850814202,4.5033419883471995,3.880488001214112,3.6834219708444293,3.9008384284913733,3.896114798386338,4.1577390891604695,4.103219114044964,3.9397942276181546,3.916976950375739,3.8118831995089346,4.073201736493116,4.0403269470045995,3.967998778567706,4.014550033279473,3.823084889988087,3.887093010060513,4.413886934348139,3.8883738340331897,3.167853349229221,4.21178957138902,4.077453237769383,3.441633511723403,3.8908698192860505,4.36740124801718,3.884789728953264,4.367294046396066,4.3519637736896435,3.621923468138483,4.0286993031430045,3.98629930616948,3.6850634564796856,3.2718657922334895,3.8360169062684113,3.5791754513766394,3.8247844381863,4.036846097795505,3.8835910650129697,3.635970904555241,3.9864342725039714,4.139654711823688,3.758437253999043,4.233083589146447,4.1999120857418,4.000111958763355,3.5788658692890976,4.065230385054981,3.668666755557286,3.9196024108403464,3.8733934430108063,3.988070593193442,3.711496689870672,4.189575522358409,3.7526480456801057,3.6888237541914193,3.884793854806714,3.8583330287254998,3.566701328176728,4.124001226855171,4.024004394978448,3.6888232410826847,3.8190022375459076,3.793525398358536,3.429150414835781,4.0326970080800715,3.9433611972592053,3.652853454361055,3.7282991423666516,3.7449471799962026,3.7111987536763027,3.761054650844226,4.18158020183198,3.826034985399484,4.1857769419395074,3.94356232811484,4.0944007101459885,4.095518483658128,3.853497898835886,4.672876963400753,3.729387209067864,2.9167357675398944,3.972234985854343,3.701876037765423,4.144242536383661,4.707266177121137,3.8867817316528734,3.877267898564072,4.564818963656791,3.785090505324579,4.5369141136418945,3.9146934711463643,4.075360188128821,4.1132445760024385,3.8025408613427265,4.071247892557687,3.9067279066508456,4.561789038898684,4.5533665327297115,3.1607097356074076,3.6993865540519093,3.7698311329832195,3.846328286342992,3.86521088227676,4.359865706739426,3.3231993271947076,4.766493461941347,3.7375791686498925,4.173665941565298,4.084419801337875,4.221126951977036,4.527804939738726,3.639397460519537,3.4687116962684805,3.524335202063071,4.372610187352131,4.61387081251678,3.937549919468867,4.099112260442564,3.4217241851602025,4.295431815410704,3.880038850589525,4.299327311017746,4.069860722128728,3.7642210823933246,3.9486570253211406,4.408796957524859,3.8103580720580688,4.419661392500945,3.9891256731403373,4.214386791480341,3.605549947047617,4.29034217353776,4.341035424126356,3.700348725483492,3.86828509449673,3.809996128309052,4.447325886724454,3.8663631453555736,3.9494037284095125,4.19081455902957,3.877946076549092,3.824126376648716,4.164058567613384,4.359122053418396,3.930455549019569,3.6817439222093826,3.590652432990698,4.205273651909813,3.863911730381453,4.063319309511183,3.43172333914327,3.8397699204020204,3.755198893591484,4.031620815806295,4.183291033348384,4.014521106245113,3.846123215435104,4.111314389803013,3.8622432514108547,3.957407241380075,3.9802844593467266,3.615267793339985,3.337577767193152,3.7441944478036793,3.8315177362068544],"x0":null,"dx":null,"y":[4.0275267990537955,3.9096226598683197,3.890539605588714,4.495214731421809,3.8235487184202337,3.6822896570609425,3.67784335228612,4.019395360669695,3.9057374238262947,3.7951178510226375,4.217090118603046,3.3233502656267513,4.065513023964217,3.9029751307046503,4.094339852144404,4.2486110991831065,4.315947395592194,3.5425609573524963,3.593547890558792,3.7434963287490404,4.341128004321086,3.7877083240838996,3.8263638349469424,3.6592039599682424,3.7961972090870546,4.242359037163215,3.562215405619923,4.062110777057884,3.916117611955873,4.443361309449109,3.9919404006364934,3.585459453526703,3.7397352175994145,3.9761786595899116,4.444677985428062,3.9040729629502433,4.1223597419191265,4.583824580709144,4.604890286299285,4.029731059225271,4.372539543313559,3.9670958300361954,3.9601560345122833,3.7403127085252814,3.874857571211641,4.214918554970322,3.850687848534744,4.080454430265321,3.8716203464236725,3.764803103480253,4.042012050092041,3.4086467134046763,3.698494143140281,3.341334054051598,3.9075133109296805,4.03291769140822,4.173994213517435,4.087794045007136,3.724553452714659,3.800252371454545,4.069960144203329,3.9743366915705542,3.8865109597834007,3.7914076099316314,4.712366971004455,3.511497064283355,4.324303209018593,3.794498622273346,4.059561369185982,4.165399449572859,3.503883027722292,3.4460586666062842,4.64811409203189,4.063615862965805,4.060936158564928,3.90622773472335,3.973322690386656,3.6166304353643404,3.7889472097367665,4.07308698720792,4.016117834563269,3.8587896524018275,4.440412470667451,4.452330226409995,4.3895479209986865,4.040376036488693,3.762664555909056,3.9951791375135457,3.8644097404861704,3.993132768594351,4.436043297020085,4.089274048668782,3.7140719410221537,4.354033900031259,3.8170175346755895,4.225347996280396,4.148951359309314,4.057486558463826,4.169567133343889,4.2707230013212225,3.583585111451637,3.8408536346791906,3.9543797241233585,4.043043862167535,3.87920367220064,4.202448890545012,3.88870014335719,3.7245999816166435,3.39870977476252,3.879817810369356,4.345108942811804,3.7476874405016174,3.9800110678289746,4.190382541244568,3.978933794626144,4.326131406767627,4.133338931622502,4.401789579751579,4.243358088069235,3.782025993680337,3.834153137333591,3.7506246582268266,3.7304808655500734,3.5481693672721337,4.2282534625192865,3.9228649074589947,4.180627184068636,3.6617735913769502,3.703469122853414,4.189422038292135,4.372173489371682,3.685344392977953,3.8782201042939093,3.7498813659806585,3.559288304076674,3.8711582626731276,3.9098133940103166,4.3124228972747245,3.8298856889943704,4.029833474403548,3.9203089900838797,3.8247088271974925,4.191989013918532,3.863277097469071,4.039392330901807,3.973399944536935,3.7611009452481943,3.715613144549559,3.640173484503707,3.847791151993958,3.847374328892468,4.622193227048833,4.2485183683108225,4.011750642349928,3.556603210854589,3.4760475753820863,3.6842691232148814,3.763569513855511,4.017779847831155,4.829701386580436,4.322247455904268,4.788416645702763,3.9705618452047955,4.100102327508115,4.840161652868848,3.7064948590177362,3.999794497838538,4.291955743188253,3.9582312660207393,4.022792849957865,3.917025126244565,4.0144705709001185,3.3329859671851367,4.174748910909153,3.9901119680852943,4.142732695351239,4.384875040194963,4.280032110087708,4.0970335113672744,4.547347139338998,4.452622220051136,4.104821817879511,3.8610648929861258,4.55050952130778,4.2065507297965175,4.186655539095369,3.7832097272705316,3.8872444952457075,3.9226613920820532,4.110136913615743,3.583249986461213,3.8291754247660084,3.783603349073694,4.1939321500866935,4.373735268783002,4.274657640400975,4.007368554594204,3.5621343445366076,4.078243329181825,4.33359799193729,3.865283007760018,3.9229853076429198,4.127482268294995,4.069733550825201,3.7687771849271225,4.3515849706015866,3.5008447524209707,4.344443282116997,3.9899871899677537,4.107226920967054,4.052993644561878,4.322881294226135,4.283806436930283,4.438823527495998,4.06136828167999,4.1906737757431,3.528087150813501,3.936276398484699,4.274957381300008,4.244486620647379,3.8230801884890107,3.4159422352837114,3.914500571199522,3.913027239368326,4.037938672702208,3.846801112105833,3.8027952796756783,4.075759428032635,4.339148192899028,4.535941566920103,3.8329883121675103,3.9843795866370124,3.7044723462742875,3.6302554149218267,3.619361201464522,4.172896903728886,3.741522388236956,4.459447132098459,3.7097767775626336,3.9507646022919736,3.9129578270422143,4.279163088148629,3.9881391297370126,3.7854321956401646,3.8498921521421607,4.821472023273457,4.122497807128016,3.743748649127694,3.809854189065504,4.440895914601533,4.125955978115723,4.022819434397397,4.046365578941341,3.508362659380469,3.429714518292604,3.691690439917977,3.398390985481735,4.16461793616254,4.235218913923276,4.555135569648101,4.732890367399823,3.9627726156291643,4.619259221700407,4.477983006406277,3.5542766637741616,4.832843246786287,3.811317356980518,4.239530746095819,3.721016806107768,3.7452941093827765,3.8699042113248803,4.29559853806168,4.486234874554456,4.191848077023337,3.967686534806865,3.932415194620478,3.754415585108118,4.594626800780094,4.477786786927465,4.204601335517687,3.770443616720221,4.076977259365892,3.963408307774552,3.857272957790237,4.059323999828713,4.054299730466528,3.834508945326639,4.257916027514559,3.913466260541564,4.030326288799218,3.9044996701321155,4.119944303921481,3.921827508781602,3.9795359300093778,4.300177882714284,4.045989733060763,3.615270762204749,4.346921065488828,3.7086940234118675,3.900197892036203],"y0":null,"dy":null,"meta":null,"custom_data":null};
var trace_3 = {"type":"scatter","mode":"markers","x":[6.308722685997051,6.502251789365028,5.88918697344314,5.651045834227919,6.097581753231741,6.220203840664942,5.741856692584831,5.6988376703307475,5.854233018795669,6.6923002834326795,6.165868186482678,5.898812899353039,6.009690242032855,6.5214743056825135,5.643092843421815,6.197933817403902,5.589966381206928,5.68737177510806,5.661385355771732,5.5294741525505176,5.989337935960583,6.573482724009413,5.578440692674201,5.983834156152197,5.470593428042466,5.506866010284034,5.619120940615113,5.429848785186272,6.372989614484155,5.975549735211117,6.199982488338754,5.041959030524709,6.177286991587404,5.687061373363964,5.9613186526958994,6.695217178545767,5.688229533347136,5.883164986043324,6.171396726735961,5.561740516297644,5.933524641996358,5.8851360802484,6.317884140143491,5.949238761054221,5.9556934222125415,6.845240853946498,6.119311311980994,5.67303404116566,6.01820340925983,6.019193845631682,5.853062804505309,6.177233898485447,6.05275650649308,6.4027588152123736,5.872759595086831,5.568178449852205,5.574113864385116,6.545417289992046,5.982492519973942,6.14022679438385,5.307052722693939,6.024056581894562,5.885426993401917,5.604920465203809,6.478203833623834,5.5801193878225535,5.964181620089111,5.825038448881022,6.470969727252356,6.690275961430597,6.340261323695891,6.1680314815094155,5.712511007366311,6.029727534382732,5.950189827785453,5.7533163735724075,5.619394089314227,6.658475102045628,5.617433738599531,6.079711160572003,6.051219136873816,6.406829263746278,6.122545978589449,6.139919383228536,5.640994520423226,6.455268577007872,5.3524715900501345,6.191373710110422,6.412616910849637,5.794115473022224,5.723721656523387,5.418821151097957,6.312671542346808,6.0984919858218545,6.457804822847217,5.990265220187552,5.2895427083348485,5.60325351674261,6.279694917010403,5.273123647771782,5.782603881006111,6.473875811930135,6.854128743667056,5.823769507808381,6.302010565065575,6.22393795475252,5.4476931821083845,5.851677380921418,6.333134788963646,5.97350479501587,5.835206484396524,6.015444889459258,5.6692014284727215,6.3245288499844445,6.133386130012535,6.465609258327318,5.68415035128529,5.5951479514257025,6.349283687684004,5.536659574109649,6.119715002021932,6.177215514474675,5.16524622848211,5.927596410514266,5.274687489945289,5.880428834838878,5.7399825804178946,5.865439350647876,6.0336212358680745,5.844939950233772,5.9719340643325864,5.830889700875347,5.6563215646854585,5.692858145996881,6.179169273913054,5.012057280409836,5.693439908126542,5.628666946941944,5.389484452947532,5.884504716651758,6.433159528076751,5.737738033259539,6.1613234296688075,5.604525474134623,6.582290493595416,6.043984611718284,6.721373815272867,5.807394091443582,6.060674084551221,5.937577917779946,5.994860706070712,5.934318398939375,6.0648023905058,6.132211717184896,5.534134161455575,5.733618684720557,5.907635151605134,5.521100768151793,5.902429219232239,5.805571737144912,6.018755084436062,6.156389151088956,6.201563379337708,5.839795321195474,5.731855046168489,6.054619299351211,5.956596823391647,6.200899906296633,5.613893059661445,5.4283135604642805,5.914841584081607,5.894444240318546,5.572541197452341,7.001639062120788,6.426081726078035,6.194984839176526,5.358746325518118,6.139572525611783,6.118089281597266,6.2448632287778985,6.114730205275098,6.0213564729009486,6.0867724427161605,6.087393290461635,5.857736519377893,5.9865763400731415,5.472014404533665,5.216595847527413,5.870331396362686,5.94917927113241,5.696477339150072,6.525879922866427,6.306346741056638,6.625487192371658,5.783995568325572,6.184980474556657,4.8316473991582,6.246586920219787,5.629838970595543,6.624316406527843,6.215176336037242,5.841436892709522,5.885702061165831,6.0444102687161845,6.217877661017253,5.5464419232686835,5.341427619964165,5.861192858159126,6.32646899210915,6.4538420242274865,5.937669250134014,6.415423390057996,6.140202998964434,5.253424456447398,6.14698760975651,6.415105058832625,5.403609618239983,4.965865892661018,5.918692140335063,5.534681144105795,6.456555998982255,5.75660813469196,6.025193419049279,6.449178770634854,5.555060086714305,5.969903452284985,6.869115281840231,5.243203486534008,5.945020850506689,5.544631383066248,6.568811741019184,6.10609407060747,6.281520288615711,5.993327257556314,6.325968859724003,5.783996371313243,5.635529191281052,6.1234819749791125,6.285378558398641,5.85982614015494,6.0050454097600845,5.643430009382203,5.720310729019959,6.197657754203956,5.274478683033324,5.874874983034057,5.463022689466365,5.953084123166693,6.2272642352691605,5.75296229435315,6.083502775963532,6.241405545217791,6.3687897564574305,5.77587345255501,6.67539828288354,6.138979339161767,5.888684035282223,4.951480244819417,5.7013293723570095,6.26775529194008,6.4619989638540325,6.056236566755767,6.758017743024892,6.24790801092764,5.953279214294294,5.891211591786086,5.729572671555567,6.458476572793394,6.169677946903115,5.8287577759772535,5.814961127543257,6.225349746530574,5.571183316373119,6.208981386773526,5.576363646846891,6.5465053050926265,5.9265475787937945,6.396021216372474,5.912883842370145,5.891135935648004,6.062484637606801,6.040807480275322,5.923737038429863,5.944849330372214,6.374450253226401,5.799030112605967,5.993786895634824,6.409030279833619,6.294496542715948,6.37159596275001,5.975559201473465,5.968732332914294,5.40203521956152,5.769234301943315,6.194514943660014,6.0614819760006595,5.8631658686949875,7.13299104924515,5.959185913935083,6.318803391207237],"x0":null,"dx":null,"y":[2.2866433232538994,2.3165526536190617,1.6574449362396473,1.3120128623045648,2.0924773449068264,1.8248987549194116,2.0240096752178234,2.096841985392658,2.5963340590897985,1.7232615553122237,1.6634099164184004,1.3685140018554645,0.9552721092363732,1.7539685787887271,2.240644507159723,1.395636774828422,1.949619022420305,1.7608560601662056,2.7510203934732864,1.9509042743498834,2.3097060533883305,2.5505743736031854,2.4057068756418567,0.9010091723793237,2.052820353031891,1.3795669007047437,2.027303690801306,1.3946008943457078,2.2200363072408367,2.157872269011385,1.495545875782697,1.4641978309491095,2.38899008416983,1.6755418541958433,2.3925135916990414,1.9701372591087218,1.8797631786520963,2.0996016851904895,2.2159565761633266,2.0403673207921815,1.436543163636959,2.450712880356252,1.8781225411323004,2.000245686348314,2.1803289454128865,2.2582573738325418,1.6839867086957363,1.186773998362829,1.3116911507260451,1.7753944996968505,2.3636638516231674,1.5242491263253453,2.4489801088726897,2.034968325243443,2.478315314073553,1.352019376025513,2.239648235614438,1.2648168488772114,1.8255955204083267,2.3935605929817365,2.5879061437953377,2.4340583821407584,1.3913026821002585,1.5275626622241079,1.0762741660919128,2.0396784136919397,1.5280366081690544,1.6331714349919693,1.5392470178852,1.404561582927423,1.8942601237479204,1.761442292500591,1.7612850309619548,2.145809144806645,1.9625105353298233,2.3404917155547635,1.4957228624851602,1.9143889738694566,2.2314424628802243,2.4444609685948087,2.6930901482066494,2.799820746330015,1.9493803765696276,2.119158158322922,2.3629904032352806,1.8974559313265627,1.0757410769995792,1.7234644890214867,2.099612880142534,2.3264104150103377,1.720511498612364,2.5238922206569083,2.176643473866142,1.280630434987171,2.0806663387287667,1.7779654905286493,1.0355392104590804,2.175786728744568,2.178551306727988,2.3614081079365525,2.1587977052905596,2.1777386683981903,1.654326069327369,1.6962439539095888,1.8207126556246405,1.5295483667416665,1.9843034326499838,2.220563872744388,2.4089545254230744,2.3065410199116636,2.079912757911088,1.4739078796481755,2.5585738731571976,2.283159816256541,2.0902090602478305,1.997409113251978,1.9577914796658966,2.6068868598190402,2.2715304952503725,2.348942691834095,2.187108013723824,1.796708790638894,2.2837768188996104,2.468109806118198,2.6579670027793547,1.9862401615777991,2.0037526601274744,2.302008993875127,1.4422664533503802,2.0716270853776964,2.1892479658099404,2.2633975384654192,2.0742278337275435,2.5134485125598656,2.9763036242960275,2.687657679554767,2.590341336141898,1.9137488151198159,1.6848894056701618,1.8585195919003636,1.9700764098197034,0.7650323254368607,2.3335584521310633,1.4042332113239522,1.368674725043177,2.1971649084656057,2.2717948949957645,2.787851674068224,1.925267170653974,2.6957027746805595,2.2419693077448692,1.5064136527348535,2.489085245663414,1.7445829849826595,2.0786913927943633,1.7228228117644262,1.8101984455141376,2.2212314331673104,1.9374165012600715,1.8423035041705698,2.2443953721006875,2.3824768987828446,1.9007490065108057,1.7410729244249972,2.5533589820583416,1.711640307905543,1.1500033530205858,1.9414600530459496,2.0021184109444805,2.045159226194435,1.758414472418954,1.3030467074685697,1.9681044427947827,2.902633465780605,1.88711129090728,2.4281504297843988,2.414827382626718,1.9718178683532328,2.214327131425496,1.4488104512224556,2.397766921822291,1.9694883027987382,1.700487841646637,2.807783514336518,1.9829578227189162,2.42459052976409,1.691653085417637,1.3397656714535875,2.3333907882292935,2.409380205591783,1.7157801429563548,2.379063509876681,1.4738710486181992,1.9752461719059433,2.118405204127554,2.3247805612085672,1.8526572447639702,2.8152520824555567,2.7273715716500373,1.9834267200879918,2.794850116637215,1.8670558385334508,2.2742785813236406,1.7458801664196013,1.7626168945818683,2.9466703497458497,1.7323648778419618,1.386793584566445,2.05160359213328,2.5060617354208503,1.6903420184303424,2.160494677512552,0.8949609921105413,1.8969869806398412,1.9797704546369717,2.166845556403765,1.534245991344163,2.2364893385675635,1.7665694456743053,2.2013057636499433,1.4600814534973254,2.0240709325986863,1.2228001662883003,2.259782970100123,2.3191428392551123,1.4650199502826249,2.461771470608095,2.1263049365481845,2.314805618362206,1.8943147419938204,1.1296263663547992,1.9446065497946434,1.9998031074158378,2.126967665243623,2.74232816379702,1.9488027715465277,1.6841288078484264,1.2594501868877575,2.385148729550052,1.8841813072151952,1.932883618369558,1.8897225012948686,2.243729976245335,1.3909162619528719,1.6079621987708825,1.6921934065400461,1.9854288373962028,2.1603605838493585,2.2366881873326037,2.532061943118065,2.5636661869273842,2.052940051488067,1.699404835888048,2.0866324501722064,1.5593579327243092,2.2459787174301433,1.4985251017520915,2.383974298050996,2.833627742835363,1.4773261165739848,2.778493445095284,2.369844643548506,2.1753440898552885,0.7502587426790785,1.8746770625085407,2.2836111761563274,2.5763461152273552,2.012550252920226,1.8500761734226523,1.931231236455906,1.9037813610961107,2.337076538713727,1.8166593243687805,2.457591973668897,1.9278941468137691,1.8141807071736211,1.9058050330093668,2.6704650754313546,2.286461449232473,2.3382427562704837,2.0156569803642563,2.3376774592171814,2.099362539319688,2.7488964443996466,1.802149627998307,1.9485328119422196,1.9293811563629144,2.0528974406144394,1.9939976567245579,1.9178142081848717,2.134579633840884,2.3570116427156127,1.9536862321909387,2.1598548343396433,1.3405530926146616,2.3838295092514015,2.4683149792938543,2.3846975710146396,1.6245988447658277,2.708869249753759],"y0":null,"dy":null,"meta":null,"custom_data":null};
var data = [trace_0,trace_1,trace_2,trace_3];
var layout = {"showlegend":false,"shapes":[{"type":"circle","xref":"x","x0":0.9743260639316429,"x1":3.429682751302849,"yref":"y","y0":0.7502587426790785,"y1":2.9763036242960275,"opacity":0.2,"line":{"color":"blue"},"fillcolor":"blue"},{"type":"circle","xref":"x","x0":4.8316473991582,"x1":7.13299104924515,"yref":"y","y0":4.808787226539741,"y1":7.13299104924515,"opacity":0.2,"line":{"color":"orange"},"fillcolor":"orange"},{"type":"circle","xref":"x","x0":2.9167357675398944,"x1":4.883685062937754,"yref":"y","y0":3.3233502656267513,"y1":4.883685062937754,"opacity":0.2,"line":{"color":"green"},"fillcolor":"green"},{"type":"circle","xref":"x","x0":4.8316473991582,"x1":7.13299104924515,"yref":"y","y0":0.7502587426790785,"y1":3.429682751302849,"opacity":0.2,"line":{"color":"red"},"fillcolor":"red"}]};
        Plotly.newPlot('highlighting_clusters_of_scatter_points_with_circle_shapes', data, layout, {"responsive": true});
    };
</script>


## Venn Diagram with Circle Shapes
```rust
fn venn_diagram_with_circle_shapes(show: bool) {
    let mut plot = Plot::new();
    plot.add_trace(
        Scatter::new(vec![1., 1.75, 2.5], vec![1., 1., 1.])
            .text_array(vec!["$A$", "$A+B$", "$B$"])
            .mode(Mode::Text)
            .text_font(
                Font::new()
                    .color(NamedColor::Black)
                    .size(18)
                    .family("Arial"),
            ),
    );

    let mut layout = Layout::new()
        .x_axis(
            Axis::new()
                .zero_line(false)
                .show_grid(false)
                .show_tick_labels(false),
        )
        .y_axis(
            Axis::new()
                .zero_line(false)
                .show_grid(false)
                .show_tick_labels(false),
        )
        .margin(Margin::new().left(20).right(20).bottom(100))
        .height(600)
        .width(800)
        .plot_background_color(NamedColor::White);

    layout.add_shape(
        Shape::new()
            .x_ref("x")
            .y_ref("y")
            .shape_type(ShapeType::Circle)
            .x0(0)
            .y0(0)
            .x1(2)
            .y1(2)
            .opacity(0.3)
            .layer(ShapeLayer::Below)
            .fill_color(NamedColor::Blue)
            .line(ShapeLine::new().color(NamedColor::Blue)),
    );
    layout.add_shape(
        Shape::new()
            .x_ref("x")
            .y_ref("y")
            .shape_type(ShapeType::Circle)
            .x0(1.5)
            .y0(0.)
            .x1(3.5)
            .y1(2.)
            .opacity(0.3)
            .layer(ShapeLayer::Below)
            .fill_color(NamedColor::Gray)
            .line(ShapeLine::new().color(NamedColor::Gray)),
    );

    plot.set_layout(layout);
    if show {
        plot.show();
    }
    println!(
        "{}",
        plot.to_inline_html(Some("venn_diagram_with_circle_shapes"))
    );
}
```
<div id="venn_diagram_with_circle_shapes" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("venn_diagram_with_circle_shapes")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"type":"scatter","mode":"text","x":[1.0,1.75,2.5],"x0":null,"dx":null,"y":[1.0,1.0,1.0],"y0":null,"dy":null,"text":["$A$","$A+B$","$B$"],"meta":null,"custom_data":null,"textfont":{"family":"Arial","size":18,"color":"black"}};
var data = [trace_0];
var layout = {"margin":{"l":20,"r":20,"b":100},"width":800,"height":600,"plot_bgcolor":"white","xaxis":{"showticklabels":false,"showgrid":false,"zeroline":false},"yaxis":{"showticklabels":false,"showgrid":false,"zeroline":false},"shapes":[{"type":"circle","layer":"below","xref":"x","x0":0,"x1":2,"yref":"y","y0":0,"y1":2,"opacity":0.3,"line":{"color":"blue"},"fillcolor":"blue"},{"type":"circle","layer":"below","xref":"x","x0":1.5,"x1":3.5,"yref":"y","y0":0.0,"y1":2.0,"opacity":0.3,"line":{"color":"gray"},"fillcolor":"gray"}]};
        Plotly.newPlot('venn_diagram_with_circle_shapes', data, layout, {"responsive": true});
    };
</script>


## Adding Shapes to Subplots
```rust
fn adding_shapes_to_subplots(show: bool) {
    let mut plot = Plot::new();
    plot.add_trace(
        Scatter::new(vec![2, 6], vec![1, 1])
            .x_axis("x1")
            .y_axis("y1"),
    );
    plot.add_trace(
        Bar::new(vec![1, 2, 3], vec![4, 5, 6])
            .x_axis("x2")
            .y_axis("y2"),
    );
    plot.add_trace(
        Scatter::new(vec![10, 20], vec![40, 50])
            .x_axis("x3")
            .y_axis("y3"),
    );
    plot.add_trace(
        Bar::new(vec![11, 13, 15], vec![8, 11, 20])
            .x_axis("x4")
            .y_axis("y4"),
    );

    let mut layout = Layout::new()
        .grid(
            LayoutGrid::new()
                .rows(2)
                .columns(2)
                .pattern(GridPattern::Independent),
        )
        .x_axis(Axis::new().domain(&[0.0, 0.48]).anchor("x1"))
        .y_axis(Axis::new().domain(&[0.52, 1.]).anchor("y1"))
        .x_axis2(Axis::new().domain(&[0.52, 1.0]).anchor("x2"))
        .y_axis2(Axis::new().domain(&[0.5, 1.]).anchor("y2"))
        .x_axis3(Axis::new().domain(&[0.0, 0.48]).anchor("x3"))
        .y_axis3(Axis::new().domain(&[0.0, 0.48]).anchor("y3"))
        .x_axis4(Axis::new().domain(&[0.52, 1.0]).anchor("x4"))
        .y_axis4(Axis::new().domain(&[0.0, 0.48]).anchor("y4"));

    layout.add_shape(
        Shape::new()
            .x_ref("x1")
            .y_ref("y1")
            .shape_type(ShapeType::Line)
            .x0(3)
            .y0(0.5)
            .x1(5)
            .y1(0.8)
            .line(ShapeLine::new().width(3.)),
    );
    layout.add_shape(
        Shape::new()
            .x_ref("x2")
            .y_ref("y2")
            .shape_type(ShapeType::Rect)
            .x0(4)
            .y0(2)
            .x1(5)
            .y1(6),
    );
    layout.add_shape(
        Shape::new()
            .x_ref("x3")
            .y_ref("y3")
            .shape_type(ShapeType::Rect)
            .x0(10)
            .y0(20)
            .x1(15)
            .y1(30),
    );
    layout.add_shape(
        Shape::new()
            .x_ref("x4")
            .y_ref("y4")
            .shape_type(ShapeType::Circle)
            .x0(5)
            .y0(12)
            .x1(10)
            .y1(18),
    );

    plot.set_layout(layout);
    if show {
        plot.show();
    }
    println!("{}", plot.to_inline_html(Some("adding_shapes_to_subplots")));
}
```
<div id="adding_shapes_to_subplots" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("adding_shapes_to_subplots")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"type":"scatter","x":[2,6],"x0":null,"dx":null,"y":[1,1],"y0":null,"dy":null,"meta":null,"custom_data":null,"xaxis":"x1","yaxis":"y1"};
var trace_1 = {"x":[1,2,3],"y":[4,5,6],"type":"bar","xaxis":"x2","yaxis":"y2"};
var trace_2 = {"type":"scatter","x":[10,20],"x0":null,"dx":null,"y":[40,50],"y0":null,"dy":null,"meta":null,"custom_data":null,"xaxis":"x3","yaxis":"y3"};
var trace_3 = {"x":[11,13,15],"y":[8,11,20],"type":"bar","xaxis":"x4","yaxis":"y4"};
var data = [trace_0,trace_1,trace_2,trace_3];
var layout = {"grid":{"rows":2,"columns":2,"pattern":"independent"},"xaxis":{"anchor":"x1","domain":[0.0,0.48]},"yaxis":{"anchor":"y1","domain":[0.52,1.0]},"xaxis2":{"anchor":"x2","domain":[0.52,1.0]},"yaxis2":{"anchor":"y2","domain":[0.5,1.0]},"xaxis3":{"anchor":"x3","domain":[0.0,0.48]},"yaxis3":{"anchor":"y3","domain":[0.0,0.48]},"xaxis4":{"anchor":"x4","domain":[0.52,1.0]},"yaxis4":{"anchor":"y4","domain":[0.0,0.48]},"shapes":[{"type":"line","xref":"x1","x0":3,"x1":5,"yref":"y1","y0":0.5,"y1":0.8,"line":{"width":3.0}},{"type":"rect","xref":"x2","x0":4,"x1":5,"yref":"y2","y0":2,"y1":6},{"type":"rect","xref":"x3","x0":10,"x1":15,"yref":"y3","y0":20,"y1":30},{"type":"circle","xref":"x4","x0":5,"x1":10,"yref":"y4","y0":12,"y1":18}]};
        Plotly.newPlot('adding_shapes_to_subplots', data, layout, {"responsive": true});
    };
</script>


## SVG Paths
```rust
fn svg_paths(show: bool) {
    let mut plot = Plot::new();
    plot.add_trace(
        Scatter::new(vec![2, 1, 8, 8], vec![0.25, 9., 2., 6.])
            .text_array(vec![
                "Filled Triangle",
                "Filled Polygon",
                "Quadratic Bezier Curves",
                "Cubic Bezier Curves",
            ])
            .mode(Mode::Text),
    );

    let mut layout = Layout::new()
        .x_axis(
            Axis::new()
                .domain(&[0.05, 0.95])
                .range(vec![0., 9.])
                .zero_line(false),
        )
        .y_axis(
            Axis::new()
                .domain(&[0.05, 0.95])
                .range(vec![0, 11])
                .zero_line(false),
        );
    layout.add_shape(
        Shape::new()
            .shape_type(ShapeType::Path)
            .path("M 4,4 Q 6,0 8,4")
            .line(ShapeLine::new().color(NamedColor::RoyalBlue)),
    );
    layout.add_shape(
        Shape::new()
            .shape_type(ShapeType::Path)
            .path("M 1,4 C 2,8 6,4 8,8")
            .line(ShapeLine::new().color(NamedColor::MediumPurple)),
    );
    layout.add_shape(
        Shape::new()
            .shape_type(ShapeType::Path)
            .path("M 1 1 L 1 3 L 4 1 Z")
            .fill_color(NamedColor::LightPink)
            .line(ShapeLine::new().color(NamedColor::Crimson)),
    );
    layout.add_shape(
        Shape::new()
            .shape_type(ShapeType::Path)
            .path("M 3,7 L2,8 L2,9 L3,10, L4,10 L5,9 L5,8 L4,7 Z")
            .fill_color(NamedColor::PaleTurquoise)
            .line(ShapeLine::new().color(NamedColor::LightSeaGreen)),
    );

    plot.set_layout(layout);
    if show {
        plot.show();
    }
    println!("{}", plot.to_inline_html(Some("svg_paths")));
}
```
<div id="svg_paths" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("svg_paths")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"type":"scatter","mode":"text","x":[2,1,8,8],"y":[0.25,9.0,2.0,6.0],"text":["Filled Triangle","Filled Polygon","Quadratic Bezier Curves","Cubic Bezier Curves"]};
var data = [trace_0];
var layout = {"xaxis":{"range":[0.0,9.0],"zeroline":false,"domain":[0.05,0.95]},"yaxis":{"range":[0,11],"zeroline":false,"domain":[0.05,0.95]},"shapes":[{"type":"path","path":"M 4,4 Q 6,0 8,4","line":{"color":"royalblue"}},{"type":"path","path":"M 1,4 C 2,8 6,4 8,8","line":{"color":"mediumpurple"}},{"type":"path","path":"M 1 1 L 1 3 L 4 1 Z","line":{"color":"crimson"},"fillcolor":"lightpink"},{"type":"path","path":"M 3,7 L2,8 L2,9 L3,10, L4,10 L5,9 L5,8 L4,7 Z","line":{"color":"lightseagreen"},"fillcolor":"paleturquoise"}]};
        Plotly.newPlot('svg_paths', data, layout, {"responsive": true});
    };
</script>
