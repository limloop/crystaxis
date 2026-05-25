use bevy::prelude::*;

/// Размеры симуляционной коробки с центром в начале координат
#[derive(Resource)]
pub struct SimulationBox {
    pub width: f32,
    pub height: f32,
    pub depth: f32,
}

impl Default for SimulationBox {
    fn default() -> Self {
        Self { width: 10.0, height: 10.0, depth: 10.0 }
    }
}

impl SimulationBox {
    pub fn new(width: f32, height: f32, depth: f32) -> Self {
        Self { width, height, depth }
    }

    /// Проверяет, находится ли точка внутри коробки
    pub fn contains(&self, pos: &Vec3) -> bool {
        pos.x >= -self.width / 2.0 && pos.x <= self.width / 2.0
            && pos.y >= -self.height / 2.0 && pos.y <= self.height / 2.0
            && pos.z >= -self.depth / 2.0 && pos.z <= self.depth / 2.0
    }
}

/// Текущее количество частиц в симуляции
#[derive(Resource)]
pub struct ParticleCount(pub usize);

impl Default for ParticleCount {
    fn default() -> Self { Self(100) }
}

/// Данные для инстансинга: матрицы трансформаций всех частиц.
/// Используется рендером при включённой фиче `instancing`.
#[derive(Resource, Default)]
pub struct InstanceData {
    pub transforms: Vec<Mat4>,
}