use chess::Color;

mod engine;
mod play;

fn main() {
    let mut engine_unpruned = engine::Engine::new(0, engine::PruningType::None);
    let mut engine_ab_pruned = engine::Engine::new(0, engine::PruningType::AlphaBeta);
    //play::play_game(Color::White, &mut engine);
    /*for _ in 0..100 {
        engine.train(100, false);
    }*/

    let time_unpruned = engine_unpruned.measure_mean_nanos_per_move(1000);
    println!("Unpruned: {}", time_unpruned);

    let time_ab_pruned = engine_ab_pruned.measure_mean_nanos_per_move(1000);
    println!("AB pruned: {}", time_ab_pruned);
}