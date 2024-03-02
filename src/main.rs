use game_core::schedules::GameWorldState;

mod game_core;

fn main() {
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
}
