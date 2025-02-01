use yew::prelude::*;
mod components;
use components::{button::Button, container::Container, footer::Footer};

#[function_component]
fn App() -> Html {
    let counter = use_state(|| 0);
    let up = {
        let counter = counter.clone();
        move |_| {
            let value = *counter + 1;
            counter.set(value);
        }
    };
    let down = {
        let counter = counter.clone();
        move |_| {
            let value = *counter - 1;
            counter.set(value);
        }
    };
    html! {
        <Container class="relative min-h-dvh bg-neutral-800 text-white flex flex-col">
            <div class="grow flex flex-col gap-y-7 justify-center items-center">
                <p class="text-9xl">{ *counter }</p>
                <div class="flex gap-x-10">
                    <Button onclick={up} color="stone" >{"+ 1"}</Button>
                    <Button onclick={down} color="stone">{"- 1"}</Button>
                </div>
            </div>
            <Footer/>
        </Container>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
