use bevy::prelude::*;

/// Позиция частицы в мировом пространстве
#[derive(Component)]
pub struct Position(pub Vec3);

/// Скорость частицы (вектор)
#[derive(Component)]
pub struct Velocity(pub Vec3);

/// Накопленная сила/ускорение для текущего шага интегрирования
#[derive(Component)]
pub struct ForceAccum(pub Vec3);

/// Маркер, указывающий, что сущность является частицей симуляции
#[derive(Component)]
pub struct Particle;