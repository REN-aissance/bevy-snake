use bevy::prelude::*;

pub struct MovementPlugin;
impl Plugin for MovementPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<MovementEvent>()
            .add_systems(FixedUpdate, update_position);
    }
}

#[derive(Bundle, Default, Debug)]
pub struct MovementBundle {
    velocity: Velocity,
}

#[derive(Event)]
pub struct MovementEvent(pub Entity);

#[derive(Component, Default, Copy, Clone, Debug)]
pub struct Velocity(pub Vec3);

fn update_position(
    mut q: Query<(Entity, &mut Transform, &Velocity)>,
    mut er: EventWriter<MovementEvent>,
) {
    let mut movement_events = vec![];
    for (e, mut transform, v) in q.iter_mut() {
        transform.translation += v.0;
        movement_events.push(MovementEvent(e));
    }
    if !movement_events.is_empty() {
        er.send_batch(movement_events);
    }
}
