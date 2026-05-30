use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use crystaxis_core::ParticleCount;
use super::ui::{FpsText, ParticleCountText, VisibilityBudgetText};

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
    particle_count: Res<ParticleCount>,
    mut query: Query<&mut Text, (With<ParticleCountText>, Without<FpsText>)>,
) {
    if !particle_count.is_changed() {
        return;
    }

    for mut text in &mut query {
        **text = format!("Particles: {}", particle_count.0);
    }
}

/// Обновляет текст с лимитом спрайтов, если AdaptiveVisibilityBudget доступен
pub fn update_visibility_budget_text(
    budget: Option<Res<crystaxis_render::AdaptiveVisibilityBudget>>,
    mut query: Query<&mut Text, With<VisibilityBudgetText>>,
) {
    let Some(budget) = budget else {
        return;
    };

    if !budget.is_changed() {
        return;
    }

    for mut text in &mut query {
        **text = format!("Sprites: {}", budget.limit);
    }
}