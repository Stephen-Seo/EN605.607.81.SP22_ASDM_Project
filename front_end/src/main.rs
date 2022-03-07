mod ai;
mod constants;
mod state;
mod yew_components;

use state::SharedState;
use yew::prelude::*;
use yew_components::Wrapper;

#[function_component(App)]
pub fn app() -> Html {
    let ctx = use_state(SharedState::default);
    html! {
        <ContextProvider<SharedState> context={(*ctx).clone()}>
            <Wrapper />
        </ContextProvider<SharedState>>
    }
}

fn main() {
    // setup logging to browser console
    wasm_logger::init(wasm_logger::Config::default());

    // start webapp
    yew::start_app::<App>();
}
