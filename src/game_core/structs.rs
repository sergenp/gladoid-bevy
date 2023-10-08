use bevy_ecs::component::Component;

#[derive(Debug, Component)]
pub(crate) struct TurnSpeed {
    pub speed: u16,
    pub progress: u16,
}

#[derive(Component)]
pub(crate) struct IsTurn;

#[derive(Component)]
pub(crate) struct IsAlive;

#[derive(Component)]
pub(crate) struct Player {
    pub name: String,
}

#[derive(Component)]
pub(crate) struct Health(pub(crate) i16);

#[derive(Component)]
pub(crate) struct Weapon {
    pub damage: u16,
    pub name: String,
}
