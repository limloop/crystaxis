use bevy::prelude::*;

/// Создаёт StandardMaterial для спрайта частицы.
/// Unlit = true, чтобы не зависеть от освещения.
/// AlphaMode = Mask(0.5) для корректной прозрачности.
pub fn build_material(image: Handle<Image>) -> StandardMaterial {
    StandardMaterial {
        base_color_texture: Some(image),
        alpha_mode: AlphaMode::Mask(0.5),
        unlit: true,
        cull_mode: None, // без отбраковки, видно с обеих сторон
        ..default()
    }
}

/// Создаёт меш плоского квада размером w×h в плоскости XY, обращённого к +Z.
/// Используется как основа для всех спрайтов частиц.
pub fn quad_mesh(w: f32, h: f32) -> Mesh {
    let w2 = w / 2.0;
    let h2 = h / 2.0;

    let mut mesh = Mesh::new(
        bevy::mesh::PrimitiveTopology::TriangleList,
        bevy::asset::RenderAssetUsages::default(),
    );

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_POSITION,
        vec![
            [-w2, -h2, 0.0],
            [w2, -h2, 0.0],
            [-w2, h2, 0.0],
            [w2, h2, 0.0],
        ],
    );

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_NORMAL,
        vec![[0.0, 0.0, 1.0]; 4],
    );

    mesh.insert_attribute(
        Mesh::ATTRIBUTE_UV_0,
        vec![
            [0.0, 1.0],
            [1.0, 1.0],
            [0.0, 0.0],
            [1.0, 0.0],
        ],
    );

    mesh.insert_indices(bevy::mesh::Indices::U32(vec![0, 1, 2, 1, 3, 2]));
    mesh
}