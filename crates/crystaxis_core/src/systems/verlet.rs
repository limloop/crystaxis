use bevy::prelude::*;
use crate::{Particle, Velocity, ForceAccum, OldForce};

pub fn verlet_predict(
    mut query: Query<(&mut Transform, &mut Velocity, &ForceAccum, &mut OldForce), With<Particle>>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    for (mut transform, mut vel, force, mut old_force) in query.iter_mut() {
        let a_old = force.0;
        old_force.0 = a_old;
        transform.translation += vel.0 * dt + 0.5 * a_old * dt * dt;
        vel.0 += 0.5 * a_old * dt;
    }
}

pub fn verlet_correct(
    mut query: Query<(&mut Velocity, &ForceAccum, &OldForce), With<Particle>>,
    time: Res<Time>,
) {
    let dt = time.delta_secs();
    for (mut vel, force, _old_force) in query.iter_mut() {
        let a_new = force.0;
        vel.0 += 0.5 * a_new * dt;
    }
}