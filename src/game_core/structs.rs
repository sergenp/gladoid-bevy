use bevy_ecs::component::Component;
use pyo3::pyclass;

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
#[pyclass]
pub struct Player {
    #[pyo3(get)]
    pub id: u32,
    #[pyo3(get)]
    pub name: String,
}

#[derive(Debug, Component)]
pub struct Health {
    pub hp: i16,
}

#[derive(Component, Debug)]
pub struct Weapon {
    pub damage: u16,
    pub name: String,
}
