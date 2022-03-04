use crate::constants::{COLS, INFO_TEXT_MAX_ITEMS, ROWS};
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

#[allow(clippy::from_over_into)]
impl Into<GameState> for MainMenuMessage {
    fn into(self) -> GameState {
        match self {
            MainMenuMessage::SinglePlayer => GameState::SinglePlayer,
            MainMenuMessage::LocalMultiplayer => GameState::LocalMultiplayer,
            MainMenuMessage::NetworkedMultiplayer => GameState::NetworkedMultiplayer,
        }
    }
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
        let window = web_sys::window().expect("no window exists");
        let document = window.document().expect("window should have a document");

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

                // DEBUG
                //log::info!("{} is {:?}", idx, shared.board[idx as usize].get());

                // DEBUG
                //log::info!("{}", &output_str);

                // info text below the grid
                if let Some(info_text) = document.get_element_by_id("info_text0") {
                    let height = info_text.client_height();

                    // create the new text to be appended in the output
                    let p = document
                        .create_element("p")
                        .expect("document should be able to create <p>");
                    let output_str = match placed {
                        true => format!("{} placed into slot {}", current_player, bottom_idx),
                        false => "Invalid place to insert".into(),
                    };
                    p.set_text_content(Some(&output_str));

                    // DEBUG
                    //log::info!(
                    //    "pre: scroll top is {}, scroll height is {}",
                    //    info_text.scroll_top(),
                    //    info_text.scroll_height()
                    //);

                    // check if scrolled to top
                    let at_top: bool = info_text.scroll_top() <= height - info_text.scroll_height();

                    // append text to output
                    info_text
                        .append_with_node_1(&p)
                        .expect("should be able to append to info_text");
                    while info_text.child_element_count() > INFO_TEXT_MAX_ITEMS {
                        info_text
                            .remove_child(&info_text.first_child().unwrap())
                            .expect("should be able to limit items in info_text");
                    }

                    // DEBUG
                    //log::info!("at_top is {}", if at_top { "true" } else { "false" });

                    // scroll to top only if at top
                    if at_top {
                        info_text.set_scroll_top(height - info_text.scroll_height());
                    }

                    // DEBUG
                    //log::info!(
                    //    "post: scroll top is {}, scroll height is {}",
                    //    info_text.scroll_top(),
                    //    info_text.scroll_height()
                    //);
                } else {
                    log::warn!("Failed to get bottom \"info_text\"");
                }

                // info text right of the grid
                if let Some(info_text) = document.get_element_by_id("info_text1") {
                    let height = info_text.client_height();

                    // create the new text to be appended in the output
                    let p = document
                        .create_element("p")
                        .expect("document should be able to create <p>");
                    let turn = shared.turn.get();
                    p.set_inner_html(&format!(
                        "<b class=\"{}\">It is {}'s turn</b>",
                        turn.get_color(),
                        turn
                    ));

                    // check if scrolled to top
                    let at_top: bool = info_text.scroll_top() <= height - info_text.scroll_height();

                    // append text to output
                    info_text
                        .append_with_node_1(&p)
                        .expect("should be able to append to info_text");
                    while info_text.child_element_count() > 1 {
                        info_text
                            .remove_child(&info_text.first_child().unwrap())
                            .expect("should be able to limit items in info_text");
                    }

                    // scroll to top only if at top
                    if at_top {
                        info_text.set_scroll_top(height - info_text.scroll_height());
                    }
                } else {
                    log::warn!("Failed to get side \"info_text\"");
                }
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
