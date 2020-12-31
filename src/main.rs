use bevy::prelude::*;
use bevy::input::prelude::*;

use bevy_rapier3d::physics::{RapierPhysicsPlugin, EventQueue};
use bevy_rapier3d::render::RapierRenderPlugin;
use bevy_rapier3d::rapier::dynamics::RigidBodyBuilder;
use bevy_rapier3d::rapier::geometry::ColliderBuilder;

mod fps;
use fps::{FpsCamera, FpsCameraPlugin};

fn main() {
    App::build()
    .add_resource(Msaa { samples: 8 })
    .add_resource(State::InGame)
    .add_plugins(DefaultPlugins)
    .add_plugin(FpsCameraPlugin) // Testing Camera
    .add_plugin(RapierPhysicsPlugin) // Physics Engine
    // .add_plugin(RapierRenderPlugin)
    .add_startup_system(setup.system())
    // .add_startup_system(play_music.system())
    .add_system(print_events.system())
    .run();
}

enum State {
    /// State to indicate we're on the Main Menu
    MainMenu,
    /// State to indicate we're in the Settings Menu
    Settings,
    /// State to pick which map (server) to play on
    MapPicker,
    /// State to indicate we're in game
    InGame
}

fn setup(commands: &mut Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>, asset_server: Res<AssetServer>) {

    let plane_rigid_body = RigidBodyBuilder::new_static();
    let plane_colider = ColliderBuilder::cuboid(500.0, 0.1, 500.0);

    let ball_rigid_body = RigidBodyBuilder::new_dynamic().translation(0.0, 10., 0.0);
    let ball_colider = ColliderBuilder::ball(5.);

    let speeder_mesh: Handle<Mesh> = asset_server.load("models/Vehicles/74Z/74Z.gltf");
    let speeder_mat: Handle<StandardMaterial> = asset_server.load("models/Vehicles/74Z/74Z.gltf");

    commands
        // .spawn_scene(asset_server.load("models/CloneTrooper/StandardClone.gltf"))
        // .spawn_scene(asset_server.load("models/Felucia/segmented.gltf"))
        // .spawn_scene(asset_server.load("models/Vehicles/74Z/74Z.gltf"))
        .spawn(PbrBundle {
            mesh: speeder_mesh,
            material: speeder_mat,
            // transform: Transform::from_translation(Vec3::new(0.0, 0.5, 0.0)),
            ..Default::default()
        })

        .spawn((plane_colider, plane_rigid_body))
        .with_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Plane { size: 500. })),
            material: materials.add(Color::rgb(0.331, 0.2, 0.2).into()),
            transform: Transform::from_translation(Vec3::new(0.0, -1., 0.0)),
            ..Default::default()
        })
        .spawn((ball_colider, ball_rigid_body))
        .with_bundle(PbrBundle {
            mesh: meshes.add(Mesh::from(shape::Icosphere { radius: 5.0, ..Default::default()})),
            material: materials.add(Color::rgb(0.331, 0.2, 0.2).into()),
            transform: Transform::from_translation(Vec3::new(0.0, 100., 0.0)),
            ..Default::default()
        })

        .spawn(LightBundle {
            transform: Transform::from_translation(Vec3::new(4.0, 50.0, 4.0)),
            ..Default::default()
        })

        .spawn(Camera3dBundle {
            transform: Transform::from_translation(Vec3::new(0., 1.5, 0.))
                .looking_at(Vec3::new(0.0, 0.3, 0.0), Vec3::unit_y()),
            ..Default::default()
        })

        .with(FpsCamera::default());
}


fn play_music(asset_server: Res<AssetServer>, audio: Res<Audio>, state: Res<State>) {
    println!("Playing music.");
    match *state {
        State::MainMenu => {
            let music = asset_server.load("sounds/music/CloneArmyTheme.mp3");
            audio.play(music);
            println!("Music done.");
        },
        State::InGame => {
            let music = asset_server.load("sounds/music/Mandalorian_Theme.mp3");
            audio.play(music);
        },
        _ => {},
    }
}

fn print_events(events: Res<EventQueue>) {
    while let Ok(proximity_event) = events.proximity_events.pop() {
        println!("Received proximity event: {:?}", proximity_event);
    }

    while let Ok(contact_event) = events.contact_events.pop() {
        println!("Received contact event: {:?}", contact_event);
    }
}