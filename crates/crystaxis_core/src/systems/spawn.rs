use bevy::prelude::*;
use crate::{Velocity, Particle, ParticleCount};

/// Создаёт начальное количество частиц при старте приложения
pub fn spawn_initial_particles(mut commands: Commands, count: Res<ParticleCount>) {
    spawn_particles(&mut commands, count.0);
}

/// Универсальная функция спавна заданного количества частиц с псевдо-распределением по кругу
pub fn spawn_particles(commands: &mut Commands, count: usize) {
    for i in 0..count {
        let angle = i as f32 * 0.5;
        let radius = 3.0 + (i as f32 * 0.1).sin() * 2.0;
        let pos = Vec3::new(
            angle.cos() * radius,
            angle.sin() * radius,
            (i as f32 * 0.3).sin() * 2.0,
        );
        commands.spawn((
            Transform::from_translation(pos),
            Velocity(Vec3::ZERO),
            Particle,
        ));
    }
}