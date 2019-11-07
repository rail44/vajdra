use ggez;
use ggez::conf::WindowSetup;
use ggez::event;
use ggez::graphics;
use ggez::graphics::DrawParam;
use ggez::nalgebra::Point2;
use ggez::{Context, GameResult};
use std::convert::Into;
use std::env;
use std::path;

use rand::thread_rng;
use rand::seq::SliceRandom;

struct Enemy {
    hp: u32,
}

struct MainState {
    frames: usize,
    font: graphics::Font,
    enemies: Vec<Enemy>,
    player: Player,
}

fn get_default_deck() -> Vec<Card> {
    use Card::*;

    vec![
        Forward1,
        Forward1,
        Forward1,
        Forward1,
        Forward1,
        Forward1,
        Forward1,
        Forward1,
        Forward1,
        Forward1,
        Headbutt,
        Headbutt,
        Headbutt,
        Headbutt,
        Headbutt,
        Headbutt,
        Headbutt,
        Bodyattack,
        Bodyattack,
        Bodyattack,
        Concentrate,
        Concentrate,
        Sleep,
        Sleep,
        Breath,
        Breath,
        Charge,
        Charge,
    ]
}

#[derive(Clone, Debug)]
enum Card {
    Forward1,
    Headbutt,
    Bodyattack,
    Concentrate,
    Sleep,
    Breath,
    Charge,
}

impl Card {
    fn draw<T>(&self, font: graphics::Font, ctx: &mut Context, params: T) -> GameResult
    where
        T: Into<DrawParam>,
    {
        use Card::*;
        match self {
            Forward1 => {
                let text = graphics::Text::new(("フォワード", font, 12.0));
                graphics::draw(ctx, &text, params)
            }
            Headbutt => {
                let text = graphics::Text::new(("ずつき", font, 12.0));
                graphics::draw(ctx, &text, params)
            }
            Bodyattack => {
                let text = graphics::Text::new(("たいあたり", font, 12.0));
                graphics::draw(ctx, &text, params)
            }
            Concentrate => {
                let text = graphics::Text::new(("コンセントレート", font, 12.0));
                graphics::draw(ctx, &text, params)
            }
            Sleep => {
                let text = graphics::Text::new(("ねむる", font, 12.0));
                graphics::draw(ctx, &text, params)
            }
            Breath => {
                let text = graphics::Text::new(("ひのいき", font, 12.0));
                graphics::draw(ctx, &text, params)
            }
            Charge => {
                let text = graphics::Text::new(("ためる", font, 12.0));
                graphics::draw(ctx, &text, params)
            }
        }
    }
}

struct Player {
    hp: u32,
    hand: Vec<Card>,
    deck: Vec<Card>,
}

impl Player {
    fn new() -> Player {
        let mut deck = get_default_deck();
        deck.shuffle(&mut thread_rng());
        let (hand, deck) = deck.split_at_mut(5);
        Player {
            hp: 100,
            deck: deck.to_vec(),
            hand: hand.to_vec(),
        }
    }

    fn draw(&self, font: graphics::Font, ctx: &mut Context) -> GameResult {
        self.draw_hp(font, ctx)?;
        self.draw_hand(font, ctx)?;

        let params = DrawParam::default().dest(Point2::new(0.0, 100.0));

        let mut offset = 0.0;
        for card in self.hand.iter() {
            let params_for_each_card = params.dest(Point2::new(params.dest.x + offset, params.dest.y));
            offset += 120.0;
            card.draw(font, ctx, params_for_each_card)?;
        }
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
