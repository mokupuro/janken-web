
use yew::{function_component, html, Properties, Callback};

#[derive(Properties, PartialEq)]
pub struct SelectButtonProps {
    pub selected: yew::UseStateHandle<i32>,
}

#[function_component(SelectButton)]
pub fn select_button(props: &SelectButtonProps) -> Html {
    let onclick = {
        let selected = props.selected.clone();
        Callback::from(move |_| selected.set(0))
    };

    html! {
      <button
        class="bg-white hover:bg-gray-100 text-gray-800 font-semibold py-2 px-4 border border-gray-400 rounded shadow"
        { onclick }
      >
        { "グー" }
      </button>
    }
}
