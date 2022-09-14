use yew::{function_component, html};

#[function_component(App)]
pub fn app() -> Html {
    html!(
        <h1 class="text-center text-5xl p-4">{"Hello, World!"}</h1>
    )
}
