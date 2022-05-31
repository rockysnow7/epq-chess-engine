use chess::Color;
//use time::Instant;

mod engine;
mod play;

fn main() {
    let mut engine = engine::Engine::new(0, false);
    //play::play_game(Color::White, &mut engine);
    for _ in 0..100 {
        engine.train(100, false);
    }
}