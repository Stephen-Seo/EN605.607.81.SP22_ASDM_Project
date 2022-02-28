use std::cell::{Cell, RefCell};
use std::rc::Rc;
use yew::prelude::*;

const ROWS: u8 = 8;
const COLS: u8 = 7;

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
struct SharedState {
    board: Rc<RefCell<[BoardState; 56]>>,
}

impl Default for SharedState {
    fn default() -> Self {
        Self {
            board: Rc::new(RefCell::new([BoardState::default(); 56])),
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
        let idx_copy = idx.clone();
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
                let (mut shared, _) = ctx
                    .link()
                    .context::<SharedState>(Callback::noop())
                    .expect("shared to be set");
                let value_at_idx = shared.board.as_ref().borrow()[idx as usize].clone();
                match value_at_idx {
                    BoardState::Empty => {
                        shared.board.as_ref().borrow_mut()[idx as usize] = BoardState::Cyan
                    }
                    BoardState::Cyan => {
                        shared.board.as_ref().borrow_mut()[idx as usize] = BoardState::Magenta
                    }
                    BoardState::Magenta => {
                        shared.board.as_ref().borrow_mut()[idx as usize] = BoardState::Empty
                    }
                }
                ctx.props()
                    .state
                    .as_ref()
                    .replace(shared.board.as_ref().borrow_mut()[idx as usize]);
            }
        }
        true
    }
}

struct Wrapper {}

impl Component for Wrapper {
    type Message = ();
    type Properties = ();

    fn create(ctx: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let (mut shared, _) = ctx
            .link()
            .context::<SharedState>(Callback::noop())
            .expect("state to be set");
        let link = ctx.link();
        html! {
            <div class="wrapper">
                <Slot idx=0 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[0]))} />
                <Slot idx=1 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[1]))} />
                <Slot idx=2 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[2]))} />
                <Slot idx=3 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[3]))} />
                <Slot idx=4 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[4]))} />
                <Slot idx=5 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[5]))} />
                <Slot idx=6 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[6]))} />
                <Slot idx=7 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[7]))} />
                <Slot idx=8 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[8]))} />
                <Slot idx=9 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[9]))} />
                <Slot idx=10 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[10]))} />
                <Slot idx=11 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[11]))} />
                <Slot idx=12 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[12]))} />
                <Slot idx=13 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[13]))} />
                <Slot idx=14 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[14]))} />
                <Slot idx=15 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[15]))} />
                <Slot idx=16 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[16]))} />
                <Slot idx=17 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[17]))} />
                <Slot idx=18 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[18]))} />
                <Slot idx=19 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[19]))} />
                <Slot idx=20 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[20]))} />
                <Slot idx=21 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[21]))} />
                <Slot idx=22 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[22]))} />
                <Slot idx=23 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[23]))} />
                <Slot idx=24 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[24]))} />
                <Slot idx=25 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[25]))} />
                <Slot idx=26 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[26]))} />
                <Slot idx=27 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[27]))} />
                <Slot idx=28 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[28]))} />
                <Slot idx=29 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[29]))} />
                <Slot idx=30 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[30]))} />
                <Slot idx=31 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[31]))} />
                <Slot idx=32 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[32]))} />
                <Slot idx=33 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[33]))} />
                <Slot idx=34 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[34]))} />
                <Slot idx=35 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[35]))} />
                <Slot idx=36 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[36]))} />
                <Slot idx=37 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[37]))} />
                <Slot idx=38 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[38]))} />
                <Slot idx=39 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[39]))} />
                <Slot idx=40 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[40]))} />
                <Slot idx=41 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[41]))} />
                <Slot idx=42 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[42]))} />
                <Slot idx=43 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[43]))} />
                <Slot idx=44 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[44]))} />
                <Slot idx=45 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[45]))} />
                <Slot idx=46 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[46]))} />
                <Slot idx=47 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[47]))} />
                <Slot idx=48 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[48]))} />
                <Slot idx=49 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[49]))} />
                <Slot idx=50 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[50]))} />
                <Slot idx=51 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[51]))} />
                <Slot idx=52 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[52]))} />
                <Slot idx=53 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[53]))} />
                <Slot idx=54 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[54]))} />
                <Slot idx=55 state={Rc::new(Cell::new(shared.board.as_ref().borrow()[55]))} />
            </div> // wrapper
        }
    }
}
#[function_component(App)]
fn app() -> Html {
    let ctx = use_state(|| SharedState::default());
    html! {
        <ContextProvider<SharedState> context={(*ctx).clone()}>
            <Wrapper />
        </ContextProvider<SharedState>>
    }
}

fn main() {
    yew::start_app::<App>();
}
