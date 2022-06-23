mod components;

use yew::prelude::*;
use components::select_button::SelectButton;

#[function_component(App)]
fn app() -> Html {
    let selected = use_state(|| 0);
    let result = use_state(|| 3);

    html! {
        <>
            <div class="flex justify-center mt-3">
                { if *result == 0 {
                    html! {
                        <div class="text-center">
                            <h1 class="text-3xl font-bold">{"勝ち！"}</h1>
                        </div>
                    }
                } else if *result == 1 {
                    html! {
                        <div class="text-center">
                            <h1 class="text-3xl font-bold">{"負け！"}</h1>
                        </div>
                    }
                } else if *result == 2 {
                    html! {
                        <div class="text-center">
                            <h1 class="text-3xl font-bold">{"引き分け！"}</h1>
                        </div>
                    }
                } else {
                    html! {
                        {""}
                    }
                } }
            </div>
            <div class="flex justify-center mt-3">
                <SelectButton selected={ selected.clone() } hand_type="グー" />
                <SelectButton selected={ selected.clone() } hand_type="チョキ" />
                <SelectButton selected={ selected.clone() } hand_type="パー" />
            </div>
        </>
    }
}

fn main() {
    yew::start_app::<App>();
}
