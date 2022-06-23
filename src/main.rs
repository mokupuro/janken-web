use yew::prelude::*;

#[function_component(App)]
fn app() -> Html {
    html! {
        <div>
            <h1 class="text-3xl font-bold text-center ">{"Hello, world!"}</h1>
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
