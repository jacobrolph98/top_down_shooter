use bevy::prelude::*;
use bevy_persistent::prelude::*;

use serde::{
    Serialize,
    Deserialize
};

use crate::game_loop::GameState;

#[derive(Event)]
pub struct SelectProfileEvent {
    pub profile_name: String
}

#[derive(Resource, Serialize, Deserialize)]
pub struct Profiles {
    pub profiles: Vec<ProfileData>
}

#[derive(Resource, Serialize, Deserialize, Default)]
pub struct ProfileData {
    pub profile_name: String,
    pub money: i32
}

pub struct PersistencePlugin;

impl Plugin for PersistencePlugin {
    fn build(&self, app: &mut App) {
        app
            .add_event::<SelectProfileEvent>()
            .add_systems(OnEnter(GameState::Profiles), load_profiles)
            .add_systems(Update, select_profile_listener.run_if(in_state(GameState::Profiles)))
            .add_systems(OnExit(GameState::Profiles), unload_profiles);

    }
}

fn load_profiles(
    mut commands: Commands
) {
    let path = dirs::document_dir().unwrap().join("game");
    commands.insert_resource(
        Persistent::<Profiles>::builder()
        .name("profiles")
        .format(StorageFormat::Bincode)
        .path(path.join("profiles.bincode"))
        .default(Profiles{ profiles: vec![]})
        .build()
        .expect("Failed to initialize profile resource")
    );
}

fn unload_profiles(
    mut profiles: ResMut<Persistent<Profiles>>
) {
    profiles.unload().expect("Failed to persist profiles before unloading");
}

fn select_profile_listener(
    mut select_reader: EventReader<SelectProfileEvent>,
    mut commands: Commands,
    profiles: Res<Persistent<Profiles>>
) {
    for event in select_reader.read() {
        if let Some(selected_profile) = profiles.profiles.iter().find(|prof| prof.profile_name == event.profile_name) {
            commands.insert_resource(ProfileData{
                profile_name: selected_profile.profile_name.clone(),
                money: selected_profile.money
            });
        }
        
    }
}