use crate::Angles;
use bevy::prelude::*;
use bevy::text::BreakLineOn;

#[derive(Component)]
pub struct AnglesUI;

pub fn add_ui(mut commands: Commands) {
    let text_style = TextStyle {
        color: Color::WHITE,
        font_size: 40.,
        ..default()
    };

    commands.spawn((
        AnglesUI,
        TextBundle {
            style: Style {
                position_type: PositionType::Absolute,
                top: Val::Px(0.),
                right: Val::Px(0.),
                ..default()
            },
            text: Text {
                sections: vec![TextSection::new("", text_style)],
                justify: JustifyText::Right,
                linebreak_behavior: BreakLineOn::NoWrap,
            },
            ..default()
        },
    ));
}

pub fn update_ui(angles: Res<Angles>, mut ui: Query<&mut Text, With<AnglesUI>>) {
    ui.single_mut().sections[0].value = format!(
        "Elev: {:.2}°\nLong: {:.2}°",
        angles.y / std::f32::consts::PI * 180.,
        angles.x / std::f32::consts::PI * 180.
    );
}
