mod plugin;

#[cfg(feature = "sprite3d")]
mod sprite3d;

#[cfg(feature = "instancing")]
mod instancing;

pub use plugin::RenderPlugin;

#[cfg(feature = "sprite3d")]
pub use sprite3d::adaptive::AdaptiveVisibilityBudget;