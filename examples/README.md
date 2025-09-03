# Examples

This folder contains a multitude of usage examples covering as many features of the library as possible. 

To run the basic examples, `cd` into the chosen example folder and run `cargo run`. Upon completion all the generated plots of that example are located in the `output` folder as `html` files and can be open in your browser. 

You can also configure the chosen example to open the resulting plots automatically. To do so, open the `src/main.rs` file, locate the `main` function and change the boolean flag of the called functions from `false` to `true`. This will make the examples open the default browser application and load the generated HTML files.

For more complex examples, instructions on how to run them can be found in the README of each example's subdirectory.

Pull requests with more examples of different behavior are always welcome.
