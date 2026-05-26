use bevy::prelude::*;
use bevy_sprite3d::{Sprite3dPlugin, Sprite3d};

use crate::systems::camera::setup_camera_and_light;
use crate::systems::sync::sync_transforms;
use crate::prepare::{prepare_render_data, RenderData};

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(Sprite3dPlugin);

        app.init_resource::<RenderData>()
           .add_systems(Startup, setup_camera_and_light)
           .add_systems(Update, sync_transforms)
           .add_systems(PostUpdate, prepare_render_data);

        #[cfg(feature = "standard")]
        {
            app.add_systems(Update, add_3d_sprites_to_new_particles);
        }

        #[cfg(feature = "instancing")]
        {
            app.add_plugins(crate::instancing::pipeline::InstancingRenderPlugin);
        }
    }
}

/// Система для добавления спрайтов к новым частицам.
/// Использует стандартные компоненты Bevy `Sprite` и `Sprite3d` из плагина.
#[cfg(feature = "standard")]
fn add_3d_sprites_to_new_particles(
    mut commands: Commands,
    query: Query<Entity, Added<crystaxis_core::Particle>>,
    image: Res<crate::systems::camera::ParticleImage>,
) {
    for entity in query.iter() {
        commands.entity(entity).insert((
            Sprite {
                image: image.0.clone(),
                ..default()
            },
            Sprite3d {
                pixels_per_metre: 1000.0,
                unlit: true,
                ..default()
            },
        ));
    }
}