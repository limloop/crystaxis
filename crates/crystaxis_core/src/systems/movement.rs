use bevy::prelude::*;
use crate::{Position, SimulationBox};

/// Временная система анимации частиц по синусоидальным траекториям.
/// Позже будет заменена на реальный физический движок.
pub fn temporary_move_particles(
    mut query: Query<&mut Position>,
    time: Res<Time>,
    sim_box: Res<SimulationBox>,
) {
    let t = time.elapsed_secs();
    for (i, mut pos) in query.iter_mut().enumerate() {
        let offset = i as f32;
        pos.0.x = (t * 0.5 + offset).sin() * 5.0;
        pos.0.y = (t * 0.3 + offset).cos() * 3.0;
        pos.0.z = (t * 0.2 + offset).sin() * 2.0;

        // Ограничиваем позицию внутри симуляционной коробки
        pos.0.x = pos.0.x.clamp(-sim_box.width / 2.0, sim_box.width / 2.0);
        pos.0.y = pos.0.y.clamp(-sim_box.height / 2.0, sim_box.height / 2.0);
        pos.0.z = pos.0.z.clamp(-sim_box.depth / 2.0, sim_box.depth / 2.0);
    }
}