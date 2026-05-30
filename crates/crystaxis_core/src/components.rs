use bevy::prelude::*;

/// Скорость частицы (вектор)
#[derive(Component)]
pub struct Velocity(pub Vec3);

/// Накопленная сила/ускорение для текущего шага интегрирования
#[derive(Component)]
pub struct ForceAccum(pub Vec3);

/// Маркер, указывающий, что сущность является частицей симуляции
#[derive(Component)]
pub struct Particle;