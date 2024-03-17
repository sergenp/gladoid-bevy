use anyhow::Result;
use env_logger::Env;
use game_core::schedules::GladoidGameWorld;
use pyo3::{pyfunction, pymodule, types::PyModule, wrap_pyfunction, PyResult, Python};
mod game_core;

#[pyfunction]
fn create_world() -> Result<GladoidGameWorld> {
    let mut world = game_core::schedules::GladoidGameWorld::new();

    world.spawn_player("Sergen".to_string(), 10);
    world.spawn_player("Quanntum".to_string(), 10);
    Ok(world)
}

#[pymodule]
fn gladoid_bevy(_py: Python, m: &PyModule) -> PyResult<()> {
    let env = Env::default()
        .filter_or("GLADOID_LOG_LEVEL", "debug")
        .write_style_or("GLADOID_LOG_STYLE", "auto");
    env_logger::init_from_env(env);

    m.add_function(wrap_pyfunction!(create_world, m)?)?;
    Ok(())
}
