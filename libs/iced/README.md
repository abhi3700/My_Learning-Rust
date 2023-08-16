# Iced

## Overview

**Iced** is a cross-platform GUI library for Rust focused on simplicity and type-safety. Inspired by Elm, it is based on a self-contained event loop where widgets and event handlers are simply functions that transform state. This makes it easy to create simple, predictable interfaces.

## Getting Started

Follow this [demo](./demo/)

## Architecture

Follow this [diagram](./iced_arch.drawio).

Inspired by The [Elm Architecture](https://guide.elm-lang.org/architecture/), Iced expects you to split user interfaces into four different concepts:

- **State** — the state of your application
- **Messages** — user interactions or meaningful events that you care about
- **View logic** — a way to display your state as widgets that may produce messages on user interaction
- **Update logic** — a way to react to messages and update your state

## References

- [Github](https://github.com/iced-rs/iced)
- [Examples](https://github.com/iced-rs/iced/tree/master/examples)
- [Awesome-iced](https://github.com/iced-rs/awesome-iced)
