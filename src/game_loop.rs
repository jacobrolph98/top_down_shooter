use bevy::prelude::*;

use crate::persistence::ProfileData;

#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum GameState {
    #[default]
    Profiles,
    Menu,
    InGame,
}

pub struct GameLoopPlugin;

impl Plugin for GameLoopPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_state::<GameState>()
            .add_systems(Update, profile_added.run_if(in_state(GameState::Profiles)));
    }
}

fn profile_added(
    mut state_change: ResMut<NextState<GameState>>,
    profile_res: Option<Res<ProfileData>>
) {
    if let Some(profile_data) = profile_res {
        if profile_data.is_added() {
            state_change.set(GameState::Menu);
        }
    }
}