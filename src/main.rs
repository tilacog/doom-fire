extern crate ggez;
extern crate rand;

use ggez::{conf, event, graphics, Context, ContextBuilder, GameResult};
use rand::{thread_rng, Rng};

const HEIGHT: usize = 600;
const WIDTH: usize = 800;
const SCALE: usize = 5;
const ROWS: usize = (HEIGHT / SCALE);
const COLS: usize = (WIDTH / SCALE);
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
    // heat source
    let src_index = {
        /* heat can go sideways, so we accept the following ranges:
        - y: [-1, 0]
        - x: [-1, +1] (must check boundaries)
        */
        let source_x = {
            let modifier: i8 = thread_rng().gen_range(-1, 2);
            match modifier {
                -1 => {
                    if target_x == 0 {
                        COLS - 1
                    } else {
                        target_x - 1
                    }
                }
                0 => target_x,
                1 => {
                    if target_x == COLS - 1 {
                        0
                    } else {
                        target_x + 1
                    }
                }
                _ => unreachable!(),
            }
        };
        let source_y = target_y - thread_rng().gen_range(0, 2);

        let source_fire_pixel = &fire_grid[source_y][source_x];
        source_fire_pixel.index
    };

    // fire pixel visited by this iteration
    let mut target_fire_pixel = &mut fire_grid[target_y][target_x];
    let decay: usize = thread_rng().gen_range(0, 2);
    target_fire_pixel.index = match src_index.checked_sub(decay) {
        Some(new_index) => new_index,
        None => 0,
    }
}

impl event::EventHandler for State {
    fn update(&mut self, _ctx: &mut Context) -> GameResult<()> {
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
                    y: ((ROWS - y_pos) * SCALE) as f32, // render upside down
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
        .window_mode(conf::WindowMode::default().dimensions(WIDTH as u32, HEIGHT as u32));
    let mut ctx = cb.build()?;
    let mut state = State::new();
    event::run(&mut ctx, &mut state)
}
