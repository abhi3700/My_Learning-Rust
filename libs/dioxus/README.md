# Dioxus

## Overview

- Platforms:
  - Native: Desktop/Mobile
  - Web: Browser
  - LiveView
  - TUI i.e. for terminal.
- Use TailwindCSS for styling of components. You can also use [Flowbite](https://flowbite.com/#components) which is built on top of TailwindCSS.
- Like Yew, has support for both CSR & SSR.
- Yew has:
- Comparo with:
  - <u>Yew</u>: famous for single page web app, whereas dioxus is for multi page web app.
  - <u>Leptos</u>: Both follow similar objective except for the Leptos' syntax that's alien to Rustaceans. Dioxus syntax is more close to Rust & is easy to write.

## Installation

- dioxus-cli: `$ cargo install dioxus-cli`
- [extension](https://marketplace.visualstudio.com/items?itemName=DioxusLabs.dioxus) for VSCode IDE.

## Quickstart

[Source](https://dioxuslabs.com/learn/0.5/getting_started)

1. `$ dx new dioxus-demo`
2. `$ cd dioxus-demo`
3. `$ dx serve` or `dx serve --platform web` or `dx serve --hot-reload` (for fast reloading in dev mode)

There are many more commands to explore:

```sh
Build, Bundle & Ship Dioxus Apps

Usage: dx [OPTIONS] <COMMAND>

Commands:
  build      Build the Rust WASM app and all of its assets
  translate  Translate some source file into Dioxus code
  serve      Build, watch & serve the Rust WASM app and all of its assets
  new        Create a new project for Dioxus.a
  init       Init a new project for Dioxus in an existing directory. Will attempt to keep your project in a good state
  clean      Clean output artifacts
  bundle     Bundle the Rust desktop app and all of its assets
  fmt        Format some rsx
  check      Check the Rust files in the project for issues
  config     Dioxus config file controls
  help       Print this message or the help of the given subcommand(s)

Options:
  -v               Enable verbose logging
      --bin <BIN>  Specify bin target
  -h, --help       Print help
  -V, --version    Print version
```

## References

- <https://github.com/DioxusLabs/dioxus>
- [awesome-dioxus](https://dioxuslabs.com/awesome/)
- [documentation](https://dioxuslabs.com/learn/0.5/)
- [Community projects](https://github.com/dioxus-community/)
- [dioxus-sdk](https://github.com/DioxusLabs/sdk): Get to use dioxus features into some other project
- Component libraries:
  - <https://github.com/RubixDev/material-dioxus>
  - <https://github.com/matthunz/dioxus-material>
