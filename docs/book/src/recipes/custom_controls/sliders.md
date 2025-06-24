# Sliders


Sliders provide interactive controls to animate through different data states or parameters. They are different from range sliders and are typically used for animating through time series data or changing plot parameters dynamically.

Data sliders support various customization options including:
- **Positioning**: Control x, y coordinates and anchors
- **Styling**: Background color, border color, width, and font
- **Behavior**: Active step highlighting, step execution control
- **Dimensions**: Length, width, and orientation (horizontal/vertical)
- **Steps**: Multiple steps with different data states or parameters

## Customizing a Simple Data Slider

Sliders are perfect for stepping through sequential data, like time series. This example creates a slider to show the population of different animals for four consecutive years.
It shows how to use the slider API to customize slider features.

```rust,no_run
{{#include ../../../../../examples/custom_controls/src/main.rs:bar_chart_with_slider_customization}}
```
{{#include ../../../../../examples/custom_controls/output/inline_bar_chart_with_slider_customization.html}}

## Slider for Parameter Control

Sliders aren't just for data; they can also be used to control parameters in a function. Here, a slider modifies the frequency of a sinusoidal wave, updating the plot in real-time.

```rust,no_run
{{#include ../../../../../examples/custom_controls/src/main.rs:sinusoidal_slider_example}}
```
{{#include ../../../../../examples/custom_controls/output/inline_sinusoidal_slider_example.html}}

## Advanced Slider: GDP vs. Life Expectancy

This example, based on the Python Plotly Gapminder dataset, demonstrates a more complex use case. A slider is used to animate a bubble chart showing the relationship between GDP per capita and life expectancy across different continents over several decades. See [https://plotly.com/python/sliders/](https://plotly.com/python/sliders/)

```rust,no_run
{{#include ../../../../../examples/custom_controls/src/main.rs:gdp_life_expectancy_slider_example}}
```
{{#include ../../../../../examples/custom_controls/output/inline_gdp_life_expectancy_slider_example.html}} 
