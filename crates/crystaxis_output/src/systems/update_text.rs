use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use crystaxis_core::Particle;
use super::ui::{FpsText, ParticleCountText};

/// Обновляет текст FPS из диагностики
pub fn update_fps_text(
    diagnostics: Res<DiagnosticsStore>,
    mut query: Query<&mut Text, With<FpsText>>,
) {
    if let Some(fps) = diagnostics.get(&FrameTimeDiagnosticsPlugin::FPS) {
        if let Some(value) = fps.smoothed() {
            for mut text in query.iter_mut() {
                **text = format!("FPS: {:.0}", value);
            }
        }
    }
}

/// Обновляет текст с количеством частиц, подсчитывая сущности с маркером Particle
pub fn update_particle_count_text(
    particle_query: Query<(), With<Particle>>,
    mut query: Query<&mut Text, (With<ParticleCountText>, Without<FpsText>)>,
) {
    let count = particle_query.iter().len();
    for mut text in query.iter_mut() {
        **text = format!("Particles: {}", count);
    }
}