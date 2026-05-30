use bevy::prelude::*;
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
use super::systems::ui::setup_ui;
use super::systems::update_text::{
    update_fps_text, 
    update_particle_count_text,
};

pub struct OutputPlugin;

impl Plugin for OutputPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(FrameTimeDiagnosticsPlugin {
                max_history_length: 120,
                smoothing_factor: 0.08,
            });

        #[cfg(feature = "ui")]
        {
            app.add_systems(Startup, setup_ui)
               .add_systems(Update, update_fps_text)
               .add_systems(Update, update_particle_count_text);
        }

        #[cfg(feature = "ui-sprites")]
        {
            use super::systems::update_text::update_visibility_budget_text;
            app.add_systems(Update, update_visibility_budget_text);
        }
    }
}