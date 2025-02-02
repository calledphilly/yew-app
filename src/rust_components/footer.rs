use yew::prelude::*;

#[function_component]
pub fn Footer() -> Html {
    html! {
        <footer class="absolute bottom-0 inset-x-0 bg-neutral-800 text-center py-4">
            <p>{ "©Yannis Bikouta" }</p>
        </footer>
    }
}