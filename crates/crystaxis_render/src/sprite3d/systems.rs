use bevy::prelude::*;
use bevy::camera::primitives::{Frustum, Sphere};
use bevy::platform::collections::HashSet;
use crate::sprite3d::plugin::Sprite3dCache;
use crystaxis_core::Particle;

/// Маркер для сущностей, уже имеющих визуальное представление
#[derive(Component)]
pub(crate) struct HasSprite;

/// Максимальное количество спрайтов для отрисовки (предотвращает перегрузку GPU)
const MAX_VISIBLE_SPRITES: usize = 100000;

/// Добавляет визуальные компоненты новым частицам
pub(crate) fn add_sprites_to_new_particles(
    mut commands: Commands,
    cache: Res<Sprite3dCache>,
    query: Query<Entity, (With<Particle>, Without<HasSprite>)>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert((
            cache.mesh.clone(),
            cache.material.clone(),
            HasSprite,
            Visibility::Visible,
        ));
    }
}

/// Адаптивная система: управляет видимостью спрайтов на основе
/// видимости камеры и лимита отрисовки.
pub(crate) fn adaptive_visibility(
    camera_query: Query<(&Frustum, &GlobalTransform)>,
    particles: Query<(Entity, &Transform), With<Particle>>,
    mut has_sprite: Query<(&mut Visibility, Entity), With<HasSprite>>,
) {
    let Ok((frustum, camera_transform)) = camera_query.single() else {
        return;
    };

    // Собираем все видимые частицы с расстоянием до камеры
    let mut visible: Vec<(Entity, f32)> = particles
        .iter()
        .filter_map(|(entity, transform)| {
            let world_pos = transform.translation;
            let sphere = Sphere {
                center: world_pos.into(),
                radius: 0.0,
            };
            if frustum.intersects_sphere(&sphere, false) {
                let distance = camera_transform.translation().distance(world_pos);
                Some((entity, distance))
            } else {
                None
            }
        })
        .collect();

    // Сортируем по расстоянию (ближние первые)
    visible.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap_or(std::cmp::Ordering::Equal));

    // Ограничиваем количество видимых спрайтов
    let limit = MAX_VISIBLE_SPRITES.min(visible.len());
    let to_show: HashSet<Entity> = visible.iter().take(limit).map(|(e, _)| *e).collect();

    // Переключаем видимость
    for (mut visibility, entity) in has_sprite.iter_mut() {
        if to_show.contains(&entity) {
            *visibility = Visibility::Visible;
        } else {
            *visibility = Visibility::Hidden;
        }
    }
}