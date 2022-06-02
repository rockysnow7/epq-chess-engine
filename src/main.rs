use chess::Color;

mod engine;
mod play;

fn main() {
    let mut engine_1 = engine::Engine::new(0, engine::PruningType::None);
    let mut engine_2 = engine::Engine::new(0, engine::PruningType::None);

    _ = play::play_engines(&mut engine_1, &mut engine_2, true);
}