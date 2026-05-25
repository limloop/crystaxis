use bevy::prelude::*;
use crate::{CommandQueue, ParticleCount, Position, SimCommand};
use super::spawn::spawn_particles;

/// Обрабатывает все накопленные команды из очереди.
/// Вычисляет итоговое требуемое количество частиц, применяет только разницу (добавление/удаление).
pub fn process_command_queue(
    mut commands: Commands,
    mut queue: ResMut<CommandQueue>,
    mut particle_count: ResMut<ParticleCount>,
    query: Query<Entity, With<Position>>,
) {
    if queue.queue.is_empty() {
        return;
    }

    let mut target = particle_count.0 as i32;
    for cmd in queue.queue.drain(..) {
        match cmd {
            SimCommand::SetParticleCount(new_count) => target = new_count as i32,
            SimCommand::AdjustParticleCount(delta) => target = (target + delta).max(0),
        }
    }

    let target = target as usize;
    let current = particle_count.0;

    if target > current {
        let to_add = target - current;
        spawn_particles(&mut commands, to_add);
    } else if target < current {
        let to_remove = current - target;
        for entity in query.iter().take(to_remove) {
            commands.entity(entity).despawn();
        }
    }

    particle_count.0 = target;
    if current != target {
        info!("Particle count: {} -> {}", current, target);
    }
}