use chess::Color;

mod engine;
mod play;
mod measure;

fn main() {
    let mut engine_1 = engine::Engine::new(0, engine::PruningType::None);
    let mut engine_2 = engine::Engine::new(0, engine::PruningType::AlphaBeta);

    let mean_time_per_move_1 = engine_1.measure_mean_nanos_per_move(100);
    let mean_time_per_move_2 = engine_1.measure_mean_nanos_per_move(100);
    let (elo_1, elo_2) = measure::measure_elo(&mut engine_1, &mut engine_2, 100);

    let score_1 = measure::engine_score(elo_1, elo_2, mean_time_per_move_1, mean_time_per_move_2);

    println!("{} {}", elo_1, elo_2);
}