use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>{"FluxGB is running!"}</div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}

