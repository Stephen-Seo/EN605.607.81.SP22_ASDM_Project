use crate::constants::{COLS, INFO_TEXT_MAX_ITEMS, ROWS};
use crate::game_logic::{check_win_draw, WinType};
use crate::html_helper::{append_to_info_text, element_append_class, get_window_document};
use crate::state::{BoardState, GameState, SharedState, Turn};

use std::cell::Cell;
use std::rc::Rc;

use yew::prelude::*;

pub struct MainMenu {}

pub enum MainMenuMessage {
    SinglePlayer,
    LocalMultiplayer,
    NetworkedMultiplayer,
}

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
                let onclick_local_multiplayer =
                    ctx.link().callback(|_| MainMenuMessage::LocalMultiplayer);
                html! {
                    <div class={"menu"} id={"mainmenu"}>
                        <b class={"menuText"}>{"Please pick a game mode."}</b>
                        <button class={"menuSinglePlayer"}>
                            {"Singleplayer"}
                        </button>
                        <button class={"menuLocalMultiplayer"} onclick={onclick_local_multiplayer}>
                            {"Local Multiplayer"}
                        </button>
                        <button class={"menuMultiplayer"}>
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
            info_text_turn.set_inner_html("<p><b class=\"cyan\">It is CyanPlayer's Turn</b></p>");
        }

        true
    }
}

pub struct Slot {}

pub enum SlotMessage {
    Press(u8),
}

#[derive(Clone, PartialEq, Properties)]
pub struct SlotProperties {
    idx: u8,
    state: Rc<Cell<BoardState>>,
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
        let idx_copy = idx;
        let onclick = ctx.link().callback(move |_| SlotMessage::Press(idx_copy));
        let col = idx % COLS;
        let row = idx / COLS;
        html! {
            <button class={format!("slot {} r{} c{}", state, row, col)} id={format!("slot{}", idx)} onclick={onclick}>
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
            GameState::SinglePlayer
            | GameState::LocalMultiplayer
            | GameState::NetworkedMultiplayer => (),
            GameState::PostGameResults(_) => return false,
        }
        if shared.game_state.get() == GameState::MainMenu {
            return false;
        }

        match msg {
            SlotMessage::Press(idx) => {
                // notify Wrapper with message
                let msg = WrapperMsg::Pressed(idx);
                if let Some(p) = ctx.link().get_parent() {
                    p.clone().downcast::<Wrapper>().send_message(msg);
                }
            }
        }

        true
    }
}

pub struct Wrapper {}

pub enum WrapperMsg {
    Pressed(u8),
}

impl Component for Wrapper {
    type Message = WrapperMsg;
    type Properties = ();

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let (shared, _) = ctx
            .link()
            .context::<SharedState>(Callback::noop())
            .expect("state to be set");
        html! {
            <div class="wrapper">
                <MainMenu />
                <Slot idx=0 state={shared.board[0].clone()} />
                <Slot idx=1 state={shared.board[1].clone()} />
                <Slot idx=2 state={shared.board[2].clone()} />
                <Slot idx=3 state={shared.board[3].clone()} />
                <Slot idx=4 state={shared.board[4].clone()} />
                <Slot idx=5 state={shared.board[5].clone()} />
                <Slot idx=6 state={shared.board[6].clone()} />
                <Slot idx=7 state={shared.board[7].clone()} />
                <Slot idx=8 state={shared.board[8].clone()} />
                <Slot idx=9 state={shared.board[9].clone()} />
                <Slot idx=10 state={shared.board[10].clone()} />
                <Slot idx=11 state={shared.board[11].clone()} />
                <Slot idx=12 state={shared.board[12].clone()} />
                <Slot idx=13 state={shared.board[13].clone()} />
                <Slot idx=14 state={shared.board[14].clone()} />
                <Slot idx=15 state={shared.board[15].clone()} />
                <Slot idx=16 state={shared.board[16].clone()} />
                <Slot idx=17 state={shared.board[17].clone()} />
                <Slot idx=18 state={shared.board[18].clone()} />
                <Slot idx=19 state={shared.board[19].clone()} />
                <Slot idx=20 state={shared.board[20].clone()} />
                <Slot idx=21 state={shared.board[21].clone()} />
                <Slot idx=22 state={shared.board[22].clone()} />
                <Slot idx=23 state={shared.board[23].clone()} />
                <Slot idx=24 state={shared.board[24].clone()} />
                <Slot idx=25 state={shared.board[25].clone()} />
                <Slot idx=26 state={shared.board[26].clone()} />
                <Slot idx=27 state={shared.board[27].clone()} />
                <Slot idx=28 state={shared.board[28].clone()} />
                <Slot idx=29 state={shared.board[29].clone()} />
                <Slot idx=30 state={shared.board[30].clone()} />
                <Slot idx=31 state={shared.board[31].clone()} />
                <Slot idx=32 state={shared.board[32].clone()} />
                <Slot idx=33 state={shared.board[33].clone()} />
                <Slot idx=34 state={shared.board[34].clone()} />
                <Slot idx=35 state={shared.board[35].clone()} />
                <Slot idx=36 state={shared.board[36].clone()} />
                <Slot idx=37 state={shared.board[37].clone()} />
                <Slot idx=38 state={shared.board[38].clone()} />
                <Slot idx=39 state={shared.board[39].clone()} />
                <Slot idx=40 state={shared.board[40].clone()} />
                <Slot idx=41 state={shared.board[41].clone()} />
                <Slot idx=42 state={shared.board[42].clone()} />
                <Slot idx=43 state={shared.board[43].clone()} />
                <Slot idx=44 state={shared.board[44].clone()} />
                <Slot idx=45 state={shared.board[45].clone()} />
                <Slot idx=46 state={shared.board[46].clone()} />
                <Slot idx=47 state={shared.board[47].clone()} />
                <Slot idx=48 state={shared.board[48].clone()} />
                <Slot idx=49 state={shared.board[49].clone()} />
                <Slot idx=50 state={shared.board[50].clone()} />
                <Slot idx=51 state={shared.board[51].clone()} />
                <Slot idx=52 state={shared.board[52].clone()} />
                <Slot idx=53 state={shared.board[53].clone()} />
                <Slot idx=54 state={shared.board[54].clone()} />
                <Slot idx=55 state={shared.board[55].clone()} />
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
        let (window, document) =
            get_window_document().expect("Should be able to get Window and Document");

        match msg {
            WrapperMsg::Pressed(idx) => {
                let mut bottom_idx = idx;
                let mut placed = false;
                let current_player = shared.turn.get();

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
                            "slot {} r{} c{}",
                            current_board_state,
                            bottom_idx / COLS,
                            bottom_idx % COLS
                        ));
                    }

                    placed = true;
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

                        match win_type {
                            WinType::Horizontal(idx) => {
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
                            WinType::None => todo!(),
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
                } else {
                    // game is still ongoing

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
                        let output_str = format!(
                            "<b class=\"{}\">It is {}'s turn</b>",
                            turn.get_color(),
                            turn
                        );

                        let text_append_result =
                            append_to_info_text(&document, "info_text1", &output_str, 1);
                        if let Err(e) = text_append_result {
                            log::warn!("ERROR: text append to info_text1 failed: {}", e);
                        }
                    }
                } // else: game is still ongoing after logic check
            } // WrapperMsg::Pressed(idx) =>
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
