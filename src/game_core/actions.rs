use bevy_ecs::system::Resource;
use std::fmt::Debug;
use std::mem::discriminant;

use super::structs::Player;

// this is the action we get from the player
// this resource will be injected into the world,
// and world won't continue untill it is injected and consumed
#[derive(Debug, Resource)]
pub(crate) enum ActionType {
    Attack(u32),
    ChooseWeapon(u32),
    Heal,
    NoAction,
}

// got from: https://stackoverflow.com/a/63466959
// makes Attack(1) == Attack(2), ChooseWeapon(5) == ChooseWeapon(99) etc.
impl PartialEq for ActionType {
    fn eq(&self, other: &Self) -> bool {
        discriminant(self) == discriminant(other)
    }
}

#[derive(Resource)]
// this will let us know that we're in need of an action from `player`
pub(crate) struct NeedAction {
    pub(crate) player: Player,
}
