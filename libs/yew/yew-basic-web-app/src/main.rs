use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
	html! {
		<>
		<div class="container">
			<h1>{"Basic Yew Web App"}</h1>
		</div>
		</>
	}
}

fn main() {
	yew::Renderer::<App>::new().render();
}
