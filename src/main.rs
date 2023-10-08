use bevy_ecs::{
    component::Component,
    entity::Entity,
    query::{With, Without},
    schedule::{IntoSystemConfigs, Schedule},
    system::Commands,
    system::Query,
    world::{Mut, World},
};
use rand::{distributions::WeightedIndex, prelude::Distribution, thread_rng};

#[derive(Debug, Component)]
struct TurnSpeed {
    speed: u16,
    progress: u16,
}

#[derive(Component)]
struct IsTurn;

#[derive(Component)]
struct IsAlive;

#[derive(Component)]
struct Player {
    name: String,
}

#[derive(Component)]
struct Health(i16);

#[derive(Component)]
struct Weapon {
    damage: u16,
    name: String,
}

fn race_for_turn(mut players: Query<(&Player, &mut TurnSpeed), With<IsAlive>>) {
    for (player, mut turnspeed) in players.iter_mut() {
        println!("{} has {:?} speed", player.name, turnspeed);
        turnspeed.progress += turnspeed.speed;
        println!(
            "{} has {:?} progress for a turn",
            player.name, turnspeed.progress
        );
    }
}

fn select_who_has_turn(
    mut commands: Commands,
    mut players_with_turns: Query<(Entity, &Player, &mut TurnSpeed), With<IsAlive>>,
) {
    let mut entities_with_turn: Vec<(Entity, &Player, Mut<'_, TurnSpeed>)> = vec![];
    let mut weights: Vec<u16> = vec![];

    for (entity, player, turnspeed) in players_with_turns.iter_mut() {
        if turnspeed.progress >= 100 {
            println!("Giving turn to {}", player.name);
            commands.entity(entity).insert(IsTurn);
            weights.push(turnspeed.progress.clone());
            entities_with_turn.push((entity, player, turnspeed));
        }
    }
    match entities_with_turn.len() {
        // if no one has a turn, just return
        0 => return,
        // set the entity with the turn's progress to 0,
        // so that it will have 0 speed when racing for turn
        1 => {
            entities_with_turn[0].2.progress = 0;
            return;
        }
        _ => {}
    }

    // randomly select one of the players to have the turn
    // it will be random but will be weighted, so if someone has say,
    // 120 and someone has 100 progress, 120 will be favored, but not get the turn 100%
    let dist = WeightedIndex::new(&weights).unwrap();
    let mut rng = thread_rng();
    // remove the selected entity from the array so the entity keeps its turn
    let mut entity_selected_for_turn = entities_with_turn.remove(dist.sample(&mut rng));
    println!("{} selected for the turn", entity_selected_for_turn.1.name);
    entity_selected_for_turn.2.progress = 0;
    // for the rest of the entities, remove IsTurn, so only one entity can keep IsTurn,
    // which is the randomly selected one
    for entity_data in entities_with_turn {
        commands.entity(entity_data.0).remove::<IsTurn>();
    }
}

fn attack(
    player_with_turn: Query<(&Player, &Weapon), (With<IsTurn>, With<IsAlive>)>,
    mut player_without_turn: Query<(&Player, &mut Health), (Without<IsTurn>, With<IsAlive>)>,
) {
    let (player1, weapon) = match player_with_turn.get_single() {
        Ok(res) => res,
        Err(_) => return,
    };
    let (player2, mut health) = match player_without_turn.get_single_mut() {
        Ok(res) => res,
        Err(_) => return,
    };

    println!(
        "{}, {}'nın kafasına {}le vurdu.",
        player1.name, player2.name, weapon.name
    );
    health.0 -= weapon.damage as i16;

    println!("{} {} hasar aldı.", player2.name, weapon.damage);
}

fn update_alive(mut commands: Commands, players: Query<(Entity, &Player, &Health), With<IsAlive>>) {
    for (entity, player, health) in players.iter() {
        if health.0 <= 0 {
            commands.entity(entity).remove::<IsAlive>();
            println!("{} öldü.", player.name);
        }
    }
}

fn check_game_end(
    dead_player: Query<&Player, Without<IsAlive>>,
    alive_player: Query<&Player, With<IsAlive>>,
) {
    if let Ok(dead_player) = dead_player.get_single() {
        if let Ok(alive_player) = alive_player.get_single() {
            println!("{} öldü. {} kazandı.", dead_player.name, alive_player.name);
        }
    };
}

fn main() {
    let mut world = World::default();
    let mut turn_schedule = Schedule::default();
    let mut attack_schedule = Schedule::default();

    // let bundle = PlayerBundle();
    world.spawn((
        Player {
            name: "Kaan".to_string(),
        },
        Weapon {
            damage: 30,
            name: "Kılıç".to_string(),
        },
        Health(3000),
        TurnSpeed {
            progress: 0,
            speed: 50,
        },
        IsAlive,
    ));
    world.spawn((
        Player {
            name: "E da".to_string(),
        },
        Weapon {
            damage: 300,
            name: "Tüfek".to_string(),
        },
        Health(3000),
        TurnSpeed {
            progress: 0,
            speed: 30,
        },
        IsAlive,
    ));

    turn_schedule.add_systems((race_for_turn, select_who_has_turn.after(race_for_turn)));
    attack_schedule.add_systems((
        attack,
        update_alive.after(attack),
        check_game_end.after(update_alive),
    ));
    loop {
        turn_schedule.run(&mut world);
        attack_schedule.run(&mut world);
    }
}
