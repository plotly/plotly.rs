use plotly::callbacks::ClickEvent;
use plotly::{Histogram, Plot, Scatter, common::Mode, histogram::Bins};
use web_sys::js_sys::Math;
use yew::prelude::*;

#[function_component(App)]
pub fn plot_component() -> Html {
    let x = use_state(|| None::<f64>);
    let y = use_state(|| None::<f64>);
    let point_numbers = use_state(|| None::<Vec<usize>>);
    let point_number = use_state(|| None::<usize>);
    let curve_number = use_state(|| 0usize);
    let click_event = use_state(ClickEvent::default);

    let x_clone = x.clone();
    let y_clone = y.clone();
    let curve_clone = curve_number.clone();
    let point_numbers_clone = point_numbers.clone();
    let point_number_clone = point_number.clone();
    let click_event_clone = click_event.clone();

    let p = yew_hooks::use_async::<_, _, ()>({
        let id = "plot-div";
        let mut fig = Plot::new();
        let xs: Vec<f64> = (0..50).map(|i| i as f64).collect();
        let ys: Vec<f64> = xs.iter().map(|x| x.sin() * 5.0).collect();
        fig.add_trace(
            Scatter::new(xs.clone(), ys.clone())
                .mode(Mode::Markers)
                .name("Sine Wave Markers"),
        );
        let random_values: Vec<f64> = (0..500).map(|_| Math::random() * 100.0).collect();
        fig.add_trace(
            Histogram::new(random_values)
                .name("Random Data Histogram")
                .x_bins(Bins::new(-1.0, 30.0, 5.0)),
        );
        let layout = plotly::Layout::new().title("Click Event Callback Example in Yew");
        fig.set_layout(layout);
        async move {
            plotly::bindings::new_plot(id, &fig).await;
            plotly::callbacks::bind_click(id, move |event| {
                let pt = &event.points[0];
                x_clone.set(pt.x);
                y_clone.set(pt.y);
                curve_clone.set(pt.curve_number);
                point_numbers_clone.set(pt.point_numbers.clone());
                point_number_clone.set(pt.point_number);
                click_event_clone.set(event);
            });
            Ok(())
        }
    });
    // Only on first render
    use_effect_with((), move |_| {
        p.run();
    });

    html! {
        <>
            <div id="plot-div"></div>
            <div>
                <p>{format!("x: {:?}",*x)}</p>
                <p>{format!("y: {:?}",*y)}</p>
                <p>{format!("curveNumber: {:?}",*curve_number)}</p>
                <p>{format!("pointNumber: {:?}",*point_number)}</p>
                <p>{format!("pointNumbers: {:?}",*point_numbers)}</p>
                <p>{format!("ClickEvent: {:?}",*click_event)}</p>
            </div>
        </>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}
