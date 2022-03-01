use std::cell::{Cell, RefCell};
use std::collections::VecDeque;
use std::rc::Rc;
use yew::prelude::*;

//const ROWS: u8 = 8;
const COLS: u8 = 7;

const INFO_TEXT_MAX_ITEMS: u32 = 100;
const INFO_TEXT_HEIGHT: i32 = 400;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum BoardState {
    Empty,
    Cyan,
    Magenta,
}

impl Default for BoardState {
    fn default() -> Self {
        Self::Empty
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct MessageBus {
    queued: VecDeque<String>,
}

impl Default for MessageBus {
    fn default() -> Self {
        Self {
            queued: VecDeque::new(),
        }
    }
}

impl MessageBus {
    pub fn get_next_msg(&mut self) -> Option<String> {
        self.queued.pop_front()
    }

    pub fn push_msg(&mut self, msg: String) -> Result<(), String> {
        self.queued.push_back(msg);
        Ok(())
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
enum Turn {
    CyanPlayer,
    MagentaPlayer,
}

#[derive(Clone, Debug, PartialEq)]
struct SharedState {
    board: [Rc<Cell<BoardState>>; 56],
    bus: Rc<RefCell<MessageBus>>,
    turn: Turn,
    info_text_ref: NodeRef,
}

impl Default for SharedState {
    fn default() -> Self {
        Self {
            // cannot use [<type>; 56] because Rc does not impl Copy
            board: [
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
                Rc::new(Cell::new(BoardState::default())),
            ],
            bus: Rc::new(RefCell::new(MessageBus::default())),
            turn: Turn::CyanPlayer,
            info_text_ref: NodeRef::default(),
        }
    }
}

struct Slot {}

enum SlotMessage {
    Press(u8),
}

#[derive(Clone, PartialEq, Properties)]
struct SlotProperties {
    idx: u8,
    state: Rc<Cell<BoardState>>,
    bus: Rc<RefCell<MessageBus>>,
}

impl Component for Slot {
    type Message = SlotMessage;
    type Properties = SlotProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let idx = ctx.props().idx;
        let state = match ctx.props().state.as_ref().get() {
            BoardState::Empty => "open",
            BoardState::Cyan => "cyan",
            BoardState::Magenta => "magenta",
        };
        let idx_copy = idx;
        let onclick = ctx.link().callback(move |_| SlotMessage::Press(idx_copy));
        let col = idx % COLS;
        let row = idx / COLS;
        html! {
            <button class={format!("slot {} r{} c{}", state, row, col)} id={format!("{}", idx)} onclick={onclick}>
            </button>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            SlotMessage::Press(idx) => {
                let (shared, _) = ctx
                    .link()
                    .context::<SharedState>(Callback::noop())
                    .expect("shared to be set");

                let result = shared.bus.borrow_mut().push_msg(format!("pressed {idx}"));
                if let Err(e) = result {
                    log::error!("Error pushing msg to bus: {}", e);
                } else {
                    // DEBUG
                    //log::info!("Pushed \"pressed {idx}\" msg to bus");
                }
            }
        }

        // notify Wrapper with message
        if let Some(p) = ctx.link().get_parent() {
            p.clone().downcast::<Wrapper>().send_message(());
        }

        true
    }
}

struct Wrapper {}

impl Component for Wrapper {
    type Message = ();
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
                <Slot idx=0 state={shared.board[0].clone()} bus={shared.bus.clone()} />
                <Slot idx=1 state={shared.board[1].clone()} bus={shared.bus.clone()} />
                <Slot idx=2 state={shared.board[2].clone()} bus={shared.bus.clone()} />
                <Slot idx=3 state={shared.board[3].clone()} bus={shared.bus.clone()} />
                <Slot idx=4 state={shared.board[4].clone()} bus={shared.bus.clone()} />
                <Slot idx=5 state={shared.board[5].clone()} bus={shared.bus.clone()} />
                <Slot idx=6 state={shared.board[6].clone()} bus={shared.bus.clone()} />
                <Slot idx=7 state={shared.board[7].clone()} bus={shared.bus.clone()} />
                <Slot idx=8 state={shared.board[8].clone()} bus={shared.bus.clone()} />
                <Slot idx=9 state={shared.board[9].clone()} bus={shared.bus.clone()} />
                <Slot idx=10 state={shared.board[10].clone()} bus={shared.bus.clone()} />
                <Slot idx=11 state={shared.board[11].clone()} bus={shared.bus.clone()} />
                <Slot idx=12 state={shared.board[12].clone()} bus={shared.bus.clone()} />
                <Slot idx=13 state={shared.board[13].clone()} bus={shared.bus.clone()} />
                <Slot idx=14 state={shared.board[14].clone()} bus={shared.bus.clone()} />
                <Slot idx=15 state={shared.board[15].clone()} bus={shared.bus.clone()} />
                <Slot idx=16 state={shared.board[16].clone()} bus={shared.bus.clone()} />
                <Slot idx=17 state={shared.board[17].clone()} bus={shared.bus.clone()} />
                <Slot idx=18 state={shared.board[18].clone()} bus={shared.bus.clone()} />
                <Slot idx=19 state={shared.board[19].clone()} bus={shared.bus.clone()} />
                <Slot idx=20 state={shared.board[20].clone()} bus={shared.bus.clone()} />
                <Slot idx=21 state={shared.board[21].clone()} bus={shared.bus.clone()} />
                <Slot idx=22 state={shared.board[22].clone()} bus={shared.bus.clone()} />
                <Slot idx=23 state={shared.board[23].clone()} bus={shared.bus.clone()} />
                <Slot idx=24 state={shared.board[24].clone()} bus={shared.bus.clone()} />
                <Slot idx=25 state={shared.board[25].clone()} bus={shared.bus.clone()} />
                <Slot idx=26 state={shared.board[26].clone()} bus={shared.bus.clone()} />
                <Slot idx=27 state={shared.board[27].clone()} bus={shared.bus.clone()} />
                <Slot idx=28 state={shared.board[28].clone()} bus={shared.bus.clone()} />
                <Slot idx=29 state={shared.board[29].clone()} bus={shared.bus.clone()} />
                <Slot idx=30 state={shared.board[30].clone()} bus={shared.bus.clone()} />
                <Slot idx=31 state={shared.board[31].clone()} bus={shared.bus.clone()} />
                <Slot idx=32 state={shared.board[32].clone()} bus={shared.bus.clone()} />
                <Slot idx=33 state={shared.board[33].clone()} bus={shared.bus.clone()} />
                <Slot idx=34 state={shared.board[34].clone()} bus={shared.bus.clone()} />
                <Slot idx=35 state={shared.board[35].clone()} bus={shared.bus.clone()} />
                <Slot idx=36 state={shared.board[36].clone()} bus={shared.bus.clone()} />
                <Slot idx=37 state={shared.board[37].clone()} bus={shared.bus.clone()} />
                <Slot idx=38 state={shared.board[38].clone()} bus={shared.bus.clone()} />
                <Slot idx=39 state={shared.board[39].clone()} bus={shared.bus.clone()} />
                <Slot idx=40 state={shared.board[40].clone()} bus={shared.bus.clone()} />
                <Slot idx=41 state={shared.board[41].clone()} bus={shared.bus.clone()} />
                <Slot idx=42 state={shared.board[42].clone()} bus={shared.bus.clone()} />
                <Slot idx=43 state={shared.board[43].clone()} bus={shared.bus.clone()} />
                <Slot idx=44 state={shared.board[44].clone()} bus={shared.bus.clone()} />
                <Slot idx=45 state={shared.board[45].clone()} bus={shared.bus.clone()} />
                <Slot idx=46 state={shared.board[46].clone()} bus={shared.bus.clone()} />
                <Slot idx=47 state={shared.board[47].clone()} bus={shared.bus.clone()} />
                <Slot idx=48 state={shared.board[48].clone()} bus={shared.bus.clone()} />
                <Slot idx=49 state={shared.board[49].clone()} bus={shared.bus.clone()} />
                <Slot idx=50 state={shared.board[50].clone()} bus={shared.bus.clone()} />
                <Slot idx=51 state={shared.board[51].clone()} bus={shared.bus.clone()} />
                <Slot idx=52 state={shared.board[52].clone()} bus={shared.bus.clone()} />
                <Slot idx=53 state={shared.board[53].clone()} bus={shared.bus.clone()} />
                <Slot idx=54 state={shared.board[54].clone()} bus={shared.bus.clone()} />
                <Slot idx=55 state={shared.board[55].clone()} bus={shared.bus.clone()} />
                <div class="info_text_wrapper">
                    <InfoText />
                </div>
            </div> // wrapper
        }
    }

    fn rendered(&mut self, ctx: &Context<Self>, _first_render: bool) {
        let (shared, _) = ctx
            .link()
            .context::<SharedState>(Callback::noop())
            .expect("state to be set");

        loop {
            if let Some(msg) = shared.bus.borrow_mut().get_next_msg() {
                let split_str: Vec<&str> = msg.split_whitespace().collect();
                if split_str.len() == 2 {
                    if split_str[0] == "pressed" {
                        if let Ok(idx) = split_str[1].parse::<u8>() {
                            let output_str: String = format!("Got {idx} pressed.");
                            // DEBUG
                            //log::info!("{}", &output_str);
                            if let Some(info_text) =
                                shared.info_text_ref.cast::<web_sys::HtmlParagraphElement>()
                            {
                                // create the new text to be appended in the output
                                let window = web_sys::window().expect("no window exists");
                                let document =
                                    window.document().expect("window should have a document");
                                let p = document
                                    .create_element("p")
                                    .expect("document should be able to create <p>");
                                p.set_text_content(Some(&output_str));

                                // check if scrolled to bottom
                                let at_bottom: bool = info_text.scroll_top() + INFO_TEXT_HEIGHT
                                    >= info_text.scroll_height();

                                // append text to output
                                info_text
                                    .append_with_node_1(&p)
                                    .expect("should be able to append to info_text");
                                while info_text.child_element_count() > INFO_TEXT_MAX_ITEMS {
                                    info_text
                                        .remove_child(&info_text.first_child().unwrap())
                                        .expect("should be able to limit items in info_text");
                                }

                                // scroll to bottom only if at bottom
                                if at_bottom {
                                    info_text.set_scroll_top(info_text.scroll_height());
                                }
                            } else {
                                log::warn!("Failed to get \"info_text\"");
                            }
                        }
                    }
                }
            } else {
                break;
            }
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        true
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
struct InfoText {}

impl Component for InfoText {
    type Message = ();
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
            <div ref={shared.info_text_ref} class="info_text">
                {"Hello"}
            </div>
        }
    }
}

#[function_component(App)]
fn app() -> Html {
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
