use ggez;
use ggez::conf::WindowSetup;
use ggez::event;
use ggez::graphics;
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};
use std::env;
use std::path;

struct Enemy {
    hp: u32,
}

struct MainState {
    frames: usize,
    font: graphics::Font,
    enemies: Vec<Enemy>,
    player: Player,
}

enum Card {
    Forward_1,
}

impl Card {
    fn draw(&self, font: graphics::Font, ctx: &mut Context) -> GameResult {
        match self {
            Forward_1 => {
                let text = graphics::Text::new(("フォワード", font, 36.0));
                graphics::draw(ctx, &text, (Point2::new(100.0, 0.0),))
            }
        }
    }
}

struct Player {
    hp: u32,
    hand: Vec<Card>,
}

impl Player {
    fn new() ->  Player {
        Player {
            hp: 100,
            hand: vec![Card::Forward_1,Card::Forward_1,Card::Forward_1,Card::Forward_1,Card::Forward_1],
        }
    }

    fn draw(&self, font: graphics::Font, ctx: &mut Context) -> GameResult {
        self.draw_hp(font, ctx)?;
        self.draw_hand(font, ctx)?;
        Ok(())
    }

    fn draw_hand(&self, font: graphics::Font, ctx: &mut Context) -> GameResult {
        let text = graphics::Text::new((format!("HP:{}", self.hp), font, 36.0));
        graphics::draw(ctx, &text, (Point2::new(0.0, 0.0),))?;
        Ok(())
    }

    fn draw_hp(&self, font: graphics::Font, ctx: &mut Context) -> GameResult {
        let text = graphics::Text::new((format!("HP:{}", self.hp), font, 36.0));
        graphics::draw(ctx, &text, (Point2::new(0.0, 0.0),))?;
        for card in self.hand.iter() {
            card.draw(font, ctx)?;
        }
        Ok(())
    }
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        Ok(MainState {
            frames: 0,
            font: graphics::Font::new(ctx, "/NuAnkoMochiFwCt-Reg.ttf")?,
            enemies: vec![],
            player: Player::new(),
        })
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.1, 0.2, 0.3, 1.0].into());

        let offset = self.frames as f32 / 2.0;
        let dest_point = Point2::new(offset, offset);
        let text = graphics::Text::new(("hoge!", self.font, 48.0));
        graphics::draw(ctx, &text, (dest_point,))?;
        self.player.draw(self.font, ctx)?;
        graphics::present(ctx)?;

        self.frames += 1;

        Ok(())
    }
}

pub fn main() -> GameResult {
    let resource_dir = if let Ok(manifest_dir) = env::var("CARGO_MANIFEST_DIR") {
        let mut path = path::PathBuf::from(manifest_dir);
        path.push("resources");
        path
    } else {
        path::PathBuf::from("./resources")
    };

    let cb = ggez::ContextBuilder::new("vajdra", "Satoshi Amemiya")
        .window_setup(WindowSetup::default().title("ヴァジュドラ"))
        .add_resource_path(resource_dir);

    let (ctx, events_loop) = &mut cb.build()?;

    println!("{}", graphics::renderer_info(ctx)?);

    let state = &mut MainState::new(ctx)?;
    event::run(ctx, events_loop, state)
}

