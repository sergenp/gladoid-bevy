use anyhow::Result;
use env_logger::Env;
use game_core::schedules::GameWorldState;
mod game_core;

fn main() -> Result<()> {
    let env = Env::default()
        .filter_or("GLADOID_LOG_LEVEL", "info")
        .write_style_or("GLADOID_LOG_STYLE", "auto");
    env_logger::init_from_env(env);

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
