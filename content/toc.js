// Populate the sidebar
//
// This is a script, and not included directly in the page, to control the total size of the book.
// The TOC contains an entry for each page, so if each page includes a copy of the TOC,
// the total size of the page becomes O(n**2).
class MDBookSidebarScrollbox extends HTMLElement {
    constructor() {
        super();
    }
    connectedCallback() {
        this.innerHTML = '<ol class="chapter"><li class="chapter-item expanded "><a href="plotly_rs.html"><strong aria-hidden="true">1.</strong> Plotly.rs</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="getting_started.html"><strong aria-hidden="true">1.1.</strong> Getting Started</a></li></ol></li><li class="chapter-item expanded "><a href="fundamentals.html"><strong aria-hidden="true">2.</strong> Fundamentals</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="fundamentals/jupyter_support.html"><strong aria-hidden="true">2.1.</strong> Jupyter Support</a></li><li class="chapter-item "><a href="fundamentals/ndarray_support.html"><strong aria-hidden="true">2.2.</strong> ndarray Support</a></li><li class="chapter-item "><a href="fundamentals/shapes.html"><strong aria-hidden="true">2.3.</strong> Shapes</a></li><li class="chapter-item "><a href="fundamentals/themes.html"><strong aria-hidden="true">2.4.</strong> Themes</a></li><li class="chapter-item "><a href="fundamentals/static_image_export.html"><strong aria-hidden="true">2.5.</strong> Static Image Export</a></li><li class="chapter-item "><a href="fundamentals/timeseries_downsampling.html"><strong aria-hidden="true">2.6.</strong> Timeseries Downsampling</a></li></ol></li><li class="chapter-item expanded "><a href="recipes.html"><strong aria-hidden="true">3.</strong> Recipes</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="recipes/basic_charts.html"><strong aria-hidden="true">3.1.</strong> Basic Charts</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="recipes/basic_charts/scatter_plots.html"><strong aria-hidden="true">3.1.1.</strong> Scatter Plots</a></li><li class="chapter-item "><a href="recipes/basic_charts/line_charts.html"><strong aria-hidden="true">3.1.2.</strong> Line Charts</a></li><li class="chapter-item "><a href="recipes/basic_charts/bar_charts.html"><strong aria-hidden="true">3.1.3.</strong> Bar Charts</a></li><li class="chapter-item "><a href="recipes/basic_charts/pie_charts.html"><strong aria-hidden="true">3.1.4.</strong> Pie Charts</a></li><li class="chapter-item "><a href="recipes/basic_charts/sankey_diagrams.html"><strong aria-hidden="true">3.1.5.</strong> Sankey Diagrams</a></li></ol></li><li class="chapter-item "><a href="recipes/statistical_charts.html"><strong aria-hidden="true">3.2.</strong> Statistical Charts</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="recipes/statistical_charts/error_bars.html"><strong aria-hidden="true">3.2.1.</strong> Error Bars</a></li><li class="chapter-item "><a href="recipes/statistical_charts/box_plots.html"><strong aria-hidden="true">3.2.2.</strong> Box Plots</a></li><li class="chapter-item "><a href="recipes/statistical_charts/histograms.html"><strong aria-hidden="true">3.2.3.</strong> Histograms</a></li></ol></li><li class="chapter-item "><a href="recipes/scientific_charts.html"><strong aria-hidden="true">3.3.</strong> Scientific Charts</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="recipes/scientific_charts/contour_plots.html"><strong aria-hidden="true">3.3.1.</strong> Contour Plots</a></li><li class="chapter-item "><a href="recipes/scientific_charts/heatmaps.html"><strong aria-hidden="true">3.3.2.</strong> Heatmaps</a></li></ol></li><li class="chapter-item "><a href="recipes/financial_charts.html"><strong aria-hidden="true">3.4.</strong> Financial Charts</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="recipes/financial_charts/time_series_and_date_axes.html"><strong aria-hidden="true">3.4.1.</strong> Time Series and Date Axes</a></li><li class="chapter-item "><a href="recipes/financial_charts/candlestick_charts.html"><strong aria-hidden="true">3.4.2.</strong> Candlestick Charts</a></li><li class="chapter-item "><a href="recipes/financial_charts/ohlc_charts.html"><strong aria-hidden="true">3.4.3.</strong> OHLC Charts</a></li><li class="chapter-item "><a href="recipes/financial_charts/rangebreaks.html"><strong aria-hidden="true">3.4.4.</strong> Rangebreaks</a></li></ol></li><li class="chapter-item "><a href="recipes/3dcharts.html"><strong aria-hidden="true">3.5.</strong> 3D Charts</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="recipes/3dcharts/3dcharts.html"><strong aria-hidden="true">3.5.1.</strong> Scatter 3D</a></li></ol></li><li class="chapter-item "><a href="recipes/subplots.html"><strong aria-hidden="true">3.6.</strong> Subplots</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="recipes/subplots/subplots.html"><strong aria-hidden="true">3.6.1.</strong> Subplots</a></li><li class="chapter-item "><a href="recipes/subplots/multiple_axes.html"><strong aria-hidden="true">3.6.2.</strong> Multiple Axes</a></li></ol></li><li class="chapter-item "><a href="recipes/custom_controls.html"><strong aria-hidden="true">3.7.</strong> Custom Controls</a><a class="toggle"><div>❱</div></a></li><li><ol class="section"><li class="chapter-item "><a href="recipes/custom_controls/dropdowns.html"><strong aria-hidden="true">3.7.1.</strong> Dropdowns</a></li><li class="chapter-item "><a href="recipes/custom_controls/sliders.html"><strong aria-hidden="true">3.7.2.</strong> Sliders</a></li><li class="chapter-item "><a href="recipes/custom_controls/animations.html"><strong aria-hidden="true">3.7.3.</strong> Animations</a></li></ol></li></ol></li></ol>';
        // Set the current, active page, and reveal it if it's hidden
        let current_page = document.location.href.toString().split("#")[0].split("?")[0];
        if (current_page.endsWith("/")) {
            current_page += "index.html";
        }
        var links = Array.prototype.slice.call(this.querySelectorAll("a"));
        var l = links.length;
        for (var i = 0; i < l; ++i) {
            var link = links[i];
            var href = link.getAttribute("href");
            if (href && !href.startsWith("#") && !/^(?:[a-z+]+:)?\/\//.test(href)) {
                link.href = path_to_root + href;
            }
            // The "index" page is supposed to alias the first chapter in the book.
            if (link.href === current_page || (i === 0 && path_to_root === "" && current_page.endsWith("/index.html"))) {
                link.classList.add("active");
                var parent = link.parentElement;
                if (parent && parent.classList.contains("chapter-item")) {
                    parent.classList.add("expanded");
                }
                while (parent) {
                    if (parent.tagName === "LI" && parent.previousElementSibling) {
                        if (parent.previousElementSibling.classList.contains("chapter-item")) {
                            parent.previousElementSibling.classList.add("expanded");
                        }
                    }
                    parent = parent.parentElement;
                }
            }
        }
        // Track and set sidebar scroll position
        this.addEventListener('click', function(e) {
            if (e.target.tagName === 'A') {
                sessionStorage.setItem('sidebar-scroll', this.scrollTop);
            }
        }, { passive: true });
        var sidebarScrollTop = sessionStorage.getItem('sidebar-scroll');
        sessionStorage.removeItem('sidebar-scroll');
        if (sidebarScrollTop) {
            // preserve sidebar scroll position when navigating via links within sidebar
            this.scrollTop = sidebarScrollTop;
        } else {
            // scroll sidebar to current active section when navigating via "next/previous chapter" buttons
            var activeSection = document.querySelector('#sidebar .active');
            if (activeSection) {
                activeSection.scrollIntoView({ block: 'center' });
            }
        }
        // Toggle buttons
        var sidebarAnchorToggles = document.querySelectorAll('#sidebar a.toggle');
        function toggleSection(ev) {
            ev.currentTarget.parentElement.classList.toggle('expanded');
        }
        Array.from(sidebarAnchorToggles).forEach(function (el) {
            el.addEventListener('click', toggleSection);
        });
    }
}
window.customElements.define("mdbook-sidebar-scrollbox", MDBookSidebarScrollbox);
