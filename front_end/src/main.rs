//Four Line Dropper Frontend - A webapp that allows one to play a game of Four Line Dropper
//Copyright (C) 2022 Stephen Seo
//
//This program is free software: you can redistribute it and/or modify it under the terms of the GNU General Public License as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
//This program is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU General Public License for more details.
//
//You should have received a copy of the GNU General Public License along with this program. If not, see <https://www.gnu.org/licenses/>.
mod ai;
mod constants;
mod game_logic;
mod html_helper;
mod random_helper;
mod state;
mod yew_components;

use state::SharedState;
use yew::prelude::*;
use yew_components::Wrapper;

#[function_component]
pub fn App() -> Html {
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
    yew::Renderer::<App>::new().render();
}
