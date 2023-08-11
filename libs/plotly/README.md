# Plotly

## Overview

- Plotly (JS, Python) lib can now be used in rust projects for data analytics.
- As a developer who has coded before in Python, I can now rewrite the python code in rust ensuring that the code is more performant and safe. It works just like the python version function calling wise.
- This crate is a wrapper around the `plotly.js` library.
- Plotly charts can now be integrated with `yew` projects as well. A small sample:

  ```rust
  #[function_component]
  fn App() -> Html {
      let mut plot = Plot::new();
      let x_values = vec![1, 2, 3];
      let y_values = vec![1, 3, 2];

      let trace = Scatter::new(x_values, y_values)
          .mode(Mode::LinesMarkersText)
          .name("Scatter");

      plot.add_trace(trace);

      html! { <Plotly plot={plot}/> }
  }
  ```

## Getting Started

- Add the following to your `Cargo.toml` file:

  ```toml
  [dependencies]
  plotly = "0.1.0"
  ```

  Also, can add more features with `features` param here.

- For code, refer [this](./demo/src/main.rs) file.

## References

- plotly lib:
  - [crates.io](https://crates.io/crates/plotly/)
  - [API documentation](https://docs.rs/plotly/latest/plotly/)
  - [Github](https://github.com/igiagkiozis/plotly)
- [yew-plotly](https://crates.io/crates/yew-plotly): Allows you to use **plotly.js** in your yew projects.
