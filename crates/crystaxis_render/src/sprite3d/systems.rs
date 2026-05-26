use bevy::prelude::*;
use crate::sprite3d::plugin::Sprite3dCache;
use crystaxis_core::{Particle, Position};

/// Маркер для сущностей, уже имеющих визуальное представление
#[derive(Component)]
pub(crate) struct HasSprite;

/// Единая система: обходит все частицы, добавляет меш/материал новым,
/// обновляет Transform существующим.
/// Работает как bundle_builder из bevy_sprite3d, но без создания
/// промежуточных компонентов Sprite.
pub(crate) fn prepare_and_draw_sprites(
    mut commands: Commands,
    cache: Res<Sprite3dCache>,
    query: Query<(Entity, &Position), (With<Particle>, Without<HasSprite>)>,
    mut existing: Query<(&Position, &mut Transform), With<HasSprite>>,
) {
    // 1. Добавляем визуальные компоненты новым частицам
    for (entity, pos) in query.iter() {
        commands.entity(entity).insert((
            cache.mesh.clone(),
            cache.material.clone(),
            Transform::from_translation(pos.0),
            HasSprite,
        ));
    }

    // 2. Обновляем трансформации существующих
    for (pos, mut transform) in existing.iter_mut() {
        transform.translation = pos.0;
    }
}