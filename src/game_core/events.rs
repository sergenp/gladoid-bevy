use bevy_ecs::event::{Event, EventReader};

#[derive(Event, Default)]
pub(crate) struct GameMessageEvent {
    pub(crate) message: String,
}

impl From<String> for GameMessageEvent {
    fn from(value: String) -> Self {
        GameMessageEvent { message: value }
    }
}

pub(crate) fn message_reader(mut reader: EventReader<GameMessageEvent>) {
    for event in reader.iter() {
        println!("{}", event.message);
    }
}
