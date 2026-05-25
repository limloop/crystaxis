use bevy::prelude::*;
use crystaxis_core::{CommandQueue, SimCommand};

/// Обрабатывает нажатия клавиш и добавляет соответствующие команды в очередь.
pub fn keyboard_input(
    keys: Res<ButtonInput<KeyCode>>,
    mut queue: ResMut<CommandQueue>,
) {
    if keys.just_pressed(KeyCode::KeyQ) {
        queue.queue.push(SimCommand::SetParticleCount(0));
    }
    if keys.just_pressed(KeyCode::KeyW) {
        queue.queue.push(SimCommand::SetParticleCount(10000));
    }
    if keys.just_pressed(KeyCode::KeyE) {
        queue.queue.push(SimCommand::AdjustParticleCount(-100));
    }
    if keys.just_pressed(KeyCode::KeyR) {
        queue.queue.push(SimCommand::AdjustParticleCount(100));
    }
    if keys.just_pressed(KeyCode::KeyT) {
        queue.queue.push(SimCommand::AdjustParticleCount(-1000));
    }
    if keys.just_pressed(KeyCode::KeyY) {
        queue.queue.push(SimCommand::AdjustParticleCount(1000));
    }
}