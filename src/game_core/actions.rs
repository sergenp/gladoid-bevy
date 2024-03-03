use bevy_ecs::system::Resource;
use std::fmt::Debug;
use std::mem::discriminant;

#[derive(Debug, Resource)]
pub(crate) enum ActionType {
    Attack(u32),
    ChooseWeapon(u32),
    Heal,
    NoAction,
}

// got from: https://stackoverflow.com/a/63466959
impl PartialEq for ActionType {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}
