use bevy::prelude::*;

/// Команды, которые могут быть поставлены в очередь для выполнения за кадр.
#[derive(Debug, Clone)]
pub enum SimCommand {
    /// Задать точное количество частиц
    SetParticleCount(usize),
    /// Изменить количество на указанную дельту (положительное – добавить, отрицательное – убавить)
    AdjustParticleCount(i32),
}

/// Очередь команд. Система process_command_queue обрабатывает все накопленные команды за кадр.
#[derive(Resource, Default)]
pub struct CommandQueue {
    pub queue: Vec<SimCommand>,
}