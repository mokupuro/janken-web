mod components;

use yew::prelude::*;
use components::select_button::SelectButton;

#[function_component(App)]
fn app() -> Html {
    let selected = use_state(|| 0);

    html! {
        <div class="flex justify-center mt-3">
            <SelectButton selected={ selected.clone() } hand_type="グー" />
            <SelectButton selected={ selected.clone() } hand_type="チョキ" />
            <SelectButton selected={ selected.clone() } hand_type="パー" />
        </div>
    }
}

fn main() {
    yew::start_app::<App>();
}
