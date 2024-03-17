use bevy_ecs::event::Event;

#[derive(Event, Default)]
pub(crate) struct GameMessageEvent {
    pub(crate) message: String,
}

#[derive(Event, Default, Debug)]
pub(crate) struct GameEndEvent;

#[derive(Event, Default, Debug)]
pub(crate) struct PlayerDiedEvent;

impl From<String> for GameMessageEvent {
    fn from(value: String) -> Self {
        GameMessageEvent { message: value }
    }
}

impl From<&GameMessageEvent> for String {
    fn from(value: &GameMessageEvent) -> Self {
        value.message.clone()
    }
}
