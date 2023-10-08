use bevy_ecs::{
    component::Component,
    entity::Entity,
    query::{With, Without},
    schedule::{IntoSystemConfigs, Schedule},
    system::Commands,
    system::Query,
    world::World,
};

#[derive(Component)]
struct IsTurn;
#[derive(Component)]
struct IsAlive;

#[derive(Component)]
struct Player {
    name: String,
}

#[derive(Component)]
struct Health(i32);

#[derive(Component)]
struct Weapon {
    damage: u32,
    name: String,
}

fn attack(
    player_with_turn: Query<(&Player, &Weapon, &Health), (With<IsTurn>, With<IsAlive>)>,
    mut player_without_turn: Query<(&Player, &mut Health), (Without<IsTurn>, With<IsAlive>)>,
) {
    let player_1 = player_with_turn.iter().next().unwrap();
    let mut player_2 = player_without_turn.iter_mut().next().unwrap();
    println!(
        "{}, {}'nın kafasına {}le vurdu.",
        player_1.0.name, player_2.0.name, player_1.1.name
    );
    player_2.1 .0 -= player_1.1.damage as i32;

    println!("{} {} hasar aldı.", player_2.0.name, player_1.1.damage);
}

fn update_alive(mut commands: Commands, players: Query<(Entity, &Player, &Health), With<IsAlive>>) {
    for player in players.iter() {
        if player.2 .0 <= 0 {
            commands.entity(player.0).remove::<IsAlive>();
            println!("{} öldü.", player.1.name);
        }
    }
}

fn main() {
    let mut world = World::default();
    let mut schedule = Schedule::default();

    // let bundle = PlayerBundle();
    world.spawn((
        Player {
            name: "Kaan".to_string(),
        },
        Weapon {
            damage: 30,
            name: "Kılıç".to_string(),
        },
        Health(30),
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
        Health(30),
        IsTurn,
        IsAlive,
    ));

    schedule.add_systems((attack, update_alive.after(attack)));
    schedule.run(&mut world);
}
