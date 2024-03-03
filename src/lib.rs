use game_core::schedules::GameWorldState;
use pyo3::prelude::*;

mod game_core;

/// Formats the sum of two numbers as string.
#[pyfunction]
fn run() -> PyResult<()> {
    let mut world = game_core::schedules::GladoidGameWorld::new();

    world.spawn_player("Sergen".to_string(), 10);
    world.spawn_player("Quanntum".to_string(), 10);
    loop {
        world.tick();
        match world.state {
            GameWorldState::Done => break,
            _ => (),
        }
    }
    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn gladoid_bevy(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(run, m)?)?;
    Ok(())
}
