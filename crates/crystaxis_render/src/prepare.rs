use bevy::prelude::*;
use crystaxis_core::Particle;

/// Данные, которые рендер подготавливает для output (например, для gizmos или UI)
#[derive(Resource, Default)]
pub struct RenderData {
    pub positions: Vec<Vec3>,
    pub count: usize,
}

/// Заполняет RenderData позициями всех частиц.
/// Вызывается каждый кадр в PostUpdate, чтобы output мог прочитать актуальные данные.
pub fn prepare_render_data(
    mut render_data: ResMut<RenderData>,
    query: Query<&Transform, With<Particle>>,
) {
    render_data.positions.clear();
    for transform in query.iter() {
        render_data.positions.push(transform.translation);
    }
    render_data.count = render_data.positions.len();
}