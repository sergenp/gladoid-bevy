use bevy_ecs::{
    entity::Entity,
    event::EventWriter,
    query::{With, Without},
    system::{Commands, Query},
    world::Mut,
};
use rand::{distributions::WeightedIndex, prelude::Distribution, thread_rng};
use std::io;

use crate::game_core::structs::IsTurn;

use super::{
    actions::ActionType,
    events::{GameEndEvent, GameMessageEvent},
    structs::{Health, IsAlive, Player, TurnProgress, TurnSpeed, Weapon},
};

pub(crate) fn race_for_turn(
    mut message_writer: EventWriter<GameMessageEvent>,
    mut commands: Commands,
    mut players: Query<(Entity, &Player, &TurnSpeed, &mut TurnProgress), With<IsAlive>>,
) {
    let mut messages: Vec<GameMessageEvent> = vec![];
    let mut entities_with_turn: Vec<(Entity, &Player, Mut<'_, TurnProgress>)> = vec![];
    let mut weights: Vec<u16> = vec![];
    for (entity, player, turn_speed, mut turn_progress) in players.iter_mut() {
        turn_progress.progress += turn_speed.speed;
        messages.push(format!("{} has {:?} speed", player.name, turn_speed).into());
        messages.push(
            format!(
                "{} has {:?} progress for a turn",
                player.name, turn_progress.progress
            )
            .into(),
        );
        if turn_progress.progress >= 100 {
            messages.push(format!("Giving turn to {}", player.name).into());
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
            messages
                .push(format!("{} selected for the turn", entity_selected_for_turn.1.name).into());
            entity_selected_for_turn.2.progress = 0;
            // for the rest of the entities, remove IsTurn, so only one entity can keep IsTurn,
            // which is the randomly selected one
            for entity_data in entities_with_turn {
                commands.entity(entity_data.0).remove::<IsTurn>();
            }
        }
    }
    message_writer.send_batch(messages);
}

pub(crate) fn attack(
    mut message_writer: EventWriter<GameMessageEvent>,
    mut commands: Commands,
    player_with_turn: Query<(Entity, &Player, &Weapon), (With<IsTurn>, With<IsAlive>)>,
    mut player_without_turn: Query<(&Player, &mut Health), (Without<IsTurn>, With<IsAlive>)>,
) {
    let (entity, player1, weapon) = match player_with_turn.get_single() {
        Ok(res) => res,
        Err(_) => return,
    };
    let (player2, mut health) = match player_without_turn.get_single_mut() {
        Ok(res) => res,
        Err(_) => return,
    };
    commands.entity(entity).remove::<IsTurn>();
    health.hp -= weapon.damage as i16;
    let messages: Vec<GameMessageEvent> = vec![
        format!(
            "{}, {}'nın kafasına {}le vurdu.",
            player1.name, player2.name, weapon.name
        )
        .into(),
        format!("{} {} hasar aldı.", player2.name, weapon.damage).into(),
    ];
    message_writer.send_batch(messages);
}

pub(crate) fn update_alive(
    mut message_writer: EventWriter<GameMessageEvent>,
    mut commands: Commands,
    players: Query<(Entity, &Player, &Health), With<IsAlive>>,
) {
    let mut messages: Vec<GameMessageEvent> = vec![];
    for (entity, player, health) in players.iter() {
        if health.hp <= 0 {
            commands.entity(entity).remove::<IsAlive>();
            messages.push(format!("{} öldü.", player.name).into());
        }
    }
    message_writer.send_batch(messages);
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
            messages
                .push(format!("{} öldü. {} kazandı.", dead_player.name, alive_player.name).into());
            game_end_writer.send(GameEndEvent);
        }
    };
    message_writer.send_batch(messages);
}

pub(crate) fn get_player_action(
    mut commands: Commands,
    player_with_turn: Query<(Entity, &Player), (With<IsTurn>, With<IsAlive>)>,
    players_without_turn: Query<&Player, (Without<IsTurn>, With<IsAlive>)>,
) {
    match player_with_turn.get_single() {
        Ok(player) => {
            println!(
                "{}",
                format!("{} select an action to perform!", player.1.name)
            );
            player
        }
        Err(_) => return,
    };

    let mut user_input = String::new();
    let stdin = io::stdin();
    stdin.read_line(&mut user_input).unwrap();
    let user_action = user_input.trim().parse::<u32>().unwrap_or(1);
    let action_type: ActionType = match user_action {
        1 => {
            println!("Select a player to attack!");
            let player = players_without_turn.get_single().unwrap();
            ActionType::Attack(player.id)
            // if players_without_turn.iter().len() == 1 {
            // } else {
            //     for (i, player) in players_without_turn.iter().enumerate(){
            //         println!("{}", format!("{} : {}", i, player.name));
            //     }
            //     let mut user_input = String::new();
            //     let stdin = io::stdin();
            //     stdin.read_line(&mut user_input).unwrap();
            //     let user_action = user_input.trim().parse::<u32>().unwrap_or(1);
            //     let action_type = match user_action {

            //     }
            // }
        }
        2 => ActionType::ChooseWeapon(1),
        3 => ActionType::Heal,
        _ => ActionType::NoAction,
    };

    println!("You Chose: {:?} ", action_type);
    // inserting the same resource will overwrite the others,
    // that means one action type can only exist per world tick,
    // perfect for a turn based game, there can only be one action per turn
    commands.insert_resource(action_type);
}
