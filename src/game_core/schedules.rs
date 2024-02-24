use bevy_ecs::{
    event::Events,
    schedule::{IntoSystemConfigs, Schedule},
    world::World,
};

use super::{
    events::{message_reader, GameMessageEvent},
    structs::{Health, IsAlive, Player, TurnProgress, TurnSpeed, Weapon},
    systems::{attack, check_game_end, race_for_turn, select_who_has_turn, update_alive},
};

pub(crate) fn run() {
    let mut world = World::default();
    let mut turn_schedule = Schedule::default();
    let mut attack_schedule = Schedule::default();
    let mut game_end_schedule = Schedule::default();
    let mut messages_schedule = Schedule::default();

    let game_message_events = Events::<GameMessageEvent>::default();

    world.insert_resource(game_message_events);

    // let bundle = PlayerBundle();
    world.spawn((
        Player {
            id: 1,
            name: "Kaan".to_string(),
        },
        Weapon {
            damage: 30,
            name: "Kılıç".to_string(),
        },
        Health { hp: 30 },
        TurnSpeed { speed: 50 },
        TurnProgress { progress: 0 },
        IsAlive,
    ));
    world.spawn((
        Player {
            id: 2,
            name: "E da".to_string(),
        },
        Weapon {
            damage: 300,
            name: "Tüfek".to_string(),
        },
        Health { hp: 3000 },
        TurnSpeed { speed: 50 },
        TurnProgress { progress: 0 },
        IsAlive,
    ));

    turn_schedule.add_systems((race_for_turn, select_who_has_turn.after(race_for_turn)));
    attack_schedule.add_systems((attack, update_alive.after(attack)));
    game_end_schedule.add_systems(check_game_end);
    messages_schedule.add_systems(message_reader);
    // loop {
    turn_schedule.run(&mut world);
    turn_schedule.run(&mut world);
    attack_schedule.run(&mut world);
    game_end_schedule.run(&mut world);
    messages_schedule.run(&mut world);
    world.clear_trackers();
    // }
}
