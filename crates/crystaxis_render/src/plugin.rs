use bevy::prelude::*;
use crate::prepare::{prepare_render_data, RenderData};

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RenderData>()
           .add_systems(PostUpdate, prepare_render_data);

        #[cfg(feature = "sprite3d")]
        {
            app.add_plugins(crate::sprite3d::plugin::Sprite3dRenderPlugin);
        }

        #[cfg(feature = "instancing")]
        {
            app.add_plugins(crate::instancing::plugin::InstancingRenderPlugin);
        }
    }
}