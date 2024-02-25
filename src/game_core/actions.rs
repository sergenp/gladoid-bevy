use super::structs::{Player, Weapon};

enum ActionType {
    Attack(Player),
    ChooseWeapon(Weapon),
    Heal,
}
