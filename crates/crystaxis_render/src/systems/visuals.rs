use bevy::prelude::*;
use crystaxis_core::{Position, Particle};

/// Для стандартного режима: добавляет меш сферы и материал к новым частицам (сущностям с Position и Particle)
pub fn add_visuals_to_new_particles(
    mut commands: Commands,
    query: Query<Entity, (Added<Position>, With<Particle>)>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    let sphere_mesh = meshes.add(Sphere::new(0.05).mesh());
    let white_material = materials.add(StandardMaterial::default());

    for entity in query.iter() {
        commands.entity(entity).insert((
            Mesh3d(sphere_mesh.clone()),
            MeshMaterial3d(white_material.clone()),
            Transform::default(),
        ));
    }
}