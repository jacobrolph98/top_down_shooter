use bevy::window::WindowResolution;
use bevy::
    {
        prelude::*, core_pipeline::bloom::BloomSettings, render::camera::ScalingMode
    };
use avian2d::prelude::*;

use bevy_lunex::{CursorBundle, UiDebugPlugin};
use bevy_persistent::Persistent;
use top_down_shooter::interface::{InterfacePlugin, MainUi, ProfileListRoute};

use top_down_shooter::{
    game_loop::{GameLoopPlugin, GameState},
    persistence::{PersistencePlugin, Profiles}
};



fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(
            WindowPlugin {
                primary_window: Some(Window {
                    title: "Game name".into(),
                    resolution: WindowResolution::new(1920.0, 1080.0),
                    ..default()
                }),
                ..default()
            }
        ))
        .add_plugins((PhysicsPlugins::default(), InterfacePlugin, PersistencePlugin))
        //.add_plugins(UiDebugPlugin::<MainUi>::new())
        .add_plugins((GameLoopPlugin))
        .add_systems(Startup, init_camera)
        .add_systems(Update, profile_loaded.run_if(in_state(GameState::Profiles)))
        .run();
}

fn init_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
        projection: OrthographicProjection {
            scale: 5.0,
            scaling_mode: ScalingMode::AutoMax {
                max_width:1920.0, 
                max_height: 1080.0
            },
            ..default()
        },
        transform: Transform::from_xyz(0.0, 0.0, 1000.0),
        camera: Camera {
            hdr: true,
            ..default()
        },
        ..default()
        },
        BloomSettings::NATURAL,
        MainUi
    )).with_children(|cam| {
        cam.spawn(CursorBundle::default());
    });
}

fn profile_loaded(
    mut commands: Commands,
    profiles_res: Option<Res<Persistent<Profiles>>>
) {
    if let Some(profiles) = profiles_res {
        if profiles.is_added() {
            commands.spawn(ProfileListRoute);
        }
    }
}