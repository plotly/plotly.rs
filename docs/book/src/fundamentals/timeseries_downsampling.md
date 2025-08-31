# Timeseries Downsampling 

In situations where the number of points of a timeseries is extremely large, generating a plot and visualizing it using plotly will be slow or not possible.

For such cases, it is ideal to use a downsampling method that preserves the visual characteristics of the timeseries. One such method is to use the Largest Triangle Three Bucket (LTTB) method.

The MinMaxLTTB or classical LTTB method can be used to downsample the timeseries prior to generating the static HTML plots. An example of how this can be achieved can be found in [examples/downsampling](https://github.com/plotly/plotly.rs/tree/main/examples/downsampling) directory using the [minmaxlttb-rs](https://github.com/andrei-ng/minmaxlttb-rs) crate.


 For more examples see the [minmaxlttb-rs](https://github.com/andrei-ng/minmaxlttb-rs) crate. 

## Example downsampling

{{#include ../../../../examples/downsampling/output/inline_minmaxlttb_downsampling.html}}
