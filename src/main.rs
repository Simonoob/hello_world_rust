use ggez::event;
use ggez::graphics::{self, Color, Rect};
use ggez::mint::Vector2;
use ggez::{input::keyboard, input::keyboard::KeyCode, Context, GameResult};

struct MainState {
    pos_x: f32,
    pos_y: f32,
}

impl MainState {
    fn new() -> GameResult<MainState> {
        let s = MainState {
            pos_x: 20.0,
            pos_y: 5.0,
        };
        Ok(s)
    }
}

impl event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        let window_dimensions = ggez::graphics::drawable_size(&_ctx);

        //move on X
        if keyboard::is_key_pressed(_ctx, KeyCode::Right) && self.pos_x < window_dimensions.0 - 10.0
        {
            self.pos_x += 10.0;
        } else if keyboard::is_key_pressed(_ctx, KeyCode::Left) && self.pos_x > 0.0 {
            self.pos_x -= 10.0;
        }

        //move on Y
        if keyboard::is_key_pressed(_ctx, KeyCode::Up) {
            self.pos_y -= 5.0;
            if self.pos_y < 0.0 {
                self.pos_y = 0.0
            }
        } else if self.pos_y < window_dimensions.1 - 10.0 {
            self.pos_y += 10.0;
        }

        if self.pos_y > window_dimensions.1 - 10.0 {
            self.pos_y = window_dimensions.1 - 10.0;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let circle = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            Rect {
                x: 0.0,
                y: 0.0,
                w: 10.0,
                h: 10.0,
            },
            Color::WHITE,
        )?;
        graphics::draw(
            ctx,
            &circle,
            (Vector2 {
                x: self.pos_x,
                y: self.pos_y,
            },),
        )?;

        graphics::present(ctx)?;
        Ok(())
    }
}

pub fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("super_simple", "ggez");
    let (ctx, event_loop) = cb.build()?;
    let state = MainState::new()?;
    event::run(ctx, event_loop, state)
}
