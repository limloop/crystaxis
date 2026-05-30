use bevy::prelude::*;
use crate::{Velocity, ForceAccum, Particle, ParticleCount, OldForce, SimulationBox};

/// Создаёт начальное количество частиц при старте приложения
pub fn spawn_initial_particles(
    mut commands: Commands,
    count: Res<ParticleCount>,
    sim_box: Res<SimulationBox>,
) {
    spawn_particles(&mut commands, count.0, &sim_box);
}

/// Универсальная функция спавна заданного количества частиц
pub fn spawn_particles(commands: &mut Commands, count: usize, sim_box: &SimulationBox) {
    for _ in 0..count {
        let pos = Vec3::new(
            rand::random_range(-sim_box.width / 2.0..sim_box.width / 2.0),
            rand::random_range(-sim_box.height / 2.0..sim_box.height / 2.0),
            rand::random_range(-sim_box.depth / 2.0..sim_box.depth / 2.0),
        );
        let vel = Vec3::new(
            rand::random_range(-1.0..1.0),
            rand::random_range(-1.0..1.0),
            rand::random_range(-1.0..1.0),
        );
        commands.spawn((
            Transform::from_translation(pos),
            Velocity(vel),
            ForceAccum(Vec3::ZERO),
            OldForce(Vec3::ZERO),
            Particle,
        ));
    }
}