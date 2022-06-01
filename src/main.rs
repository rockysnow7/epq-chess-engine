use chess::Color;
use std::time::{Instant, Duration};

mod engine;
mod play;

fn main() {
    let mut engine_unpruned = engine::Engine::new(0, engine::PruningType::None);
    let mut engine_ab_pruned = engine::Engine::new(0, engine::PruningType::AlphaBeta);
    //play::play_game(Color::White, &mut engine);
    /*for _ in 0..100 {
        engine.train(100, false);
    }*/

    let start = Instant::now();
    engine_unpruned.train(10, false);
    let elapsed = start.elapsed();
    println!("Unpruned: {}s {}ms\n", elapsed.as_secs(), elapsed.as_millis());

    let start = Instant::now();
    engine_ab_pruned.train(10, false);
    let elapsed = start.elapsed();
    println!("AB pruned: {}s {}ms", elapsed.as_secs(), elapsed.as_millis());
}