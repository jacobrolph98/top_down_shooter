use bevy::prelude::*;

pub struct AttackPlugin;

enum AttackType {
    RayCast,
    ShapeCast {
        radius: f32,
    }
}

struct AttackStats {
    cooldown: f32,
    range: f32,
    damage: u32
}

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        
    }
}

fn shape_cast(

) {

}

fn ray_cast(

) {
    
}