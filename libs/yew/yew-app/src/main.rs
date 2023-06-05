use std::vec;

use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    let show_book = use_state(|| false);
    let onclick = {
        let show_book = show_book.clone();
        move |_| {
            show_book.set(!(*show_book));
        }
    };

    // fn showorhide_button() -> Html {
    //     match s {}
    // }

    fn book(title: String) -> Html {
        html!(
            <li {title}></li>
        )
    }

    fn books() -> Html {
        let book_list = vec!["Lord of the Rings", "Game of Thrones"];

        html! (
            <div>
                <li>{book_list[0]}</li>
                <li>{book_list[1]}</li>
            </div>
        )
    }

    html! (
        <div>
            <button {onclick}>{ |x: bool| -> Html {
                if x {
                html!(
                    <div>{"Show"}</div>
                )
            } else {
                html!(
                    <div>{"Hide"}</div>
                )
            }} }</button>
            <p>{ *show_book }</p>
            <div>{books()}</div>
        </div>
    )
}

fn main() {
    yew::Renderer::<App>::new().render();
}
