mod systems;
mod prepare;
mod plugin;

// Модуль инстансинга подключается только при соответствующей фиче
#[cfg(feature = "instancing")]
mod instancing;

pub use plugin::RenderPlugin;
pub use prepare::RenderData;