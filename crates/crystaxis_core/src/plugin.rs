use bevy::prelude::*;
use crate::resources::*;
use crate::commands::*;
use crate::systems::*;

pub struct CoreSimulationPlugin;

impl Plugin for CoreSimulationPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SimulationBox>()
           .init_resource::<ParticleCount>()
           .init_resource::<CommandQueue>()
           .add_systems(Startup, spawn::spawn_initial_particles)
           .add_systems(PostUpdate, queue::process_command_queue)
           .add_systems(Update, movement::temporary_move_particles);
    }
}