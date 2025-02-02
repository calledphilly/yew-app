use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct ButtonProps {
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub color: String,
}

#[function_component]
pub fn Button(props: &ButtonProps) -> Html {
    let ButtonProps {
        children,
        onclick,
        color,
    } = props;
    html! {
        <button class={format!("bg-{}-500 hover:bg-{}-400 duration-400 text-xl w-fit px-10 py-3 rounded-xl", color, color)} {onclick}>{children.clone()}</button>
    }
}
