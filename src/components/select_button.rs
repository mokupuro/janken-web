
use yew::{function_component, html, Properties, Callback};

#[function_component(SelectButton)]
pub fn select_button() -> Html {
    let onclick = {
        let selected = props.selected.clone();
        let content  = props.content.clone();
        Callback::from(move |_| selected.set(content.clone()))
    };

    html! {
      <act_style_button
        class="bg-white hover:bg-gray-100 text-gray-800 font-semibold py-2 px-4 border border-gray-400 rounded shadow"
      >
        { "グー" }
      </act_style_button>
    }
}
