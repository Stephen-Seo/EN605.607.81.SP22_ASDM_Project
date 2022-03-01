use std::cell::Cell;
use std::rc::Rc;
use yew::prelude::*;

//const ROWS: u8 = 8;
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
    board: [Rc<Cell<BoardState>>; 56],
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
                let value_at_idx = shared.board[idx as usize].get();
                match value_at_idx {
                    BoardState::Empty => {
                        shared.board[idx as usize].replace(BoardState::Cyan);
                    }
                    BoardState::Cyan => {
                        shared.board[idx as usize].replace(BoardState::Magenta);
                    }
                    BoardState::Magenta => {
                        shared.board[idx as usize].replace(BoardState::Empty);
                    }
                }
            }
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
            </div> // wrapper
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
    yew::start_app::<App>();
}
