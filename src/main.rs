mod components;

use yew::prelude::*;
use components::select_button::SelectButton;

#[function_component(App)]
fn app() -> Html {
    let selected = use_state(|| 0);

    html! {
        <div class="flex justify-center mt-3">
            <SelectButton selected={ selected.clone() } />
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
