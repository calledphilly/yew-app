use yew::prelude::*;
mod rust_components;
use reqwest;
use rust_components::{button::Button, container::Container, footer::Footer};

#[function_component]
fn App() -> Html {
    let data = use_state(|| None);

    // Effet pour exécuter la requête après le premier rendu :
    {
        let data = data.clone();
        use_effect_with(data.clone(), move |_| {
            wasm_bindgen_futures::spawn_local(async move {
                let response = reqwest::get("http://127.0.0.1:8080/api/users/").await;
                if let Ok(response) = response {
                    let text = response
                        .text()
                        .await
                        .unwrap_or_else(|_| "Erreur".to_string());
                    data.set(Some(text));
                    println!("this data : {:?}",data);
                }
            })
        });
    }
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
            <div class="grow flex flex-col gap-y-15 justify-center items-center">
                <p class="text-9xl">{ *counter }</p>
                <div class="flex gap-x-10">
                    <Button onclick={down} color="stone">{"-"}</Button>
                    <Button onclick={up} color="stone" >{"+"}</Button>
                </div>
                { data.as_ref().map_or(html! { <p class="text-base">{ "Aucune donnée" }</p> }, |value| html! { <p class="text-base">{ value }</p> }) }
            </div>
            <Footer/>
        </Container>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
