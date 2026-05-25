use bevy::prelude::*;
use crate::systems::camera::setup_camera_and_light;
use crate::systems::sync::sync_transforms;
use crate::prepare::{prepare_render_data, RenderData};

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RenderData>()
           .add_systems(Startup, setup_camera_and_light)
           .add_systems(Update, sync_transforms)
           .add_systems(PostUpdate, prepare_render_data);

        // Стандартный рендер (сущности с мешами) включается только с фичей standard
        #[cfg(feature = "standard")]
        {
            app.add_systems(Update, crate::systems::visuals::add_visuals_to_new_particles);
        }

        // Инстансинг-рендер подключается только с фичей instancing
        #[cfg(feature = "instancing")]
        {
            app.add_plugins(crate::instancing::pipeline::InstancingRenderPlugin);
        }
    }
}