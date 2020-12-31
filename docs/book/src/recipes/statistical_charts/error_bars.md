# Error Bars

The following imports have been used to produce the plots below:

```rust
use itertools_num::linspace;
use plotly::box_plot::{BoxMean, BoxPoints};
use plotly::common::{ErrorData, ErrorType, Line, Marker, Mode, Orientation, Title};
use plotly::histogram::{Bins, Cumulative, HistFunc, HistNorm};
use plotly::layout::{Axis, BarMode, BoxMode, Layout, Margin};
use plotly::{Bar, BoxPlot, Histogram, NamedColor, Plot, Rgb, Rgba, Scatter};
use rand_distr::{Distribution, Normal, Uniform};

```

The `to_inline_html` method is used to produce the html plot displayed in this page.

## Basic Symmetric Error Bars
```rust
fn basic_symmetric_error_bars(show: bool) {
    let trace1 = Scatter::new(vec![0, 1, 2], vec![6, 10, 2])
        .name("trace1")
        .error_y(ErrorData::new(ErrorType::Data).array(vec![1.0, 2.0, 3.0]));

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    if show {
        plot.show().unwrap();
    }
    println!(
        "{}",
        plot.to_inline_html(Some("basic_symmetric_error_bars")).unwrap()
    );
}
```
<div id="basic_symmetric_error_bars" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("basic_symmetric_error_bars")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"type":"scatter","x":[0,1,2],"y":[6,10,2],"name":"trace1","error_y":{"type":"data","array":[1.0,2.0,3.0]}};
var data = [trace_0];
var layout = {};
        Plotly.newPlot('basic_symmetric_error_bars', data, layout, {"responsive": true});
    };
</script>

## Asymmetric Error Bars
```rust
fn asymmetric_error_bars(show: bool) {
    let trace1 = Scatter::new(vec![1, 2, 3, 4], vec![2, 1, 3, 4])
        .name("trace1")
        .error_y(
            ErrorData::new(ErrorType::Data)
                .array(vec![0.1, 0.2, 0.1, 0.1])
                .array_minus(vec![0.2, 0.4, 1., 0.2]),
        );

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    if show {
        plot.show().unwrap();
    }
    println!("{}", plot.to_inline_html(Some("asymmetric_error_bars")).unwrap());
}
```
<div id="asymmetric_error_bars" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("asymmetric_error_bars")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"type":"scatter","x":[1,2,3,4],"y":[2,1,3,4],"name":"trace1","error_y":{"type":"data","array":[0.1,0.2,0.1,0.1],"arrayminus":[0.2,0.4,1.0,0.2]}};
var data = [trace_0];
var layout = {};
        Plotly.newPlot('asymmetric_error_bars', data, layout, {"responsive": true});
    };
</script>


## Error Bars as a Percentage of the Y Value
```rust
fn error_bars_as_a_percentage_of_the_y_value(show: bool) {
    let trace1 = Scatter::new(vec![0, 1, 2], vec![6, 10, 2])
        .name("trace1")
        .error_y(ErrorData::new(ErrorType::Percent).value(50.).visible(true));

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    if show {
        plot.show().unwrap();
    }
    println!(
        "{}",
        plot.to_inline_html(Some("error_bars_as_a_percentage_of_the_y_value")).unwrap()
    );
}
```
<div id="error_bars_as_a_percentage_of_the_y_value" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("error_bars_as_a_percentage_of_the_y_value")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"type":"scatter","x":[0,1,2],"y":[6,10,2],"name":"trace1","error_y":{"type":"percent","visible":true,"value":50.0}};
var data = [trace_0];
var layout = {};
        Plotly.newPlot('error_bars_as_a_percentage_of_the_y_value', data, layout, {"responsive": true});
    };
</script>


## Asymmetric Error Bars with a Constant Offset
```rust
fn asymmetric_error_bars_with_a_constant_offset(show: bool) {
    let trace1 = Scatter::new(vec![1, 2, 3, 4], vec![2, 1, 3, 4])
        .name("trace1")
        .error_y(
            ErrorData::new(ErrorType::Percent)
                .symmetric(false)
                .value(15.)
                .value_minus(25.),
        );

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    if show {
        plot.show().unwrap();
    }
    println!(
        "{}",
        plot.to_inline_html(Some("asymmetric_error_bars_with_a_constant_offset")).unwrap()
    );
}

```
<div id="asymmetric_error_bars_with_a_constant_offset" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("asymmetric_error_bars_with_a_constant_offset")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"type":"scatter","x":[1,2,3,4],"y":[2,1,3,4],"name":"trace1","error_y":{"type":"percent","symmetric":false,"value":15.0,"valueminus":25.0}};
var data = [trace_0];
var layout = {};
        Plotly.newPlot('asymmetric_error_bars_with_a_constant_offset', data, layout, {"responsive": true});
    };
</script>


## Horizontal Error Bars
```rust
fn horizontal_error_bars(show: bool) {
    let trace1 = Scatter::new(vec![1, 2, 3, 4], vec![2, 1, 3, 4])
        .name("trace1")
        .error_x(ErrorData::new(ErrorType::Percent).value(10.));

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    if show {
        plot.show().unwrap();
    }
    println!("{}", plot.to_inline_html(Some("horizontal_error_bars")).unwrap());
}
```
<div id="horizontal_error_bars" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("horizontal_error_bars")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"type":"scatter","x":[1,2,3,4],"y":[2,1,3,4],"name":"trace1","error_x":{"type":"percent","value":10.0}};
var data = [trace_0];
var layout = {};
        Plotly.newPlot('horizontal_error_bars', data, layout, {"responsive": true});
    };
</script>


## Bar Chart with Error Bars
```rust
fn bar_chart_with_error_bars(show: bool) {
    let trace_c = Bar::new(vec!["Trial 1", "Trial 2", "Trial 3"], vec![3, 6, 4])
        .error_y(ErrorData::new(ErrorType::Data).array(vec![1., 0.5, 1.5]));
    let trace_e = Bar::new(vec!["Trial 1", "Trial 2", "Trial 3"], vec![4, 7, 3])
        .error_y(ErrorData::new(ErrorType::Data).array(vec![0.5, 1., 2.]));

    let mut plot = Plot::new();
    plot.add_trace(trace_c);
    plot.add_trace(trace_e);

    let layout = Layout::new().bar_mode(BarMode::Group);
    plot.set_layout(layout);

    if show {
        plot.show().unwrap();
    }
    println!("{}", plot.to_inline_html(Some("bar_chart_with_error_bars")).unwrap());
}
```
<div id="bar_chart_with_error_bars" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("bar_chart_with_error_bars")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"x":["Trial 1","Trial 2","Trial 3"],"y":[3,6,4],"type":"bar","error_y":{"type":"data","array":[1.0,0.5,1.5]}};
var trace_1 = {"x":["Trial 1","Trial 2","Trial 3"],"y":[4,7,3],"type":"bar","error_y":{"type":"data","array":[0.5,1.0,2.0]}};
var data = [trace_0,trace_1];
var layout = {"barmode":"group"};
        Plotly.newPlot('bar_chart_with_error_bars', data, layout, {"responsive": true});
    };
</script>


## Colored and Styled Error Bars
```rust
fn colored_and_styled_error_bars(show: bool) {
    let x_theo: Vec<f64> = linspace(-4., 4., 100).collect();
    let sincx: Vec<f64> = x_theo
        .iter()
        .map(|x| (x * std::f64::consts::PI).sin() / (*x * std::f64::consts::PI))
        .collect();
    let x = vec![
        -3.8, -3.03, -1.91, -1.46, -0.89, -0.24, -0.0, 0.41, 0.89, 1.01, 1.91, 2.28, 2.79, 3.56,
    ];
    let y = vec![
        -0.02, 0.04, -0.01, -0.27, 0.36, 0.75, 1.03, 0.65, 0.28, 0.02, -0.11, 0.16, 0.04, -0.15,
    ];

    let trace1 = Scatter::new(x_theo, sincx).name("sinc(x)");
    let trace2 = Scatter::new(x, y)
        .mode(Mode::Markers)
        .name("measured")
        .error_y(
            ErrorData::new(ErrorType::Constant)
                .value(0.1)
                .color(NamedColor::Purple)
                .thickness(1.5)
                .width(3),
        )
        .error_x(
            ErrorData::new(ErrorType::Constant)
                .value(0.2)
                .color(NamedColor::Purple)
                .thickness(1.5)
                .width(3),
        )
        .marker(Marker::new().color(NamedColor::Purple).size(8));

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);

    if show {
        plot.show().unwrap();
    }
    println!(
        "{}",
        plot.to_inline_html(Some("colored_and_styled_error_bars")).unwrap()
    );
}
```
<div id="colored_and_styled_error_bars" class="plotly-graph-div" style="height:100%; width:100%;"></div>
<script type="text/javascript">
    window.PLOTLYENV=window.PLOTLYENV || {};
    if (document.getElementById("colored_and_styled_error_bars")) {
        var d3 = Plotly.d3;
        var image_element= d3.select('#image-export');
        var trace_0 = {"type":"scatter","x":[-4.0,-3.919191919191919,-3.8383838383838382,-3.757575757575758,-3.676767676767677,-3.595959595959596,-3.515151515151515,-3.4343434343434343,-3.3535353535353534,-3.2727272727272725,-3.191919191919192,-3.111111111111111,-3.0303030303030303,-2.9494949494949494,-2.8686868686868685,-2.787878787878788,-2.7070707070707067,-2.6262626262626263,-2.5454545454545454,-2.4646464646464645,-2.3838383838383836,-2.3030303030303028,-2.2222222222222223,-2.141414141414141,-2.0606060606060606,-1.9797979797979797,-1.8989898989898988,-1.818181818181818,-1.737373737373737,-1.6565656565656566,-1.5757575757575757,-1.4949494949494948,-1.414141414141414,-1.333333333333333,-1.2525252525252522,-1.1717171717171713,-1.0909090909090908,-1.01010101010101,-0.9292929292929291,-0.8484848484848482,-0.7676767676767673,-0.6868686868686864,-0.606060606060606,-0.5252525252525251,-0.4444444444444442,-0.3636363636363633,-0.28282828282828243,-0.20202020202020154,-0.1212121212121211,-0.04040404040404022,0.040404040404040664,0.12121212121212199,0.20202020202020243,0.2828282828282829,0.3636363636363642,0.44444444444444464,0.525252525252526,0.6060606060606064,0.6868686868686869,0.7676767676767682,0.8484848484848486,0.92929292929293,1.0101010101010104,1.0909090909090917,1.1717171717171722,1.2525252525252526,1.333333333333334,1.4141414141414144,1.4949494949494957,1.5757575757575761,1.6565656565656575,1.737373737373738,1.8181818181818183,1.8989898989898997,1.9797979797979801,2.0606060606060614,2.141414141414142,2.2222222222222223,2.3030303030303036,2.383838383838384,2.4646464646464654,2.545454545454546,2.626262626262627,2.7070707070707076,2.787878787878788,2.8686868686868694,2.94949494949495,3.030303030303031,3.1111111111111116,3.191919191919193,3.2727272727272734,3.353535353535354,3.434343434343435,3.5151515151515156,3.595959595959597,3.6767676767676774,3.757575757575758,3.838383838383839,3.9191919191919196,4.0],"y":[-3.8981718325193755e-17,-0.020397798541945587,-0.04031937248783161,-0.05845762953942655,-0.07356352880377899,-0.08452676279573113,-0.09045110791884156,-0.09071966773486224,-0.08504567456412637,-0.07350522697104646,-0.056549288961486976,-0.03499341200182195,-0.009984901847327851,0.017051531611460852,0.04448730688801305,0.0705791505006865,0.0935693553283155,0.11179174825914784,0.12377748055647672,0.1283545583699417,0.12473517546144892,0.11258539596462419,0.09207254289585291,0.06388674049716098,0.02923438070194994,-0.010197232673846484,-0.05230324323402314,-0.09465022438883168,-0.13458693322243767,-0.16937168694961346,-0.19631016500519627,-0.21289670377041103,-0.2169517735088934,-0.20674833578317192,-0.18112018110459843,-0.13954612597107577,-0.08220506992725843,-0.009998321751610412,0.07546277218501142,0.17190410911627638,0.2764694381823275,0.3858309757531781,0.4963251200302811,0.6041070057159245,0.7053165984920191,0.796248356503686,0.8735162206555506,0.934205854303506,0.9760066301702228,0.9973168298474766,0.9973168298474766,0.9760066301702224,0.9342058543035054,0.8735162206555502,0.7962483565036851,0.7053165984920186,0.6041070057159234,0.49632512003028056,0.38583097575317754,0.2764694381823262,0.17190410911627585,0.07546277218501045,-0.009998321751610828,-0.08220506992725911,-0.13954612597107635,-0.18112018110459863,-0.2067483357831721,-0.2169517735088934,-0.2128967037704109,-0.1963101650051961,-0.16937168694961313,-0.13458693322243725,-0.09465022438883153,-0.0523032432340227,-0.010197232673846196,0.029234380701950335,0.06388674049716131,0.09207254289585291,0.11258539596462437,0.12473517546144898,0.1283545583699417,0.12377748055647667,0.11179174825914762,0.0935693553283152,0.0705791505006865,0.044487306888012675,0.01705153161146066,-0.009984901847328035,-0.034993412001822106,-0.056549288961487254,-0.07350522697104656,-0.08504567456412637,-0.09071966773486229,-0.09045110791884153,-0.084526762795731,-0.07356352880377891,-0.05845762953942655,-0.04031937248783147,-0.020397798541945445,-3.8981718325193755e-17],"name":"sinc(x)"};
var trace_1 = {"type":"scatter","x":[-3.8,-3.03,-1.91,-1.46,-0.89,-0.24,-0.0,0.41,0.89,1.01,1.91,2.28,2.79,3.56],"y":[-0.02,0.04,-0.01,-0.27,0.36,0.75,1.03,0.65,0.28,0.02,-0.11,0.16,0.04,-0.15],"name":"measured","mode":"markers","marker":{"size":8,"color":"purple"},"error_x":{"type":"constant","value":0.2,"color":"purple","thickness":1.5,"width":3},"error_y":{"type":"constant","value":0.1,"color":"purple","thickness":1.5,"width":3}};
var data = [trace_0,trace_1];
var layout = {};
        Plotly.newPlot('colored_and_styled_error_bars', data, layout, {"responsive": true});
    };
</script>

