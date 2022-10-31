use bevy::prelude::*;
use bevy::window;
use bevy::window::WindowMode;
use bevy_rapier3d::prelude::*;

#[derive(Component, Default)]
pub struct Player;

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands
            .spawn_bundle(TransformBundle::from(Transform {
                translation: Vec3::new(0.0, 0., 0.),
                rotation: Quat::from_rotation_y(-std::f32::consts::FRAC_PI_4),
                ..default()
            }))
            .with_children(|cell| {
                cell.spawn_bundle(SceneBundle {
                    scene: asset_server.load("vaisseau2/scene.gltf#Scene0"),
                    ..Default::default()
                });
                
            })
            .insert(Player)
            .insert(RigidBody::Dynamic)
            .insert(Velocity {
                linvel: Vec3::new(0.0, 0.0, 0.0),
                angvel: Vec3::new(0.0, 0.0, 0.0),
            })
            .insert(Collider::cuboid(0.5, 0.5, 0.5))
            .insert(GravityScale(1.5))
            .insert(Sleeping::disabled())
            .insert(Ccd::enabled())
            .insert(LockedAxes::ROTATION_LOCKED)
            ;
    
}
fn move_player() {

}
fn camera_focus() {

}
fn main() {
    App::new()
        //.init_resource::<Game>()
        .insert_resource(ClearColor(Color::rgb(0.53, 0.53, 0.53)))
        .insert_resource(WindowDescriptor {
            title: "Jeux video".to_string(),
            width: 500.0,
            height: 400.0,
            present_mode: window::PresentMode::Fifo,
            mode: WindowMode::Windowed,
            cursor_locked: true,
            cursor_visible: false,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::default())
        .add_plugin(RapierDebugRenderPlugin::default())
        .add_startup_system(setup)
        .add_system(move_player)
        //.add_system(camera::camera_focus)
        .run();
}
