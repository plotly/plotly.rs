# Plotly Orca

**This feature is deprecated, please use the `kaleido` feature instead.**

Plotly Orca implements the `orca` feature for [Plotly.rs](https://github.com/igiagkiozis/plotly)
 
The `orca` feature enables `Plot` conversion to the following output formats: png, jpeg, webp, svg, pdf and eps. 

## Installation instructions
To use `plotly_orca` which is used by the `orca` feature for `plotly`, first you need to install the
[Orca command line utility](https://github.com/plotly/orca). 

Download the appropriate binary of Orca for your system from [here](https://github.com/plotly/orca/releases).

### Linux
Copy the orca-<version>-x86_64.AppImage anywhere in your home directory. 
Say you saved this in: /home/<user_name>/apps/orca-<version>-x86_64.AppImage
Then simply create a symbolic link pointing to the AppImage:

```bash 
chmod +x /home/<user_name>/apps/orca-<version>-x86_64.AppImage
sudo ln -s /home/<user_name>/apps/orca-<version>-x86_64.AppImage /usr/bin/plotly_orca
```

Note, it's important that the symbolic link is named exactly as shown above. The name of the link is not `orca` as there 
already exists an executable on RHEL 8 and Centos 8 with that name. 

### MacOSX
Install the dmg package. After that the `orca` binary will be detected by `plotly_orca`.

### Windows
Run the installation executable with the default target path. After that `plotly_orca` will be able to find the `orca.exe`.

## Examples
 
Once the `Orca` executable is installed you can give the feature a spin as follows: 
 
```rust
extern crate plotly;
use plotly::common::Mode;
use plotly::{Plot, Scatter};

fn line_and_scatter_plot() {
    let trace1 = Scatter::new(vec![1, 2, 3, 4], vec![10, 15, 13, 17])
        .name("trace1")
        .mode(Mode::Markers);
    let trace2 = Scatter::new(vec![2, 3, 4, 5], vec![16, 5, 11, 9])
        .name("trace2")
        .mode(Mode::Lines);
    let trace3 = Scatter::new(vec![1, 2, 3, 4], vec![12, 9, 15, 12]).name("trace3");

    let mut plot = Plot::new();
    plot.add_trace(trace1);
    plot.add_trace(trace2);
    plot.add_trace(trace3);

    // The following will save the plot in all available formats and show the plot.
    plot.to_png("scatter", 1024, 680);
    plot.to_jpeg("scatter", 1024, 680);
    plot.to_webp("scatter", 1024, 680);
    plot.to_svg("scatter", 1024, 680);
    plot.to_pdf("scatter",1024, 680);
    plot.to_eps("scatter", 1024, 680);
    plot.show();
}

fn main() -> std::io::Result<()> {
    line_and_scatter_plot();
    Ok(())
}
```

Note that `Orca` is not particularly fast; so it may take up to a second to produce an image; this performance is consistent with 
the Plotly package for Python which also uses `Orca` under the covers to rasterize plots. That said, it's still faster than the 
alternative manual approach.