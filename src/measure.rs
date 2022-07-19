use crate::{
    engine::Engine,
    play::play_engines
};
use indicatif::ProgressBar;

/// Plays two engines against each other for `num_games` games and returns the
/// Elo score of each engine.
pub fn measure_elo(engine_1: &mut Engine, engine_2: &mut Engine, num_games: u32) -> (u16, u16) {
    let mut rating_1 = 1500f32;
    let mut rating_2 = 1500f32;

    let pb = ProgressBar::new(num_games as u64);
    for i in 0..num_games {
        let expected_1 = 1.0 / (1.0 + 10f32.powf((rating_2 - rating_1) as f32 / 400.0));
        let expected_2 = 1.0 / (1.0 + 10f32.powf((rating_1 - rating_2) as f32 / 400.0));

        if i % 2 == 0 {
            let result = play_engines(engine_1, engine_2, false);
            if result == 1 {
                rating_1 += 32.0 * (1.0 - expected_1);
                rating_2 += 32.0 * (0.0 - expected_2);
            } else if result == -1 {
                rating_1 += 32.0 * (0.0 - expected_1);
                rating_2 += 32.0 * (1.0 - expected_2);
            } else {
                rating_1 += 32.0 * (0.5 - expected_1);
                rating_2 += 32.0 * (0.5 - expected_2);
            }
        } else {
            let result = play_engines(engine_2, engine_1, false);
            if result == 1 {
                rating_2 += 32.0 * (1.0 - expected_2);
                rating_1 += 32.0 * (0.0 - expected_1);
            } else if result == -1 {
                rating_2 += 32.0 * (0.0 - expected_2);
                rating_1 += 32.0 * (1.0 - expected_1);
            } else {
                rating_2 += 32.0 * (0.5 - expected_2);
                rating_1 += 32.0 * (0.5 - expected_1);
            }
        }

        pb.inc(1);
    }
    pb.finish();

    (rating_1 as u16, rating_2 as u16)
}

/// Returns the score of an engine relative to a standard engine.
pub fn engine_score(elo_standard: f32, elo: f32, mean_time_per_move_standard: f32, mean_time_per_move: f32) -> f32 {
    (elo / elo_standard) * (mean_time_per_move_standard / mean_time_per_move)
}