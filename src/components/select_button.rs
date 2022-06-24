use yew::{function_component, html, Properties, Callback};
use rand::Rng;

#[derive(Properties, PartialEq)]
pub struct SelectButtonProps {
    pub selected: yew::UseStateHandle<i32>,
    pub result: yew::UseStateHandle<i32>,
    pub hand_type: std::string::String,
}

#[function_component(SelectButton)]
pub fn select_button(props: &SelectButtonProps) -> Html {
    // FIXME: 数回は正常にrandが動くが、途中からwasmでエラーが出るのでその修正が必要
    let onclick = {
        let result = props.result.clone();
        let random_int = rand::thread_rng().gen_range(0, 3);
        Callback::from(move |_| result.set(random_int as i32))
    };

    html! {
      <button
        class="bg-white hover:bg-gray-100 text-gray-800 font-semibold py-2 px-4 border border-gray-400 rounded shadow"
        onclick={ onclick }
      >
        { &props.hand_type }
      </button>
    }
}
