use bevy::prelude::*;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::platform::collections::HashSet;
use crystaxis_core::Particle;
use crate::sprite3d::systems::HasSprite;

/// Бюджет видимости с авто-регулировкой по FPS
#[derive(Resource)]
pub struct AdaptiveVisibilityBudget {
    pub limit: usize,
    pub min_limit: usize,
    pub max_limit: usize,
    pub target_fps: f64,
}

impl Default for AdaptiveVisibilityBudget {
    fn default() -> Self {
        Self {
            limit: 50_000,
            min_limit: 5_000,
            max_limit: 300_000,
            target_fps: 50.0,
        }
    }
}

/// Таймер для периодического обновления бюджета
#[derive(Resource)]
pub struct VisibilityBudgetTimer(pub Timer);

impl Default for VisibilityBudgetTimer {
    fn default() -> Self {
        Self(Timer::from_seconds(1.0, TimerMode::Repeating))
    }
}

/// Обновляет лимит видимости на основе текущего FPS
/// Скорость адаптации пропорциональна отклонению от целевого FPS
pub fn update_visibility_budget(
    time: Res<Time>,
    diagnostics: Res<DiagnosticsStore>,
    mut timer: ResMut<VisibilityBudgetTimer>,
    mut budget: ResMut<AdaptiveVisibilityBudget>,
) {
    timer.0.tick(time.delta());

    if !timer.0.just_finished() {
        return;
    }

    let Some(fps) = diagnostics
        .get(&FrameTimeDiagnosticsPlugin::FPS)
        .and_then(|v| v.smoothed())
    else {
        return;
    };

    let target = budget.target_fps;

    if fps < target - 10.0 {
        // FPS значительно ниже целевого — снижаем лимит
        let shortage = target - fps;
        let factor = (shortage / target).min(0.5); // максимум 50% снижения
        let multiplier = 1.0 - factor;
        budget.limit = ((budget.limit as f64) * multiplier) as usize;
    } else if fps > target - 5.0 && budget.limit < budget.max_limit {
        // FPS хороший — медленно повышаем лимит
        budget.limit = ((budget.limit as f64) * 1.05) as usize;
    }

    budget.limit = budget.limit.clamp(budget.min_limit, budget.max_limit);
}

/// Адаптивная система видимости: показывает ближайшие к камере частицы
pub fn adaptive_visibility(
    budget: Res<AdaptiveVisibilityBudget>,
    camera_query: Query<&GlobalTransform, With<Camera3d>>,
    particles: Query<(Entity, &Transform), With<Particle>>,
    mut sprites: Query<(&mut Visibility, Entity), With<HasSprite>>,
) {
    let Ok(camera_transform) = camera_query.single() else {
        return;
    };

    let camera_pos = camera_transform.translation();
    let camera_forward = camera_transform.forward();

    let mut candidates: Vec<(Entity, f32)> = particles
        .iter()
        .filter_map(|(entity, transform)| {
            let to_particle = transform.translation - camera_pos;
            if to_particle.dot(*camera_forward) > 0.0 {
                Some((entity, to_particle.length_squared()))
            } else {
                None
            }
        })
        .collect();

    let total_visible = candidates.len();
    let limit = budget.limit.min(total_visible);

    // Если видимых частиц меньше лимита — показываем все
    if total_visible <= limit {
        sprites.par_iter_mut().for_each(|(mut visibility, entity)| {
            // Проверяем, есть ли в candidates (все видимые)
            *visibility = if candidates.iter().any(|(e, _)| *e == entity) {
                Visibility::Visible
            } else {
                Visibility::Hidden
            };
        });
        return;
    }

    // Иначе оставляем только ближайшие
    candidates.select_nth_unstable_by(
        limit,
        |a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal),
    );
    candidates.truncate(limit);

    let visible: HashSet<Entity> = candidates.into_iter().map(|(e, _)| e).collect();

    sprites.par_iter_mut().for_each(|(mut visibility, entity)| {
        *visibility = if visible.contains(&entity) {
            Visibility::Visible
        } else {
            Visibility::Hidden
        };
    });
}