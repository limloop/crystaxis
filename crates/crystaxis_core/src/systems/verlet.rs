use bevy::prelude::*;
use crate::{Particle, Velocity, ForceAccum, OldForce};

/// Первый проход Velocity Verlet:
/// - Сохраняет текущее ускорение как OldForce
/// - Обновляет позицию: p += v*dt + 0.5*a_old*dt²
/// - Обновляет скорость наполовину: v += 0.5*a_old*dt
pub fn verlet_predict(
    mut query: Query<(&mut Transform, &mut Velocity, &ForceAccum, &mut OldForce), With<Particle>>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    query.par_iter_mut().for_each(|(mut transform, mut vel, force, mut old_force)| {
        let a_old = force.0;
        old_force.0 = a_old;
        transform.translation += vel.0 * dt + 0.5 * a_old * dt * dt;
        vel.0 += 0.5 * a_old * dt;
    });
}

/// Второй проход Velocity Verlet:
/// - Завершает обновление скорости: v += 0.5*a_new*dt
pub fn verlet_correct(
    mut query: Query<(&mut Velocity, &ForceAccum, &OldForce), With<Particle>>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    query.par_iter_mut().for_each(|(mut vel, force, _old_force)| {
        let a_new = force.0;
        vel.0 += 0.5 * a_new * dt;
    });
}