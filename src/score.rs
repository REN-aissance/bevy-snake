use bevy::prelude::*;

use crate::snek::{DeathEvent, SnekHead};

pub struct ScorePlugin;
impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(FixedUpdate, draw_score);
    }
}

fn draw_score(mut er: EventReader<DeathEvent>, mut commands: Commands, snek: Query<&SnekHead>) {
    if er.read().next().is_some() {
        let text_style = TextStyle {
            font_size: 300.0,
            color: Color::SEA_GREEN.with_a(0.5),
            ..Default::default()
        };
        commands.spawn(Text2dBundle {
            text: Text::from_section((snek.single().children.len() + 1).to_string(), text_style),
            ..default()
        });
    }
}
