use bevy_ecs::system::adapter::new;

mod game_core;

fn main() {
    let mut world = game_core::schedules::GladoidGameWorld::new();

    world.spawn_player("Sergen".to_string(), 10);
    world.spawn_player("Quanntum".to_string(), 10);
    // loop {
    // }
}
