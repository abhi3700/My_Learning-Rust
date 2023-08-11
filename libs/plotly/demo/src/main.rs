use plotly::{Plot, Scatter};

fn main() {
    let mut plot = Plot::new();
    let trace = Scatter::new(vec![0, 1, 2], vec![2, 1, 0]);
    plot.add_trace(trace);

    // creates plot (portable) at root directory where `Cargo.toml` file exists.
    plot.write_html("out.html");
    plot.show(); // opens in browser
}
