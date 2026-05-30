use bevy::prelude::*;

pub struct RenderPlugin;

impl Plugin for RenderPlugin {
    fn build(&self, app: &mut App) {
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