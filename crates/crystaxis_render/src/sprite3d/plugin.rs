use bevy::prelude::*;
use crate::sprite3d::material::{build_material, quad_mesh};
use crate::sprite3d::systems::add_sprites_to_new_particles;

/// Ресурс с общими мешем и материалом для всех спрайтов частиц
#[derive(Resource)]
pub struct Sprite3dCache {
    pub mesh: Mesh3d,
    pub material: MeshMaterial3d<StandardMaterial>,
}

pub struct Sprite3dRenderPlugin;

impl Plugin for Sprite3dRenderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_sprite3d_cache)
           .add_systems(Startup, setup_camera_and_light)
           .add_systems(PostUpdate, add_sprites_to_new_particles);
    }
}

fn setup_sprite3d_cache(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    asset_server: Res<AssetServer>,
) {
    let mesh = Mesh3d(meshes.add(quad_mesh(0.2, 0.2)));
    let image: Handle<Image> = asset_server.load("particle.png");
    let material = MeshMaterial3d(materials.add(build_material(image)));

    commands.insert_resource(Sprite3dCache { mesh, material });
}

fn setup_camera_and_light(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(5.0, 10.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));
}