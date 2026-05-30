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
           .add_systems(PostUpdate, queue::process_command_queue);

        #[cfg(feature = "verlet")]
        {
            // Определяем порядок сетов
            app.configure_sets(FixedUpdate, (
                physics_set::PhysicsSet::First,
                physics_set::PhysicsSet::Forces,
                physics_set::PhysicsSet::Last,
            ).chain());

            // Добавляем системы Verlet в нужные сеты
            app.add_systems(FixedUpdate, verlet::verlet_predict.in_set(physics_set::PhysicsSet::First));
            app.add_systems(FixedUpdate, verlet::verlet_correct.in_set(physics_set::PhysicsSet::Last));
        }
    }
}