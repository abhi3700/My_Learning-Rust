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
  > To update by force: `cargo install dioxus-cli --force`.
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

### Commands

- `dx serve --hot-reload`: Hot reload the web page.

## Coding

- For "**web** + **vanilla** + **router**" option, in `Cargo.toml` file, it's like this:

  ```toml
  ...
  dioxus = { version = "0.5.6", features = ["web", "router"] }
  ```

  > There is no `tailwind.config.js` file as it was not opted for during `$ dx new hello`.
- Selecting desktop over web as default platform would
  - change the feature in `Cargo.toml` file and,
  - set desktop as default platform in `Dioxus.toml` file.

### Deploy

- For running on different platform, you need to pass the value (web, desktop, ..) in `platform` flag like: `$ dx serve --platform desktop`. This would override whichever default platform selected during project creation with `$ dx new ...`.
- Dioxus is soon coming with feature <kbd>Deploy</kbd> as provided by **Vercel**.

### Design

- Use TailwindCSS. [Cheatsheet](https://www.creative-tim.com/twcomponents/cheatsheet) instead of adding `class` manually in "**assets/main.css**" file.
  - [Guide](https://dioxuslabs.com/learn/0.5/cookbook/tailwind)
  In the VSCode editor, corresponding to tailwindCSS extension, add this to User Settings:

  ```json
  "tailwindCSS.experimental.classRegex": [
      "class: \"(.*)\""
  ],
  "tailwindCSS.includeLanguages": {
      "rust": "html"
  },
  ```

- During project creation via `$ dx new ..`, prefer **tailwind** option over **vanilla** for CSS.
  - Then, follow this:
    1. create `public/tailwind.css` file.
    2. `cargo add manganis`
    3. Add this line to "`main.rs`" file:

      ```rust
      use self::manganis;
      ...
      const _TAILWIND_URL: &str = manganis::mg!(file("./public/tailwind.css"));
      ```

- Using **vanilla CSS**, a button element could be written like this:

```rust
#[component]
pub(crate) fn Counter(id: i32) -> Element {
 let mut count = use_signal(|| id);

 // NOTE: Unnecessary when using tailwind CSS.
 let mut hovered = use_signal(|| false); // Track hover state

 rsx! {
    button {
     // NOTE: It's CSS equivalent is working. Not sure how to add hover, so added "onmouseenter", "onmouseleave" fields.
     style: format!(
      "background-color: {}; color: #ffffff; font-weight: bold; padding: 0.5rem 1rem; border-radius: 0.25rem; border: none;",
      if hovered() { "#2563eb" } else { "#3B82F6" },
     ),
     onmouseenter: move |_| hovered.set(true),
     onmouseleave: move |_| hovered.set(false),
     onclick: move |_| count -= 1,
     "Dislike 👎"
    }
   }
}
```

- Using **tailwind CSS**, a button element could be written like this:

```rust
#[component]
pub(crate) fn Counter(id: i32) -> Element {
 let mut count = use_signal(|| id);

 rsx! {
    button {
     // FIXME: tailwind css is not working.
     class: "bg-blue-500 hover:bg-blue-700 text-white font-bold py-2 px-4 rounded",
     "Dislike 👎"
    }
   }
}
```

## Troubleshoot

### 1. Error: Hot reload not working especially after TailwindCSS

- _Cause_: In a tailwind CSS project, Hot reload functionality doesn't work with setup done in [playg](./playg/) project.
- _Solution_: Just need to re-run `$ dx serve`.

## References

- <https://github.com/DioxusLabs/dioxus>
- [awesome-dioxus](https://dioxuslabs.com/awesome/)
- [documentation](https://dioxuslabs.com/learn/0.5/)
- [Community projects](https://github.com/dioxus-community/)
- [dioxus-sdk](https://github.com/DioxusLabs/sdk): Get to use dioxus features into some other project
- Component libraries:
  - <https://github.com/matthunz/dioxus-material>

### Videos

- [Cross Platform Development using RUST | Dioxus YT Playlist](https://www.youtube.com/playlist?list=PLDi2liHqCnVqDGWduQ2NuGbEX64-V0-sc)
