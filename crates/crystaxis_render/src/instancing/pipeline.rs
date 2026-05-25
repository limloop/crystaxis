use bevy::prelude::*;
use bevy::render::{RenderApp, Render, Set};
use crate::instancing::buffer::ParticleInstanceBuffer;

/// Плагин, который настраивает кастомный рендер-пайплайн для инстансинга частиц.
/// Активируется только при фиче `instancing`.
pub struct InstancingRenderPlugin;

impl Plugin for InstancingRenderPlugin {
    fn build(&self, _app: &mut App) {
        // TODO: реализовать полный пайплайн с шейдером, нодой и т.д.
        // Здесь будет инициализация ParticleInstanceBuffer, регистрация draw-функций.
    }
}