use bevy::prelude::*;
use crystaxis_core::Position;

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
    query: Query<&Position>,
) {
    render_data.positions.clear();
    for pos in query.iter() {
        render_data.positions.push(pos.0);
    }
    render_data.count = render_data.positions.len();
}