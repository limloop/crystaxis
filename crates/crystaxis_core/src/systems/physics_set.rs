use bevy::prelude::*;

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub enum PhysicsSet {
    /// Первый проход Verlet (predict)
    First,
    /// Системы, вычисляющие силы и столкновения
    Forces,
    /// Финальный проход Verlet (correct)
    Last,
}