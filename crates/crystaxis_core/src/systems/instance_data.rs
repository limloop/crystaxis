use bevy::prelude::*;
use crate::{Position, Particle, InstanceData};

/// Собирает матрицы трансформаций для всех частиц с маркером Particle.
/// Используется для GPU-инстансинга (если фича включена в рендере).
pub fn prepare_instance_data(
    mut instance_data: ResMut<InstanceData>,
    query: Query<&Position, With<Particle>>,
) {
    instance_data.transforms.clear();
    for pos in query.iter() {
        instance_data.transforms.push(Mat4::from_translation(pos.0));
    }
}