use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <h1>{ "Veni Vidi Vici.dev" }</h1>
    }
}

fn main() {
    yew::start_app::<App>();
}
