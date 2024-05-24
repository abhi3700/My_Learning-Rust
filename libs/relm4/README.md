# Relm4

## Description

Relm4 is a GUI library inspired by Elm and based on GTK4.

A Relm4 application has three important types:

- The model type that stores the application state, the memory of our app.
The message type that describes which information can be sent to update the model.
- The widgets type that stores our widgets.

Also, there are two important functions:

- `update` receives a message and updates the model accordingly.
- `update_view` receives the updated model and updates the widgets accordingly.
- `#[watch]` attribute is used to update the property in the `view!` function. Example: `counter` value in a Counter App may be updated based on App inputs/events. It is the replacement of this code snippet:

```rust
// update the view based on the model
 fn update_view(&self, widgets: &mut Self::Widgets, _sender: ComponentSender<Self>) {
  widgets.label.set_text(&format!("Counter: {}", self.counter.to_string()));
 }
```

The app does all those things in a loop. It waits for messages and once a message is received, it runs `update` and then `view`.

![](../../img/relm4_elm_architecture.png)

<u>Cons</u>:

- Doesn't have much of component library which needs to be built from scratch using `DrawingArea` may be.
