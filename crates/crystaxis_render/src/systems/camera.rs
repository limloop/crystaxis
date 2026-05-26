use bevy::prelude::*;

/// Ресурс, хранящий изображение для спрайтов частиц.
#[derive(Resource)]
pub struct ParticleImage(pub Handle<Image>);

/// Настраивает 3D-камеру и загружает текстуру частицы.
pub fn setup_camera_and_light(mut commands: Commands, asset_server: Res<AssetServer>) {
    // 3D-камера
    commands.spawn((
        Camera3d::default(),
        Transform::from_xyz(0.0, 0.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Освещение (для будущих 3D-объектов)
    commands.spawn((
        DirectionalLight::default(),
        Transform::from_xyz(5.0, 10.0, 5.0).looking_at(Vec3::ZERO, Vec3::Y),
    ));

    // Загружаем текстуру частицы (белый круг с мягкими краями)
    let image: Handle<Image> = asset_server.load("particle.png");
    commands.insert_resource(ParticleImage(image));
}