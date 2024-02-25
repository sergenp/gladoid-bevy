use bevy_ecs::{
    event::Events,
    schedule::{IntoSystemConfigs, Schedule},
    world::World,
};

use super::{
    events::{message_reader, GameEndEvent, GameMessageEvent},
    structs::{Health, IsAlive, Player, TurnProgress, TurnSpeed, Weapon},
    systems::{
        attack, check_game_end, get_player_action, race_for_turn, select_who_has_turn, update_alive,
    },
};
pub(crate) fn spawn_player(world: &mut World, name: String, hp: i16) {
    world.spawn((
        Player {
            id: 1,
            name: name.to_string(),
        },
        Weapon {
            damage: 3,
            name: "Kılıç".to_string(),
        },
        Health { hp },
        TurnSpeed { speed: 50 },
        TurnProgress { progress: 0 },
        IsAlive,
    ));
}

pub(crate) struct GladoidGameWorld {
    world: World,
    schedules: Vec<Schedule>,
}

impl GladoidGameWorld {
    pub fn new() -> Self {
        let mut world = World::default();
        let mut turn_schedule = Schedule::default();
        let mut attack_schedule = Schedule::default();
        let mut game_end_schedule = Schedule::default();
        let mut messages_schedule = Schedule::default();
        let mut input_schedule = Schedule::default();

        let game_message_events = Events::<GameMessageEvent>::default();
        let game_end_event = Events::<GameEndEvent>::default();

        world.insert_resource(game_message_events);
        world.insert_resource(game_end_event);

        turn_schedule.add_systems((race_for_turn, select_who_has_turn.after(race_for_turn)));
        input_schedule.add_systems(get_player_action);
        attack_schedule.add_systems((attack, update_alive.after(attack)));
        game_end_schedule.add_systems(check_game_end);
        messages_schedule.add_systems(message_reader);

        let mut schedules = Vec::new();
        schedules.push(turn_schedule);
        schedules.push(input_schedule);
        schedules.push(attack_schedule);
        schedules.push(game_end_schedule);
        schedules.push(messages_schedule);
        Self {
            world,
            schedules: schedules,
        }
    }

    pub fn tick(&mut self) {
        println!("Ticking...");
        let game_end_event = self.world.get_resource::<Events<GameEndEvent>>().unwrap();
        let game_end_event_reader = game_end_event.get_reader();
        if game_end_event_reader.len(&game_end_event) > 0 {
            println!("Game has ended, can't progress further.");
            return;
        }

        for schedule in self.schedules.iter_mut() {
            schedule.run(&mut self.world);
        }
        self.world.clear_trackers();
    }

    pub(crate) fn spawn_player(&mut self, name: String, hp: i16) {
        self.world.spawn((
            Player {
                id: 1,
                name: name.to_string(),
            },
            Weapon {
                damage: 3,
                name: "Kılıç".to_string(),
            },
            Health { hp },
            TurnSpeed { speed: 50 },
            TurnProgress { progress: 0 },
            IsAlive,
        ));
    }
}
