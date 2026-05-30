use bevy::prelude::*;
use crate::sprite3d::plugin::Sprite3dCache;
use crystaxis_core::Particle;

/// Маркер для сущностей, уже имеющих визуальное представление
#[derive(Component)]
pub(crate) struct HasSprite;

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