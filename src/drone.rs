use bevy::prelude::*;
use crate::combat::weapons::Weapon;

#[derive(Resource)]
struct DroneStats {
    move_speed: f32,
    acceleration: f32,
    weapon: Weapon
}

pub struct DronePlugin;

impl Plugin for DronePlugin {
    fn build(&self, app: &mut App) {
        
    }
}

fn spawn_drone(

) {

}

fn drone_follow_target(

) {

}

fn drone_aim_target(

) {

}

fn drone_fire(

) {
    
}