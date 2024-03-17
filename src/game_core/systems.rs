use bevy_ecs::{
    entity::Entity,
    event::EventWriter,
    query::{With, Without},
    system::{Commands, Query, Res},
    world::Mut,
};
use rand::{distributions::WeightedIndex, prelude::Distribution, thread_rng};

use crate::game_core::{actions::NeedAction, structs::IsTurn};

use super::{
    actions::ActionType,
    events::{GameEndEvent, GameMessageEvent, PlayerDiedEvent},
    structs::{Health, IsAlive, Player, TurnProgress, TurnSpeed, Weapon},
};

pub(crate) fn race_for_turn(
    mut commands: Commands,
    mut players: Query<(Entity, &Player, &TurnSpeed, &mut TurnProgress), With<IsAlive>>,
) {
    let mut entities_with_turn: Vec<(Entity, &Player, Mut<'_, TurnProgress>)> = vec![];
    let mut weights: Vec<u16> = vec![];
    for (entity, player, turn_speed, mut turn_progress) in players.iter_mut() {
        turn_progress.progress += turn_speed.speed;
        log::debug!("{} has {:?}", player.name, turn_speed);
        log::debug!(
            "{} has {:?} progress for a turn",
            player.name,
            turn_progress.progress
        );
        if turn_progress.progress >= 100 {
            log::debug!("Giving turn to {}", player.name);
            commands.entity(entity).insert(IsTurn);
            weights.push(turn_progress.progress);
            entities_with_turn.push((entity, player, turn_progress));
        }
    }
    match entities_with_turn.len() {
        // if no one has a turn, just return
        0 => return,
        // set the entity with the turn's progress to 0,
        // so that it will have 0 speed when racing for turn the next time
        // other entities won't have their speed 0 when racing for next time,
        // this way we make sure whomever gets the turn, won't get it the next time,
        1 => {
            entities_with_turn[0].2.progress = 0;
        }
        // if there are more than 2 players who got the turn,
        // randomly select one of the players to have the turn
        // it will be random but will be weighted, so if someone has say,
        // 120 and someone has 100 progress, 120 will be favored, but not get the turn 100%
        _ => {
            let dist = WeightedIndex::new(&weights).unwrap();
            let mut rng = thread_rng();
            // remove the selected entity from the array so the entity keeps its turn
            let mut entity_selected_for_turn = entities_with_turn.remove(dist.sample(&mut rng));
            log::debug!("{} selected for the turn", entity_selected_for_turn.1.name);
            entity_selected_for_turn.2.progress = 0;
            // for the rest of the entities, remove IsTurn, so only one entity can keep IsTurn,
            // which is the randomly selected one
            for entity_data in entities_with_turn {
                commands.entity(entity_data.0).remove::<IsTurn>();
            }
        }
    }
}

pub(crate) fn attack(
    mut message_writer: EventWriter<GameMessageEvent>,
    attack_action: Res<ActionType>,
    player_with_turn: Query<(&Player, &Weapon), (With<IsTurn>, With<IsAlive>)>,
    mut players_without_turn: Query<(&Player, &mut Health), (Without<IsTurn>, With<IsAlive>)>,
) {
    let (player1, weapon) = match player_with_turn.get_single() {
        Ok(res) => res,
        Err(_) => {
            log::debug!("No one has the turn...");
            return;
        }
    };

    let attacked_player_id = match attack_action.as_ref() {
        ActionType::Attack(player_id) => player_id.to_owned(),
        _ => {
            log::debug!(
                "ActionType is not Attack yet somehow we're running the attack schedule..."
            );
            return;
        }
    };

    let mut attacked_entity: Option<(&Player, Mut<'_, Health>)> = None;
    for entity_without_turn in players_without_turn.iter_mut() {
        if entity_without_turn.0.id == attacked_player_id {
            attacked_entity = Some(entity_without_turn);
            break;
        }
    }

    let (player2, mut health) = match attacked_entity {
        Some(entity) => entity,
        // should never reach here
        None => {
            message_writer.send(
                format!(
                    "{} Attacked someone that does not exist... Somehow.",
                    player1.name
                )
                .into(),
            );
            return;
        }
    };
    health.hp -= weapon.damage as i16;
    let messages: Vec<GameMessageEvent> = vec![
        format!(
            "{} hit {} in the head with a {}.",
            player1.name, player2.name, weapon.name
        )
        .into(),
        format!(
            "{} took {} damage. {} has {} hp left",
            player2.name, weapon.damage, player2.name, health.hp
        )
        .into(),
    ];
    message_writer.send_batch(messages);
}

pub(crate) fn end_turn(
    mut message_writer: EventWriter<GameMessageEvent>,
    mut commands: Commands,
    players_with_turn: Query<(Entity, &Player), (With<IsTurn>, With<IsAlive>)>,
) {
    let mut messages: Vec<GameMessageEvent> = Vec::new();
    for (entity, player) in players_with_turn.iter() {
        commands.entity(entity).remove::<IsTurn>();
        messages.push(format!("Removing turn from {}...", player.name).into())
    }
    message_writer.send_batch(messages);
    commands.remove_resource::<ActionType>();
    commands.remove_resource::<NeedAction>();
}

pub(crate) fn update_alive(
    mut commands: Commands,
    mut player_died_writer: EventWriter<PlayerDiedEvent>,
    players: Query<(Entity, &Health), With<IsAlive>>,
) {
    for (entity, health) in players.iter() {
        if health.hp <= 0 {
            commands.entity(entity).remove::<IsAlive>();
            player_died_writer.send_default();
        }
    }
}

pub(crate) fn check_game_end(
    mut message_writer: EventWriter<GameMessageEvent>,
    mut game_end_writer: EventWriter<GameEndEvent>,
    dead_player: Query<&Player, Without<IsAlive>>,
    alive_player: Query<&Player, With<IsAlive>>,
) {
    let mut messages: Vec<GameMessageEvent> = vec![];

    if let Ok(dead_player) = dead_player.get_single() {
        if let Ok(alive_player) = alive_player.get_single() {
            messages.push(format!("{} died. {} won.", dead_player.name, alive_player.name).into());
            message_writer.send_batch(messages);
            game_end_writer.send_default();
        }
    };
}

pub(crate) fn insert_need_action(
    mut commands: Commands,
    player_with_turn: Query<(Entity, &Player), (With<IsTurn>, With<IsAlive>)>,
) {
    let player: Player = match player_with_turn.get_single() {
        Ok(entity) => entity.1.clone(),
        Err(_) => return,
    };

    commands.insert_resource(NeedAction { player });
}
