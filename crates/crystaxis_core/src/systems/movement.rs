use bevy::prelude::*;
use crate::{Particle, SimulationBox};

/// Временная система анимации частиц по синусоидальным траекториям.
pub fn temporary_move_particles(
    mut query: Query<&mut Transform, With<Particle>>,
    time: Res<Time>,
    sim_box: Res<SimulationBox>,
) {
    let t = time.elapsed_secs();
    for (i, mut transform) in query.iter_mut().enumerate() {
        let offset = i as f32;
        let mut x = (t * 0.5 + offset).sin() * 5.0;
        let mut y = (t * 0.3 + offset).cos() * 3.0;
        let mut z = (t * 0.2 + offset).sin() * 2.0;

        // Ограничиваем позицию внутри симуляционной коробки
        x = x.clamp(-sim_box.width / 2.0, sim_box.width / 2.0);
        y = y.clamp(-sim_box.height / 2.0, sim_box.height / 2.0);
        z = z.clamp(-sim_box.depth / 2.0, sim_box.depth / 2.0);

        transform.translation = Vec3::new(x, y, z);
    }
}