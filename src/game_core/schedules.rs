use anyhow::bail;
use anyhow::Context;
use anyhow::Result;
use bevy_ecs::{
    event::Events,
    schedule::{
        common_conditions::on_event,
        common_conditions::{resource_equals, resource_exists},
        Condition, IntoSystemConfigs, Schedule,
    },
    world::World,
};
use pyo3::prelude::*;
use pyo3::types::PyDict;

use super::{
    actions::{ActionType, NeedAction},
    events::{GameEndEvent, GameMessageEvent, PlayerDiedEvent},
    structs::{Health, IsAlive, Player, TurnProgress, TurnSpeed, Weapon},
    systems::{attack, check_game_end, end_turn, insert_need_action, race_for_turn, update_alive},
};

#[derive(PartialEq)]
#[pyclass]
pub enum GameWorldState {
    NeedAction,
    Fighting,
    Done,
}

#[pyclass]
pub(crate) struct GladoidGameWorld {
    world: World,
    schedules: Vec<Schedule>,
    event_schedules: Vec<Schedule>,
    id_counter: u32,
    pub state: GameWorldState,
}

#[pymethods]
impl GladoidGameWorld {
    #[new]
    pub fn new() -> Self {
        let mut world = World::default();
        let mut turn_schedule = Schedule::default();
        let mut attack_schedule = Schedule::default();
        let mut game_end_schedule = Schedule::default();
        let mut input_schedule = Schedule::default();

        let player_died_event = Events::<PlayerDiedEvent>::default();
        let game_message_events = Events::<GameMessageEvent>::default();
        let game_end_event = Events::<GameEndEvent>::default();

        world.insert_resource(player_died_event);
        world.insert_resource(game_message_events);
        world.insert_resource(game_end_event);

        turn_schedule.add_systems(race_for_turn);
        input_schedule.add_systems(insert_need_action);
        attack_schedule.add_systems(
            (attack, update_alive.after(attack), end_turn.after(attack)).run_if(
                // only run if there is an ActionType that matches ActionType::Attack variant
                // giving 1 to Attack variant will match every "Attack", it is here to satisfy resource_equals
                resource_exists::<ActionType>.and_then(resource_equals(ActionType::Attack(1))),
            ),
        );
        game_end_schedule.add_systems(check_game_end.run_if(on_event::<PlayerDiedEvent>()));

        let mut event_schedules = Vec::new();
        event_schedules.push(game_end_schedule);

        let mut schedules = Vec::new();
        schedules.push(attack_schedule);
        schedules.push(turn_schedule);
        schedules.push(input_schedule);
        Self {
            world,
            schedules,
            event_schedules,
            id_counter: 0,
            state: GameWorldState::Fighting,
        }
    }

    pub fn check_need_action(&mut self) -> bool {
        self.world.get_resource::<NeedAction>().is_some()
    }

    pub fn check_need_action_from(&mut self) -> Option<Player> {
        match self.world.get_resource::<NeedAction>() {
            Some(need_action_from) => Some(need_action_from.player.clone()),
            None => None,
        }
    }

    pub fn is_done(&mut self) -> bool {
        match self.state {
            GameWorldState::Done => return true,
            _ => return false,
        }
    }

    fn set_state(&mut self) {
        if self.check_need_action() {
            self.state = GameWorldState::NeedAction
        } else if self.check_game_ended() {
            self.state = GameWorldState::Done
        } else {
            self.state = GameWorldState::Fighting
        }
    }

    fn check_game_ended(&mut self) -> bool {
        let game_end_event = self.world.get_resource::<Events<GameEndEvent>>().unwrap();
        let game_end_event_reader = game_end_event.get_reader();
        if game_end_event_reader.len(&game_end_event) > 0 {
            return true;
        }
        return false;
    }

    #[pyo3(signature = (**kwargs))]
    pub fn insert_action(&mut self, kwargs: Option<&PyDict>) -> Result<()> {
        if self.state != GameWorldState::NeedAction {
            bail!("No need for an action...")
        }

        let kwargs = match kwargs {
            Some(kwargs) => kwargs,
            None => bail!("No kwargs has been passed!"),
        };

        let action_id: u32 = kwargs
            .get_item("action_id")?
            .context("No `action_id kwargs has been passed!")?
            .extract()?;
        let action = match action_id {
            1 => {
                let player_id_kw: u32 = kwargs
                    .get_item("player_id")?
                    .context("Action with `action_id` 1 needs to also have `player_id`!")?
                    .extract()?;
                ActionType::Attack(player_id_kw)
            }
            2 => {
                let weapon_id: u32 = kwargs
                    .get_item("player_id")?
                    .context("Action with `action_id` 2 needs to also have `weapon_id`!")?
                    .extract()?;
                ActionType::ChooseWeapon(weapon_id)
            }
            3 => ActionType::Heal,
            _ => ActionType::NoAction,
        };
        self.world.insert_resource(action);
        self.world
            .remove_resource::<NeedAction>()
            .context("Couldn't remove NeedAction...")?;
        Ok(())
    }

    pub fn tick(&mut self) -> Result<()> {
        log::debug!("Ticking...");
        for schedule in self.schedules.iter_mut() {
            schedule.run(&mut self.world);
            // need to process events per schedule, to avoid frame delays
            for event_schedule in self.event_schedules.iter_mut() {
                event_schedule.run(&mut self.world);
            }
        }
        self.set_state();
        if self.is_done() {
            bail!("Game is done, cannot continue...")
        }
        self.world.clear_trackers();
        Ok(())
    }

    pub fn spawn_player(&mut self, name: String, hp: i16) {
        self.id_counter += 1;
        self.world.spawn((
            Player {
                id: self.id_counter,
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

    pub fn get_game_messages(&mut self) -> Vec<String> {
        let mut message_event = self
            .world
            .get_resource_mut::<Events<GameMessageEvent>>()
            .unwrap();

        let messages: Vec<String> = message_event
            .get_reader()
            .read(&message_event)
            .map(|event| event.into())
            .collect();
        // clear old events
        message_event.clear();
        messages
    }
}
