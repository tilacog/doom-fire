extern crate ggez;

use ggez::{conf, event, graphics, Context, ContextBuilder, GameResult};

const HEIGHT: u32 = 600;
const WIDTH: u32 = 800;
const SCALE: usize = 5;
const ROWS: usize = (HEIGHT as usize / SCALE);
const COLS: usize = (WIDTH as usize / SCALE);
const MAX_COLOR: usize = 35;
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

impl Into<graphics::Color> for &RGB {
    fn into(self) -> graphics::Color {
        let RGB(r, g, b) = self;
        graphics::Color::from_rgb(*r, *g, *b)
    }
}

struct FirePixel {
    index: usize,
}

impl FirePixel {
    fn new() -> FirePixel {
        FirePixel { index: 0 }
    }
}

type FireGrid = Vec<Vec<FirePixel>>;

struct State {
    fire_grid: FireGrid,
}

impl State {
    /// Initialize a new state with a fire grid where all `FirePixel`s are black (index == 0), except for the first row,
    /// where all `FirePixels` are at full heat (index = MAX_COLOR).
    fn new() -> State {
        let mut rows: FireGrid = Vec::with_capacity(ROWS as usize);
        for row_idx in 0..ROWS {
            let mut row: Vec<FirePixel> = Vec::with_capacity(COLS as usize);
            for _ in 0..COLS {
                let mut fire_pixel = FirePixel::new();
                if row_idx == 0 {
                    fire_pixel.index = MAX_COLOR
                }
                row.push(fire_pixel)
            }
            rows.push(row);
        }
        State { fire_grid: rows }
    }
}

fn spread_fire(target_y: usize, target_x: usize, fire_grid: &mut FireGrid) {
    let src_index = {
        let source_fire_pixel = &fire_grid[target_y - 1][target_x];
        source_fire_pixel.index
    };
    let mut target_fire_pixel = &mut fire_grid[target_y][target_x];
    if src_index > 0 {
        target_fire_pixel.index = src_index - 1
    }
}

impl event::EventHandler for State {
    fn update(&mut self, ctx: &mut Context) -> GameResult<()> {
        for y_pos in 1..ROWS {
            for x_pos in 0..COLS {
                spread_fire(y_pos, x_pos, &mut self.fire_grid)
            }
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult<()> {
        graphics::clear(ctx);
        graphics::set_background_color(ctx, graphics::BLACK);

        for y_pos in 0..ROWS {
            for x_pos in 0..COLS {
                let fire_pixel = &self.fire_grid[y_pos][x_pos];
                let color = &COLORS[fire_pixel.index];
                graphics::set_color(ctx, color.into())?;
                let rect = graphics::Rect {
                    x: (x_pos * SCALE) as f32,
                    y: (y_pos * SCALE) as f32,
                    w: SCALE as f32,
                    h: SCALE as f32,
                };
                graphics::rectangle(ctx, graphics::DrawMode::Fill, rect)?
            }
        }
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
