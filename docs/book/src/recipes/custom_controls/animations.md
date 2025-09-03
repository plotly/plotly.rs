# Animations

Animations in Plotly.rs allow you to create dynamic, interactive visualizations that can play through different data states over time.

## GDP vs. Life Expectancy Animation

This example demonstrates an animation based on the Gapminder dataset, showing the relationship between GDP per capita and life expectancy across different continents over several decades. The animation is based on the JavaScript example https://plotly.com/javascript/gapminder-example/ and shows how to create buttons and sliders that interact with the animation mechanism.

```rust,no_run
{{#include ../../../../../examples/custom_controls/src/main.rs:gdp_life_expectancy_animation_example}}
```
{{#include ../../../../../examples/custom_controls/output/inline_gdp_life_expectancy_animation_example.html}}
