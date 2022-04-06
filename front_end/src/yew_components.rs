use crate::ai::{get_ai_choice, AIDifficulty};
use crate::constants::{
    AI_CHOICE_DURATION_MILLIS, BACKEND_TICK_DURATION_MILLIS, BACKEND_URL, COLS,
    INFO_TEXT_MAX_ITEMS, ROWS,
};
use crate::game_logic::{check_win_draw, WinType};
use crate::html_helper::{
    append_to_info_text, create_json_request, element_append_class, element_remove_class,
    get_window_document, send_to_backend,
};
use crate::random_helper::get_seeded_random;
use crate::state::{
    BoardState, GameState, MainMenuMessage, PairingRequestResponse, SharedState, Turn,
};

use std::cell::Cell;
use std::collections::HashMap;
use std::rc::Rc;

use js_sys::{Function, Promise};
use wasm_bindgen::JsCast;
use web_sys::Response;

use serde_json::Value as SerdeJSONValue;

use wasm_bindgen_futures::JsFuture;

use yew::prelude::*;

pub struct MainMenu {}

impl Component for MainMenu {
    type Message = MainMenuMessage;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let (shared, _) = ctx
            .link()
            .context::<SharedState>(Callback::noop())
            .expect("state to be set");
        match shared.game_state.get() {
            GameState::MainMenu => {
                let player_type: Turn;
                {
                    let mut rng = get_seeded_random().expect("Random should be available");
                    player_type = if rng.rand_range(0..2) == 0 {
                        Turn::CyanPlayer
                    } else {
                        Turn::MagentaPlayer
                    };
                }

                let easy_player_type = player_type;
                let normal_player_type = player_type;
                let hard_player_type = player_type;

                let onclick_singleplayer_easy = ctx.link().callback(move |_| {
                    MainMenuMessage::SinglePlayer(easy_player_type, AIDifficulty::Easy)
                });
                let onclick_singleplayer_normal = ctx.link().callback(move |_| {
                    MainMenuMessage::SinglePlayer(normal_player_type, AIDifficulty::Normal)
                });
                let onclick_singleplayer_hard = ctx.link().callback(move |_| {
                    MainMenuMessage::SinglePlayer(hard_player_type, AIDifficulty::Hard)
                });

                let onclick_local_multiplayer =
                    ctx.link().callback(|_| MainMenuMessage::LocalMultiplayer);

                let onclick_networked_multiplayer = ctx
                    .link()
                    .callback(|_| MainMenuMessage::NetworkedMultiplayer);

                html! {
                    <div class={"menu"} id={"mainmenu"}>
                        <b class={"menuText"}>{"Please pick a game mode."}</b>
                        <div class={"singlePlayerMenu"}>
                            <button class={"menuSinglePlayerEasy"} onclick={onclick_singleplayer_easy}>
                                {"Singleplayer Easy"}
                            </button>
                            <button class={"menuSinglePlayerNormal"} onclick={onclick_singleplayer_normal}>
                                {"Singleplayer Normal"}
                            </button>
                            <button class={"menuSinglePlayerHard"} onclick={onclick_singleplayer_hard}>
                                {"Singleplayer Hard"}
                            </button>
                        </div>
                        <button class={"menuLocalMultiplayer"} onclick={onclick_local_multiplayer}>
                            {"Local Multiplayer"}
                        </button>
                        <button class={"menuMultiplayer"} onclick={onclick_networked_multiplayer}>
                            {"Networked Multiplayer"}
                        </button>
                    </div>
                }
            }
            _ => html! {
                <div class={"hidden_menu"} id={"mainmenu"}>
                </div>
            },
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let (shared, _) = ctx
            .link()
            .context::<SharedState>(Callback::noop())
            .expect("state to be set");
        let window = web_sys::window().expect("no window exists");
        let document = window.document().expect("window should have a document");

        shared.game_state.replace(msg.into());
        if shared.game_state.get() != GameState::MainMenu {
            let mainmenu = document
                .get_element_by_id("mainmenu")
                .expect("mainmenu should exist");
            mainmenu.set_class_name("hidden_menu");
            mainmenu.set_inner_html("");

            let info_text_turn = document
                .get_element_by_id("info_text1")
                .expect("info_text1 should exist");

            match shared.game_state.get() {
                GameState::SinglePlayer(turn, _) => {
                    if shared.turn.get() == turn {
                        info_text_turn.set_inner_html(
                            "<p><b class=\"cyan\">It is CyanPlayer's (player) Turn</b></p>",
                        );
                    } else {
                        info_text_turn.set_inner_html(
                            "<p><b class=\"cyan\">It is CyanPlayer's (ai) Turn</b></p>",
                        );
                        // AI player starts first
                        ctx.link()
                            .get_parent()
                            .expect("Wrapper should be parent of MainMenu")
                            .clone()
                            .downcast::<Wrapper>()
                            .send_message(WrapperMsg::AIChoice);
                    }
                }
                GameState::NetworkedMultiplayer {
                    paired: _,
                    current_side: _,
                    current_turn: _,
                } => {
                    // start the Wrapper Tick loop
                    ctx.link()
                        .get_parent()
                        .expect("Wrapper should be a parent of MainMenu")
                        .clone()
                        .downcast::<Wrapper>()
                        .send_message(WrapperMsg::BackendTick);
                }
                _ => {
                    info_text_turn
                        .set_inner_html("<p><b class=\"cyan\">It is CyanPlayer's Turn</b></p>");
                }
            }
        }

        true
    }
}

pub struct Slot {}

pub enum SlotMessage {
    Press,
}

#[derive(Clone, PartialEq, Properties)]
pub struct SlotProperties {
    idx: u8,
    state: Rc<Cell<BoardState>>,
    placed: Rc<Cell<bool>>,
}

impl Component for Slot {
    type Message = SlotMessage;
    type Properties = SlotProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let idx = ctx.props().idx;
        let state = ctx.props().state.as_ref().get();
        let onclick = ctx.link().callback(move |_| SlotMessage::Press);
        let col = idx % COLS;
        let row = idx / COLS;
        let place = if ctx.props().placed.get() && !state.is_win() {
            "placed"
        } else {
            ""
        };
        ctx.props().placed.replace(false);
        html! {
            <button class={format!("slot {} r{} c{} {}", state, row, col, place)} id={format!("slot{}", idx)} onclick={onclick}>
            {idx}
            </button>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let (shared, _) = ctx
            .link()
            .context::<SharedState>(Callback::noop())
            .expect("state to be set");

        match shared.game_state.get() {
            GameState::MainMenu => return false,
            GameState::SinglePlayer(_, _) | GameState::LocalMultiplayer => (),
            GameState::NetworkedMultiplayer {
                paired,
                current_side,
                current_turn,
            } => {
                // notify Wrapper with picked slot
                if let Some(p) = ctx.link().get_parent() {
                    p.clone()
                        .downcast::<Wrapper>()
                        .send_message(WrapperMsg::BackendRequest {
                            place: ctx.props().idx,
                        });
                }
            }
            GameState::PostGameResults(_) => return false,
        }
        if shared.game_state.get() == GameState::MainMenu {
            return false;
        }

        match msg {
            SlotMessage::Press => {
                // notify Wrapper with message
                let msg = WrapperMsg::Pressed(ctx.props().idx);
                if let Some(p) = ctx.link().get_parent() {
                    p.clone().downcast::<Wrapper>().send_message(msg);
                }
            }
        }

        true
    }
}

pub struct Wrapper {
    player_id: Option<u32>,
    place_request: Option<u8>,
    do_backend_tick: bool,
}

impl Wrapper {
    fn defer_message(
        &self,
        ctx: &Context<Self>,
        msg: <Wrapper as Component>::Message,
        millis: i32,
    ) {
        ctx.link().send_future(async move {
            let promise = Promise::new(&mut |resolve: js_sys::Function, _reject| {
                let window = web_sys::window();
                if window.is_none() {
                    resolve.call0(&resolve).ok();
                    return;
                }
                let window = window.unwrap();
                if window
                    .set_timeout_with_callback_and_timeout_and_arguments_0(&resolve, millis)
                    .is_err()
                {
                    resolve.call0(&resolve).ok();
                }
            });
            let js_fut = JsFuture::from(promise);
            js_fut.await.ok();
            msg
        });
    }

    fn get_networked_player_id(&mut self, ctx: &Context<Self>) {
        // make a request to get the player_id
        ctx.link().send_future(async {
            let mut json_entries = HashMap::new();
            json_entries.insert("type".into(), "pairing_request".into());

            let send_to_backend_result = send_to_backend(json_entries).await;
            if let Err(e) = send_to_backend_result {
                return WrapperMsg::BackendResponse(BREnum::Error(format!("{:?}", e)));
            }

            let request_result: Result<PairingRequestResponse, _> =
                serde_json::from_str(&send_to_backend_result.unwrap());
            if let Err(e) = request_result {
                return WrapperMsg::BackendResponse(BREnum::Error(format!("{:?}", e)));
            }
            let request = request_result.unwrap();

            if request.r#type != "pairing_response" {
                return WrapperMsg::BackendResponse(BREnum::Error(
                    "Backend returned invalid type for pairing_request".into(),
                ));
            }

            // set up onbeforeunload to disconnect with the received id
            let function = Function::new_no_args(&format!(
                "
                window.addEventListener(\"beforeunload\", function(event) {{
                    let xhr = new XMLHttpRequest();
                    xhr.open('POST', '{}');
                    xhr.send('{{\"type\": \"disconnect\", \"id\": {}}}');
                }});
            ",
                BACKEND_URL, request.id
            ));
            function.call0(&function).ok();

            if let Some(color) = request.color {
                WrapperMsg::BackendResponse(BREnum::GotID(
                    request.id,
                    if color == "cyan" {
                        Some(Turn::CyanPlayer)
                    } else {
                        Some(Turn::MagentaPlayer)
                    },
                ))
            } else {
                WrapperMsg::BackendResponse(BREnum::GotID(request.id, None))
            }
        });
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum BREnum {
    Error(String),
    GotID(u32, Option<Turn>),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum WrapperMsg {
    Pressed(u8),
    AIPressed(u8),
    AIChoice,
    AIChoiceImpl,
    BackendTick,
    BackendRequest { place: u8 },
    BackendResponse(BREnum),
}

impl WrapperMsg {
    fn is_ai_pressed(self) -> bool {
        matches!(self, WrapperMsg::AIPressed(_))
    }
}

impl Component for Wrapper {
    type Message = WrapperMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            player_id: None,
            place_request: None,
            do_backend_tick: true,
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let (shared, _) = ctx
            .link()
            .context::<SharedState>(Callback::noop())
            .expect("state to be set");
        html! {
            <div class="wrapper">
                <MainMenu />
                <Slot idx=0 state={shared.board[0].clone()} placed={shared.placed[0].clone()} />
                <Slot idx=1 state={shared.board[1].clone()} placed={shared.placed[1].clone()} />
                <Slot idx=2 state={shared.board[2].clone()} placed={shared.placed[2].clone()} />
                <Slot idx=3 state={shared.board[3].clone()} placed={shared.placed[3].clone()} />
                <Slot idx=4 state={shared.board[4].clone()} placed={shared.placed[4].clone()} />
                <Slot idx=5 state={shared.board[5].clone()} placed={shared.placed[5].clone()} />
                <Slot idx=6 state={shared.board[6].clone()} placed={shared.placed[6].clone()} />
                <Slot idx=7 state={shared.board[7].clone()} placed={shared.placed[7].clone()} />
                <Slot idx=8 state={shared.board[8].clone()} placed={shared.placed[8].clone()} />
                <Slot idx=9 state={shared.board[9].clone()} placed={shared.placed[9].clone()} />
                <Slot idx=10 state={shared.board[10].clone()} placed={shared.placed[10].clone()} />
                <Slot idx=11 state={shared.board[11].clone()} placed={shared.placed[11].clone()} />
                <Slot idx=12 state={shared.board[12].clone()} placed={shared.placed[12].clone()} />
                <Slot idx=13 state={shared.board[13].clone()} placed={shared.placed[13].clone()} />
                <Slot idx=14 state={shared.board[14].clone()} placed={shared.placed[14].clone()} />
                <Slot idx=15 state={shared.board[15].clone()} placed={shared.placed[15].clone()} />
                <Slot idx=16 state={shared.board[16].clone()} placed={shared.placed[16].clone()} />
                <Slot idx=17 state={shared.board[17].clone()} placed={shared.placed[17].clone()} />
                <Slot idx=18 state={shared.board[18].clone()} placed={shared.placed[18].clone()} />
                <Slot idx=19 state={shared.board[19].clone()} placed={shared.placed[19].clone()} />
                <Slot idx=20 state={shared.board[20].clone()} placed={shared.placed[20].clone()} />
                <Slot idx=21 state={shared.board[21].clone()} placed={shared.placed[21].clone()} />
                <Slot idx=22 state={shared.board[22].clone()} placed={shared.placed[22].clone()} />
                <Slot idx=23 state={shared.board[23].clone()} placed={shared.placed[23].clone()} />
                <Slot idx=24 state={shared.board[24].clone()} placed={shared.placed[24].clone()} />
                <Slot idx=25 state={shared.board[25].clone()} placed={shared.placed[25].clone()} />
                <Slot idx=26 state={shared.board[26].clone()} placed={shared.placed[26].clone()} />
                <Slot idx=27 state={shared.board[27].clone()} placed={shared.placed[27].clone()} />
                <Slot idx=28 state={shared.board[28].clone()} placed={shared.placed[28].clone()} />
                <Slot idx=29 state={shared.board[29].clone()} placed={shared.placed[29].clone()} />
                <Slot idx=30 state={shared.board[30].clone()} placed={shared.placed[30].clone()} />
                <Slot idx=31 state={shared.board[31].clone()} placed={shared.placed[31].clone()} />
                <Slot idx=32 state={shared.board[32].clone()} placed={shared.placed[32].clone()} />
                <Slot idx=33 state={shared.board[33].clone()} placed={shared.placed[33].clone()} />
                <Slot idx=34 state={shared.board[34].clone()} placed={shared.placed[34].clone()} />
                <Slot idx=35 state={shared.board[35].clone()} placed={shared.placed[35].clone()} />
                <Slot idx=36 state={shared.board[36].clone()} placed={shared.placed[36].clone()} />
                <Slot idx=37 state={shared.board[37].clone()} placed={shared.placed[37].clone()} />
                <Slot idx=38 state={shared.board[38].clone()} placed={shared.placed[38].clone()} />
                <Slot idx=39 state={shared.board[39].clone()} placed={shared.placed[39].clone()} />
                <Slot idx=40 state={shared.board[40].clone()} placed={shared.placed[40].clone()} />
                <Slot idx=41 state={shared.board[41].clone()} placed={shared.placed[41].clone()} />
                <Slot idx=42 state={shared.board[42].clone()} placed={shared.placed[42].clone()} />
                <Slot idx=43 state={shared.board[43].clone()} placed={shared.placed[43].clone()} />
                <Slot idx=44 state={shared.board[44].clone()} placed={shared.placed[44].clone()} />
                <Slot idx=45 state={shared.board[45].clone()} placed={shared.placed[45].clone()} />
                <Slot idx=46 state={shared.board[46].clone()} placed={shared.placed[46].clone()} />
                <Slot idx=47 state={shared.board[47].clone()} placed={shared.placed[47].clone()} />
                <Slot idx=48 state={shared.board[48].clone()} placed={shared.placed[48].clone()} />
                <Slot idx=49 state={shared.board[49].clone()} placed={shared.placed[49].clone()} />
                <Slot idx=50 state={shared.board[50].clone()} placed={shared.placed[50].clone()} />
                <Slot idx=51 state={shared.board[51].clone()} placed={shared.placed[51].clone()} />
                <Slot idx=52 state={shared.board[52].clone()} placed={shared.placed[52].clone()} />
                <Slot idx=53 state={shared.board[53].clone()} placed={shared.placed[53].clone()} />
                <Slot idx=54 state={shared.board[54].clone()} placed={shared.placed[54].clone()} />
                <Slot idx=55 state={shared.board[55].clone()} placed={shared.placed[55].clone()} />
                <div class="info_text_wrapper">
                    <InfoText id=0 />
                </div>
                <div class="info_text_turn_wrapper">
                    <InfoText id=1 />
                </div>
            </div> // wrapper
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let (shared, _) = ctx
            .link()
            .context::<SharedState>(Callback::noop())
            .expect("state to be set");
        let (_window, document) =
            get_window_document().expect("Should be able to get Window and Document");

        match msg {
            WrapperMsg::Pressed(idx) | WrapperMsg::AIPressed(idx) => {
                let mut bottom_idx = idx;
                let mut placed = false;
                let current_player = shared.turn.get();

                // check if player can make a move
                if !msg.is_ai_pressed() {
                    match shared.game_state.get() {
                        GameState::MainMenu => (),
                        GameState::SinglePlayer(turn, _) => {
                            if current_player != turn {
                                return false;
                            }
                        }
                        GameState::LocalMultiplayer => (),
                        GameState::NetworkedMultiplayer {
                            paired,
                            current_side,
                            current_turn,
                        } => {
                            // TODO
                        }
                        GameState::PostGameResults(_) => (),
                    }
                }

                // check if clicked on empty slot
                if shared.board[idx as usize].get().is_empty() {
                    // get bottom-most empty slot
                    while bottom_idx + COLS < ROWS * COLS
                        && shared.board[(bottom_idx + COLS) as usize].get().is_empty()
                    {
                        bottom_idx += COLS;
                    }

                    // apply current player's color to bottom-most empty slot
                    shared.board[bottom_idx as usize].replace(shared.turn.get().into());

                    let current_board_state = shared.board[bottom_idx as usize].get();

                    // swap turn
                    shared.turn.replace(shared.turn.get().get_opposite());

                    // get handle to slot
                    if let Some(slot) = document.get_element_by_id(&format!("slot{bottom_idx}")) {
                        // set slot info
                        slot.set_class_name(&format!(
                            "slot {} r{} c{} placed",
                            current_board_state,
                            bottom_idx / COLS,
                            bottom_idx % COLS
                        ));
                        shared.placed[bottom_idx as usize].replace(true);
                    }

                    placed = true;
                }

                // info text below the grid
                {
                    let output_str = match placed {
                        true => format!("{} placed into slot {}", current_player, bottom_idx),
                        false => "Invalid place to insert".into(),
                    };

                    let text_append_result = append_to_info_text(
                        &document,
                        "info_text0",
                        &output_str,
                        INFO_TEXT_MAX_ITEMS,
                    );
                    if let Err(e) = text_append_result {
                        log::warn!("ERROR: text append to info_text0 failed: {}", e);
                    }
                }

                // info text right of the grid
                {
                    let turn = shared.turn.get();
                    let output_str =
                        if let GameState::SinglePlayer(player_turn, _) = shared.game_state.get() {
                            if shared.turn.get() == player_turn {
                                format!(
                                    "<b class=\"{}\">It is {}'s (player) turn</b>",
                                    turn.get_color(),
                                    turn
                                )
                            } else {
                                format!(
                                    "<b class=\"{}\">It is {}'s (ai) turn</b>",
                                    turn.get_color(),
                                    turn
                                )
                            }
                        } else {
                            format!(
                                "<b class=\"{}\">It is {}'s turn</b>",
                                turn.get_color(),
                                turn
                            )
                        };

                    let text_append_result =
                        append_to_info_text(&document, "info_text1", &output_str, 1);
                    if let Err(e) = text_append_result {
                        log::warn!("ERROR: text append to info_text1 failed: {}", e);
                    }
                }

                // check for win
                let check_win_draw_opt = check_win_draw(&shared.board);
                if let Some((endgame_state, win_type)) = check_win_draw_opt {
                    if endgame_state == BoardState::Empty {
                        // draw
                        let text_append_result = append_to_info_text(
                            &document,
                            "info_text0",
                            "Game ended in a draw",
                            INFO_TEXT_MAX_ITEMS,
                        );
                        if let Err(e) = text_append_result {
                            log::warn!("ERROR: text append to info_text0 failed: {}", e);
                        }
                        shared
                            .game_state
                            .replace(GameState::PostGameResults(BoardState::Empty));
                    } else {
                        // a player won
                        let turn = Turn::from(endgame_state);
                        let text_string =
                            format!("<b class=\"{}\">{} has won</b>", turn.get_color(), turn);
                        let text_append_result = append_to_info_text(
                            &document,
                            "info_text0",
                            &text_string,
                            INFO_TEXT_MAX_ITEMS,
                        );
                        if let Err(e) = text_append_result {
                            log::warn!("ERROR: text append to info_text0 failed: {}", e);
                        }

                        shared
                            .game_state
                            .replace(GameState::PostGameResults(turn.into()));

                        match win_type {
                            WinType::Horizontal(idx) => {
                                let placed_class_erase_result = element_remove_class(
                                    &document,
                                    &format!("slot{}", idx),
                                    "placed",
                                );
                                if let Err(e) = placed_class_erase_result {
                                    log::warn!("ERROR: element_remove_class failed: {}", e);
                                }
                                let placed_class_erase_result = element_remove_class(
                                    &document,
                                    &format!("slot{}", idx + 1),
                                    "placed",
                                );
                                if let Err(e) = placed_class_erase_result {
                                    log::warn!("ERROR: element_remove_class failed: {}", e);
                                }
                                let placed_class_erase_result = element_remove_class(
                                    &document,
                                    &format!("slot{}", idx + 2),
                                    "placed",
                                );
                                if let Err(e) = placed_class_erase_result {
                                    log::warn!("ERROR: element_remove_class failed: {}", e);
                                }
                                let placed_class_erase_result = element_remove_class(
                                    &document,
                                    &format!("slot{}", idx + 3),
                                    "placed",
                                );
                                if let Err(e) = placed_class_erase_result {
                                    log::warn!("ERROR: element_remove_class failed: {}", e);
                                }

                                let append_result =
                                    element_append_class(&document, &format!("slot{}", idx), "win");
                                if let Err(e) = append_result {
                                    log::warn!("ERROR: element_append_class failed: {}", e);
                                }
                                let append_result = element_append_class(
                                    &document,
                                    &format!("slot{}", idx + 1),
                                    "win",
                                );
                                if let Err(e) = append_result {
                                    log::warn!("ERROR: element_append_class failed: {}", e);
                                }
                                let append_result = element_append_class(
                                    &document,
                                    &format!("slot{}", idx + 2),
                                    "win",
                                );
                                if let Err(e) = append_result {
                                    log::warn!("ERROR: element_append_class failed: {}", e);
                                }
                                let append_result = element_append_class(
                                    &document,
                                    &format!("slot{}", idx + 3),
                                    "win",
                                );
                                if let Err(e) = append_result {
                                    log::warn!("ERROR: element_append_class failed: {}", e);
                                }

                                shared.board[idx].replace(shared.board[idx].get().into_win());
                                shared.board[idx + 1]
                                    .replace(shared.board[idx + 1].get().into_win());
                                shared.board[idx + 2]
                                    .replace(shared.board[idx + 2].get().into_win());
                                shared.board[idx + 3]
                                    .replace(shared.board[idx + 3].get().into_win());
                            }
                            WinType::Vertical(idx) => {
                                let placed_class_erase_result = element_remove_class(
                                    &document,
                                    &format!("slot{}", idx),
                                    "placed",
                                );
                                if let Err(e) = placed_class_erase_result {
                                    log::warn!("ERROR: element_remove_class failed: {}", e);
                                }
                                let placed_class_erase_result = element_remove_class(
                                    &document,
                                    &format!("slot{}", idx + (COLS as usize)),
                                    "placed",
                                );
                                if let Err(e) = placed_class_erase_result {
                                    log::warn!("ERROR: element_remove_class failed: {}", e);
                                }
                                let placed_class_erase_result = element_remove_class(
                                    &document,
                                    &format!("slot{}", idx + 2 * (COLS as usize)),
                                    "placed",
                                );
                                if let Err(e) = placed_class_erase_result {
                                    log::warn!("ERROR: element_remove_class failed: {}", e);
                                }
                                let placed_class_erase_result = element_remove_class(
                                    &document,
                                    &format!("slot{}", idx + 3 * (COLS as usize)),
                                    "placed",
                                );
                                if let Err(e) = placed_class_erase_result {
                                    log::warn!("ERROR: element_remove_class failed: {}", e);
                                }

                                let append_result =
                                    element_append_class(&document, &format!("slot{}", idx), "win");
                                if let Err(e) = append_result {
                                    log::warn!("ERROR: element_append_class failed: {}", e);
                                }
                                let append_result = element_append_class(
                                    &document,
                                    &format!("slot{}", idx + (COLS as usize)),
                                    "win",
                                );
                                if let Err(e) = append_result {
                                    log::warn!("ERROR: element_append_class failed: {}", e);
                                }
                                let append_result = element_append_class(
                                    &document,
                                    &format!("slot{}", idx + 2 * (COLS as usize)),
                                    "win",
                                );
                                if let Err(e) = append_result {
                                    log::warn!("ERROR: element_append_class failed: {}", e);
                                }
                                let append_result = element_append_class(
                                    &document,
                                    &format!("slot{}", idx + 3 * (COLS as usize)),
                                    "win",
                                );
                                if let Err(e) = append_result {
                                    log::warn!("ERROR: element_append_class failed: {}", e);
                                }

                                shared.board[idx].replace(shared.board[idx].get().into_win());
                                shared.board[idx + (COLS as usize)]
                                    .replace(shared.board[idx + (COLS as usize)].get().into_win());
                                shared.board[idx + 2 * (COLS as usize)].replace(
                                    shared.board[idx + 2 * (COLS as usize)].get().into_win(),
                                );
                                shared.board[idx + 3 * (COLS as usize)].replace(
                                    shared.board[idx + 3 * (COLS as usize)].get().into_win(),
                                );
                            }
                            WinType::DiagonalUp(idx) => {
                                let placed_class_erase_result = element_remove_class(
                                    &document,
                                    &format!("slot{}", idx),
                                    "placed",
                                );
                                if let Err(e) = placed_class_erase_result {
                                    log::warn!("ERROR: element_remove_class failed: {}", e);
                                }
                                let placed_class_erase_result = element_remove_class(
                                    &document,
                                    &format!("slot{}", idx + 1 - (COLS as usize)),
                                    "placed",
                                );
                                if let Err(e) = placed_class_erase_result {
                                    log::warn!("ERROR: element_remove_class failed: {}", e);
                                }
                                let placed_class_erase_result = element_remove_class(
                                    &document,
                                    &format!("slot{}", idx + 2 - 2 * (COLS as usize)),
                                    "placed",
                                );
                                if let Err(e) = placed_class_erase_result {
                                    log::warn!("ERROR: element_remove_class failed: {}", e);
                                }
                                let placed_class_erase_result = element_remove_class(
                                    &document,
                                    &format!("slot{}", idx + 3 - 3 * (COLS as usize)),
                                    "placed",
                                );
                                if let Err(e) = placed_class_erase_result {
                                    log::warn!("ERROR: element_remove_class failed: {}", e);
                                }

                                let append_result =
                                    element_append_class(&document, &format!("slot{}", idx), "win");
                                if let Err(e) = append_result {
                                    log::warn!("ERROR: element_append_class failed: {}", e);
                                }
                                let append_result = element_append_class(
                                    &document,
                                    &format!("slot{}", idx + 1 - (COLS as usize)),
                                    "win",
                                );
                                if let Err(e) = append_result {
                                    log::warn!("ERROR: element_append_class failed: {}", e);
                                }
                                let append_result = element_append_class(
                                    &document,
                                    &format!("slot{}", idx + 2 - 2 * (COLS as usize)),
                                    "win",
                                );
                                if let Err(e) = append_result {
                                    log::warn!("ERROR: element_append_class failed: {}", e);
                                }
                                let append_result = element_append_class(
                                    &document,
                                    &format!("slot{}", idx + 3 - 3 * (COLS as usize)),
                                    "win",
                                );
                                if let Err(e) = append_result {
                                    log::warn!("ERROR: element_append_class failed: {}", e);
                                }

                                shared.board[idx].replace(shared.board[idx].get().into_win());
                                shared.board[idx + 1 - (COLS as usize)].replace(
                                    shared.board[idx + 1 - (COLS as usize)].get().into_win(),
                                );
                                shared.board[idx + 2 - 2 * (COLS as usize)].replace(
                                    shared.board[idx + 2 - 2 * (COLS as usize)].get().into_win(),
                                );
                                shared.board[idx + 3 - 3 * (COLS as usize)].replace(
                                    shared.board[idx + 3 - 3 * (COLS as usize)].get().into_win(),
                                );
                            }
                            WinType::DiagonalDown(idx) => {
                                let placed_class_erase_result = element_remove_class(
                                    &document,
                                    &format!("slot{}", idx),
                                    "placed",
                                );
                                if let Err(e) = placed_class_erase_result {
                                    log::warn!("ERROR: element_remove_class failed: {}", e);
                                }
                                let placed_class_erase_result = element_remove_class(
                                    &document,
                                    &format!("slot{}", idx + 1 + (COLS as usize)),
                                    "placed",
                                );
                                if let Err(e) = placed_class_erase_result {
                                    log::warn!("ERROR: element_remove_class failed: {}", e);
                                }
                                let placed_class_erase_result = element_remove_class(
                                    &document,
                                    &format!("slot{}", idx + 2 + 2 * (COLS as usize)),
                                    "placed",
                                );
                                if let Err(e) = placed_class_erase_result {
                                    log::warn!("ERROR: element_remove_class failed: {}", e);
                                }
                                let placed_class_erase_result = element_remove_class(
                                    &document,
                                    &format!("slot{}", idx + 3 + 3 * (COLS as usize)),
                                    "placed",
                                );
                                if let Err(e) = placed_class_erase_result {
                                    log::warn!("ERROR: element_remove_class failed: {}", e);
                                }

                                let append_result =
                                    element_append_class(&document, &format!("slot{}", idx), "win");
                                if let Err(e) = append_result {
                                    log::warn!("ERROR: element_append_class failed: {}", e);
                                }
                                let append_result = element_append_class(
                                    &document,
                                    &format!("slot{}", idx + 1 + (COLS as usize)),
                                    "win",
                                );
                                if let Err(e) = append_result {
                                    log::warn!("ERROR: element_append_class failed: {}", e);
                                }
                                let append_result = element_append_class(
                                    &document,
                                    &format!("slot{}", idx + 2 + 2 * (COLS as usize)),
                                    "win",
                                );
                                if let Err(e) = append_result {
                                    log::warn!("ERROR: element_append_class failed: {}", e);
                                }
                                let append_result = element_append_class(
                                    &document,
                                    &format!("slot{}", idx + 3 + 3 * (COLS as usize)),
                                    "win",
                                );
                                if let Err(e) = append_result {
                                    log::warn!("ERROR: element_append_class failed: {}", e);
                                }

                                shared.board[idx].replace(shared.board[idx].get().into_win());
                                shared.board[idx + 1 + (COLS as usize)].replace(
                                    shared.board[idx + 1 + (COLS as usize)].get().into_win(),
                                );
                                shared.board[idx + 2 + 2 * (COLS as usize)].replace(
                                    shared.board[idx + 2 + 2 * (COLS as usize)].get().into_win(),
                                );
                                shared.board[idx + 3 + 3 * (COLS as usize)].replace(
                                    shared.board[idx + 3 + 3 * (COLS as usize)].get().into_win(),
                                );
                            }
                            WinType::None => unreachable!("WinType should never be None on win"),
                        }
                    }

                    let text_append_result =
                        append_to_info_text(&document, "info_text1", "<b>Game Over</b>", 1);
                    if let Err(e) = text_append_result {
                        log::warn!("ERROR: text append to info_text1 failed: {}", e);
                    }

                    shared
                        .game_state
                        .replace(GameState::PostGameResults(endgame_state));
                } // if: check for win or draw

                // check if it is AI's turn
                if let GameState::SinglePlayer(player_type, _ai_difficulty) =
                    shared.game_state.get()
                {
                    if shared.turn.get() != player_type {
                        ctx.link().send_message(WrapperMsg::AIChoice);
                    }
                }
            } // WrapperMsg::Pressed(idx) =>
            WrapperMsg::AIChoice => {
                // defer by 1 second
                self.defer_message(ctx, WrapperMsg::AIChoiceImpl, AI_CHOICE_DURATION_MILLIS);
            }
            WrapperMsg::AIChoiceImpl => {
                // get AI's choice
                if let GameState::SinglePlayer(player_type, ai_difficulty) = shared.game_state.get()
                {
                    if shared.turn.get() != player_type {
                        let choice =
                            get_ai_choice(ai_difficulty, player_type.get_opposite(), &shared.board)
                                .expect("AI should have an available choice");
                        ctx.link()
                            .send_message(WrapperMsg::AIPressed(usize::from(choice) as u8));
                    }
                }
            }
            WrapperMsg::BackendTick => {
                if self.player_id.is_none() {
                    self.get_networked_player_id(ctx);
                }

                // repeat BackendTick handling while "connected" to backend
                if self.do_backend_tick {
                    self.defer_message(ctx, WrapperMsg::BackendTick, BACKEND_TICK_DURATION_MILLIS);
                }
            }
            WrapperMsg::BackendRequest { place } => {
                self.place_request = Some(place);
            }
            WrapperMsg::BackendResponse(br_enum) => match br_enum {
                BREnum::Error(string) => {
                    log::warn!("{}", string);
                }
                BREnum::GotID(id, turn_opt) => {
                    self.player_id = Some(id);
                    log::warn!("Got player id {}", id);
                }
            },
        } // match (msg)

        true
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct InfoText {}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Properties)]
pub struct InfoTextProperties {
    id: usize,
}

impl Component for InfoText {
    type Message = ();
    type Properties = InfoTextProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let (shared, _) = ctx
            .link()
            .context::<SharedState>(Callback::noop())
            .expect("state to be set");
        match ctx.props().id {
            0 => {
                html! {
                    <div id={format!("info_text{}", ctx.props().id)} class={format!("info_text{}", ctx.props().id)}>
                        {"Hello"}
                    </div>
                }
            }
            1 => {
                if shared.game_state.get() == GameState::MainMenu {
                    html! {
                        <div id={format!("info_text{}", ctx.props().id)} class={format!("info_text{}", ctx.props().id)}>
                            <p>
                                <b>
                                    {"Waiting to choose game-mode..."}
                                </b>
                            </p>
                        </div>
                    }
                } else if shared.turn.get() == Turn::CyanPlayer {
                    html! {
                        <div id={format!("info_text{}", ctx.props().id)} class={format!("info_text{}", ctx.props().id)}>
                            <p>
                                <b class={"cyan"}>
                                    {"It is CyanPlayer's turn"}
                                </b>
                            </p>
                        </div>
                    }
                } else {
                    html! {
                        <div id={format!("info_text{}", ctx.props().id)} class={format!("info_text{}", ctx.props().id)}>
                            <p>
                                <b class={"magenta"}>
                                    {"It is MagentaPlayer's turn"}
                                </b>
                            </p>
                        </div>
                    }
                }
            }
            _ => {
                unreachable!();
            }
        }
    }
}
