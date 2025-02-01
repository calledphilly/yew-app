use yew::prelude::*;

#[derive(PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub class: String,
}

#[function_component]
pub fn Container(props: &Props) -> Html {
    let Props { children, class } = props;
    html! {
      <div  class={format!("{} w-full", class)}>
        <div class="max-w-6xl m-auto">
          {children.clone()}
        </div>
      </div>
    }
}
