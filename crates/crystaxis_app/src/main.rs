use bevy::prelude::*;
use crystaxis_core::CoreSimulationPlugin;
use crystaxis_input::InputPlugin;
use crystaxis_render::RenderPlugin;
use crystaxis_output::OutputPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(CoreSimulationPlugin)
        .add_plugins(InputPlugin)
        .add_plugins(RenderPlugin)
        .add_plugins(OutputPlugin)
        .run();
}