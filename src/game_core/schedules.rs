use bevy_ecs::{
    event::Events,
    schedule::{
        common_conditions::on_event,
        common_conditions::{resource_equals, resource_exists},
        Condition, IntoSystemConfigs, Schedule,
    },
    world::World,
};

use super::{
    actions::ActionType,
    events::{message_reader, GameEndEvent, GameMessageEvent, PlayerDiedEvent},
    structs::{Health, IsAlive, Player, TurnProgress, TurnSpeed, Weapon},
    systems::{attack, check_game_end, end_turn, get_player_action, race_for_turn, update_alive},
};

pub enum GameWorldState {
    Fighting,
    Done,
}

pub(crate) struct GladoidGameWorld {
    world: World,
    schedules: Vec<Schedule>,
    event_schedules: Vec<Schedule>,
    pub(crate) state: GameWorldState,
}

impl GladoidGameWorld {
    pub fn new() -> Self {
        let mut world = World::default();
        let mut turn_schedule = Schedule::default();
        let mut attack_schedule = Schedule::default();
        let mut game_end_schedule = Schedule::default();
        let mut messages_schedule = Schedule::default();
        let mut input_schedule = Schedule::default();
        let mut cleanup_schedule = Schedule::default();

        let player_died_event = Events::<PlayerDiedEvent>::default();
        let game_message_events = Events::<GameMessageEvent>::default();
        let game_end_event = Events::<GameEndEvent>::default();

        world.insert_resource(player_died_event);
        world.insert_resource(game_message_events);
        world.insert_resource(game_end_event);

        turn_schedule.add_systems(race_for_turn);
        input_schedule.add_systems(get_player_action);
        attack_schedule.add_systems((attack, update_alive.after(attack)).run_if(
            // only run if there is an ActionType that matches ActionType::Attack variant
            // giving 1 to Attack variant will match every "Attack", it is here to satisfy resource_equals
            resource_exists::<ActionType>().and_then(resource_equals(ActionType::Attack(1))),
        ));
        messages_schedule.add_systems(message_reader.run_if(on_event::<GameMessageEvent>()));
        game_end_schedule.add_systems(check_game_end.run_if(on_event::<PlayerDiedEvent>()));
        cleanup_schedule.add_systems(end_turn);

        let mut schedules = Vec::new();
        let mut event_schedules = Vec::new();
        schedules.push(turn_schedule);
        schedules.push(input_schedule);
        schedules.push(attack_schedule);
        schedules.push(cleanup_schedule);
        event_schedules.push(game_end_schedule);
        event_schedules.push(messages_schedule);
        Self {
            world,
            schedules,
            event_schedules,
            state: GameWorldState::Fighting,
        }
    }

    pub fn tick(&mut self) {
        match self.state {
            GameWorldState::Done => return,
            GameWorldState::Fighting => (),
        }

        println!("Ticking...");
        // TODO: make the checking for game_end better, somehow
        let game_end_event = self.world.get_resource::<Events<GameEndEvent>>().unwrap();
        let game_end_event_reader = game_end_event.get_reader();
        if game_end_event_reader.len(&game_end_event) > 0 {
            println!("Game has ended, can't progress further.");
            self.state = GameWorldState::Done;
            return;
        }

        for schedule in self.schedules.iter_mut() {
            schedule.run(&mut self.world);
            // need to process events per schedule, to avoid frame delays
            for event_schedule in self.event_schedules.iter_mut() {
                event_schedule.run(&mut self.world);
            }
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
