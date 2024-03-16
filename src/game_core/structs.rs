use bevy_ecs::component::Component;

#[derive(Debug, Component)]
pub struct TurnProgress {
    pub progress: u16,
}

#[derive(Debug, Component)]
pub struct TurnSpeed {
    pub speed: u16,
}

#[derive(Component)]
pub struct IsTurn;

#[derive(Component)]
pub struct IsAlive;

#[derive(Component, Clone)]
pub struct Player {
    pub id: u32,
    pub name: String,
}

#[derive(Component)]
pub struct Health {
    pub hp: i16,
}

#[derive(Component, Debug)]
pub struct Weapon {
    pub damage: u16,
    pub name: String,
}
