use bevy::prelude::*;

pub fn event_manager<T: Event>(mut events: ResMut<Events<T>>) {
    events.update();
}
