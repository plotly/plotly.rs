#[cfg(feature = "plotly_static")]
pub mod sync {
    use std::path::Path;

    use crate::{plot::Plot, ImageFormat};

    /// Extension methods for exporting plots using a synchronous exporter.
    pub trait ExporterSyncExt {
        /// Convert the `Plot` to a static image of the given image format and
        /// save at the given location using a provided StaticExporter.
        ///
        /// This method allows you to reuse a StaticExporter instance across
        /// multiple plots, which is more efficient than creating a new one for
        /// each operation.
        ///
        /// This method requires the usage of the `plotly_static` crate using
        /// one of the available feature flags. For advanced usage (parallelism, exporter reuse, custom config), see the [plotly_static documentation](https://docs.rs/plotly_static/).
        ///
        /// # Arguments
        ///
        /// * `exporter` - A mutable reference to a StaticExporter instance
        /// * `filename` - The destination path for the output file
        /// * `format` - The desired output image format
        /// * `width` - The width of the output image in pixels
        /// * `height` - The height of the output image in pixels
        /// * `scale` - The scale factor for the image (1.0 = normal size)
        ///
        /// # Examples
        ///
        /// ```no_run
        /// use plotly::{Plot, Scatter};
        /// use plotly::export::sync::ExporterSyncExt as _;
        /// use plotly::plotly_static::{StaticExporterBuilder, ImageFormat};
        ///
        /// let mut plot = Plot::new();
        /// plot.add_trace(Scatter::new(vec![1, 2, 3], vec![4, 5, 6]));
        ///
        /// let mut exporter = StaticExporterBuilder::default()
        ///     .build()
        ///     .expect("Failed to create StaticExporter");
        ///
        /// // Export multiple plots using the same exporter
        /// exporter.write_image(&plot, "plot1", ImageFormat::PNG, 800, 600, 1.0)
        ///     .expect("Failed to export plot");
        ///
        /// exporter.close();
        /// ```
        fn write_image<P: AsRef<Path>>(
            &mut self,
            plot: &Plot,
            filename: P,
            format: ImageFormat,
            width: usize,
            height: usize,
            scale: f64,
        ) -> Result<(), Box<dyn std::error::Error>>;

        /// Convert the `Plot` to a static image and return the image as a
        /// `base64` string. Supported formats are [ImageFormat::JPEG],
        /// [ImageFormat::PNG] and [ImageFormat::WEBP].
        ///
        /// This method allows you to reuse the same StaticExporter instance
        /// across multiple plots, which is more efficient than creating
        /// a new one for each operation.
        ///
        /// For advanced usage (parallelism, exporter reuse, custom config), see the [plotly_static documentation](https://docs.rs/plotly_static/).
        ///
        /// # Arguments
        ///
        /// * `format` - The desired output image format
        /// * `width` - The width of the output image in pixels
        /// * `height` - The height of the output image in pixels
        /// * `scale` - The scale factor for the image (1.0 = normal size)
        ///
        /// # Examples
        ///
        /// ```no_run
        /// use plotly::{Plot, Scatter};
        /// use plotly::export::sync::ExporterSyncExt as _;
        /// use plotly::plotly_static::{StaticExporterBuilder, ImageFormat};
        ///
        /// let mut plot = Plot::new();
        /// plot.add_trace(Scatter::new(vec![1, 2, 3], vec![4, 5, 6]));
        ///
        /// let mut exporter = StaticExporterBuilder::default()
        ///     .build()
        ///     .expect("Failed to create StaticExporter");
        ///
        /// let base64_data = exporter.to_base64(&plot, ImageFormat::PNG, 800, 600, 1.0)
        ///     .expect("Failed to export plot");
        ///
        /// exporter.close();
        /// ```
        fn to_base64(
            &mut self,
            plot: &Plot,
            format: ImageFormat,
            width: usize,
            height: usize,
            scale: f64,
        ) -> Result<String, Box<dyn std::error::Error>>;

        /// Convert the `Plot` to SVG and return it as a String.
        ///
        /// This method allows you to reuse the same StaticExporter instance
        /// across multiple plots, which is more efficient than creating
        /// a new one for each operation.
        ///
        /// For advanced usage (parallelism, exporter reuse, custom config), see the [plotly_static documentation](https://docs.rs/plotly_static/).
        ///
        /// # Arguments
        ///
        /// * `width` - The width of the output image in pixels
        /// * `height` - The height of the output image in pixels
        /// * `scale` - The scale factor for the image (1.0 = normal size)
        ///
        /// # Examples
        ///
        /// ```no_run
        /// use plotly::{Plot, Scatter};
        /// use plotly::export::sync::ExporterSyncExt as _;
        /// use plotly::plotly_static::{StaticExporterBuilder, ImageFormat};
        ///
        /// let mut plot = Plot::new();
        /// plot.add_trace(Scatter::new(vec![1, 2, 3], vec![4, 5, 6]));
        ///
        /// let mut exporter = StaticExporterBuilder::default()
        ///     .build()
        ///     .expect("Failed to create StaticExporter");
        ///
        /// let svg_data = exporter.to_svg(&plot, 800, 600, 1.0)
        ///     .expect("Failed to export plot");
        ///
        /// exporter.close();
        /// ```
        fn to_svg(
            &mut self,
            plot: &Plot,
            width: usize,
            height: usize,
            scale: f64,
        ) -> Result<String, Box<dyn std::error::Error>>;
    }

    impl ExporterSyncExt for plotly_static::StaticExporter {
        fn write_image<P: AsRef<Path>>(
            &mut self,
            plot: &Plot,
            filename: P,
            format: ImageFormat,
            width: usize,
            height: usize,
            scale: f64,
        ) -> Result<(), Box<dyn std::error::Error>> {
            self.write_fig(
                filename.as_ref(),
                &serde_json::to_value(plot)?,
                format,
                width,
                height,
                scale,
            )
        }

        fn to_base64(
            &mut self,
            plot: &Plot,
            format: ImageFormat,
            width: usize,
            height: usize,
            scale: f64,
        ) -> Result<String, Box<dyn std::error::Error>> {
            match format {
                ImageFormat::JPEG | ImageFormat::PNG | ImageFormat::WEBP => self.write_to_string(
                    &serde_json::to_value(plot)?,
                    format,
                    width,
                    height,
                    scale,
                ),
                _ => Err(format!(
                    "Cannot generate base64 string for ImageFormat:{format}. Allowed formats are JPEG, PNG, WEBP"
                )
                .into()),
            }
        }

        fn to_svg(
            &mut self,
            plot: &Plot,
            width: usize,
            height: usize,
            scale: f64,
        ) -> Result<String, Box<dyn std::error::Error>> {
            self.write_to_string(
                &serde_json::to_value(plot)?,
                ImageFormat::SVG,
                width,
                height,
                scale,
            )
        }
    }
}

#[cfg(feature = "plotly_static")]
pub mod r#async {
    use std::path::Path;

    use async_trait::async_trait;

    use crate::{plot::Plot, ImageFormat};

    /// Extension methods for exporting plots using an asynchronous exporter.
    #[async_trait(?Send)]
    pub trait ExporterAsyncExt {
        /// Convert the `Plot` to a static image of the given format and save at
        /// the given location using the asynchronous exporter.
        ///
        /// The exporter must have been built with the `build_async` method of
        /// the StaticExporterBuilder.
        ///
        /// Functionally signature equivalent to the sync version in
        /// [`crate::export::sync::ExporterSyncExt::write_image`], but meant for
        /// async contexts.
        ///
        /// For more details see the [plotly_static documentation](https://docs.rs/plotly_static/).
        async fn write_image<P: AsRef<Path>>(
            &mut self,
            plot: &Plot,
            filename: P,
            format: ImageFormat,
            width: usize,
            height: usize,
            scale: f64,
        ) -> Result<(), Box<dyn std::error::Error>>;

        /// Convert the `Plot` to a static image and return the image as a
        /// `base64` string using the asynchronous exporter.
        ///
        /// The exporter must have been built with the `build_async` method of
        /// the StaticExporterBuilder.
        ///
        /// Functionally signature equivalent to the sync version in
        /// [`crate::export::sync::ExporterSyncExt::to_base64`], but meant for
        /// async contexts.
        ///
        /// For more details see the [plotly_static documentation](https://docs.rs/plotly_static/).
        async fn to_base64(
            &mut self,
            plot: &Plot,
            format: ImageFormat,
            width: usize,
            height: usize,
            scale: f64,
        ) -> Result<String, Box<dyn std::error::Error>>;

        /// Convert the `Plot` to SVG and return it as a String using the
        /// asynchronous exporter.
        ///
        /// Functionally signature equivalent to the sync version in
        /// [`crate::export::sync::ExporterSyncExt::to_svg`], but meant for
        /// async contexts.
        ///
        /// For more details see the [plotly_static documentation](https://docs.rs/plotly_static/).
        async fn to_svg(
            &mut self,
            plot: &Plot,
            width: usize,
            height: usize,
            scale: f64,
        ) -> Result<String, Box<dyn std::error::Error>>;
    }

    #[async_trait(?Send)]
    impl ExporterAsyncExt for plotly_static::AsyncStaticExporter {
        async fn write_image<P: AsRef<Path>>(
            &mut self,
            plot: &Plot,
            filename: P,
            format: ImageFormat,
            width: usize,
            height: usize,
            scale: f64,
        ) -> Result<(), Box<dyn std::error::Error>> {
            self.write_fig(
                filename.as_ref(),
                &serde_json::to_value(plot)?,
                format,
                width,
                height,
                scale,
            )
            .await
        }

        async fn to_base64(
            &mut self,
            plot: &Plot,
            format: ImageFormat,
            width: usize,
            height: usize,
            scale: f64,
        ) -> Result<String, Box<dyn std::error::Error>> {
            match format {
                ImageFormat::JPEG | ImageFormat::PNG | ImageFormat::WEBP => self
                    .write_to_string(
                        &serde_json::to_value(plot)?,
                        format,
                        width,
                        height,
                        scale,
                    )
                    .await,
                _ => Err(format!(
                    "Cannot generate base64 string for ImageFormat:{format}. Allowed formats are JPEG, PNG, WEBP"
                )
                .into()),
            }
        }

        async fn to_svg(
            &mut self,
            plot: &Plot,
            width: usize,
            height: usize,
            scale: f64,
        ) -> Result<String, Box<dyn std::error::Error>> {
            self.write_to_string(
                &serde_json::to_value(plot)?,
                ImageFormat::SVG,
                width,
                height,
                scale,
            )
            .await
        }
    }
}
