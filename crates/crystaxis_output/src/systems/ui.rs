use bevy::prelude::*;

#[derive(Component)]
pub struct FpsText;

#[derive(Component)]
pub struct ParticleCountText;

#[derive(Component)]
pub struct VisibilityBudgetText;

/// Создаёт UI-элементы: счётчики FPS, количества частиц и лимита видимости
pub fn setup_ui(mut commands: Commands) {
    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(10.0),
                left: Val::Px(10.0),
                ..default()
            },
            GlobalZIndex(1000),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("FPS: --"),
                TextFont { font_size: 20.0, ..default() },
                TextColor(Color::WHITE),
                FpsText,
            ));
        });

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(35.0),
                left: Val::Px(10.0),
                ..default()
            },
            GlobalZIndex(1000),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Particles: --"),
                TextFont { font_size: 20.0, ..default() },
                TextColor(Color::WHITE),
                ParticleCountText,
            ));
        });

    commands
        .spawn((
            Node {
                position_type: PositionType::Absolute,
                top: Val::Px(60.0),
                left: Val::Px(10.0),
                ..default()
            },
            GlobalZIndex(1000),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Sprites: --"),
                TextFont { font_size: 20.0, ..default() },
                TextColor(Color::WHITE),
                VisibilityBudgetText,
            ));
        });
}