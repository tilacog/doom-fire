extern crate ggez;

use ggez::{conf, event, graphics, Context, ContextBuilder, GameResult};

const HEIGHT: u32 = 600;
const WIDTH: u32 = 800;
const SCALE: u32 = 20;
const ROWS: u32 = HEIGHT / SCALE;
const COLS: u32 = WIDTH / SCALE;
const COLORS: [RGB; 36] = [
    RGB(7, 7, 7),
    RGB(31, 7, 7),
    RGB(47, 15, 7),
    RGB(71, 15, 7),
    RGB(87, 23, 7),
    RGB(103, 31, 7),
    RGB(119, 31, 7),
    RGB(143, 39, 7),
    RGB(159, 47, 7),
    RGB(175, 63, 7),
    RGB(191, 71, 7),
    RGB(199, 71, 7),
    RGB(223, 79, 7),
    RGB(223, 87, 7),
    RGB(223, 87, 7),
    RGB(215, 95, 7),
    RGB(215, 103, 15),
    RGB(207, 111, 15),
    RGB(207, 119, 15),
    RGB(207, 127, 15),
    RGB(207, 135, 23),
    RGB(199, 135, 23),
    RGB(199, 143, 23),
    RGB(199, 151, 31),
    RGB(191, 159, 31),
    RGB(191, 159, 31),
    RGB(191, 167, 39),
    RGB(191, 167, 39),
    RGB(191, 175, 47),
    RGB(183, 175, 47),
    RGB(183, 183, 47),
    RGB(183, 183, 55),
    RGB(207, 207, 111),
    RGB(223, 223, 159),
    RGB(239, 239, 199),
    RGB(255, 255, 255),
];

struct RGB(u8, u8, u8);

struct FirePixel {
    index: u8,
}

impl FirePixel {
    fn new() -> FirePixel {
        FirePixel { index: 0 }
    }
}

struct State {
    fire_grid: Vec<Vec<FirePixel>>,
}

impl State {
    /// Initialize a new state with a 0-indexed fire grid
    fn new() -> State {
        let mut rows: Vec<Vec<FirePixel>> = Vec::with_capacity(ROWS as usize);
        for _ in 0..ROWS {
            let mut row: Vec<FirePixel> = Vec::with_capacity(COLS as usize);
            for _ in 0..COLS {
                row.push(FirePixel::new())
            }
            rows.push(row);
        }
        State { fire_grid: rows }
    }
}

impl event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::set_background_color(ctx, graphics::BLACK);
        graphics::set_color(ctx, graphics::WHITE)?;
        graphics::rectangle(
            ctx,
            graphics::DrawMode::Fill,
            graphics::Rect {
                x: (WIDTH / 2) as f32,
                y: (HEIGHT / 2) as f32,
                w: SCALE as f32,
                h: SCALE as f32,
            },
        )?;
        graphics::present(ctx);
        Ok(())
    }
}

pub fn main() -> GameResult<()> {
    let cb = ContextBuilder::new("doom-fire", "tilacog")
        .window_setup(conf::WindowSetup::default().title("doom-fire"))
        .window_mode(conf::WindowMode::default().dimensions(WIDTH, HEIGHT));
    let mut ctx = cb.build()?;
    let mut state = State::new();
    event::run(&mut ctx, &mut state)
}
