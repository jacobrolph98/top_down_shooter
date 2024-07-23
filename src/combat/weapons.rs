use bevy::prelude::*;

#[derive(Eq, Hash, PartialEq, Copy, Clone)]
pub enum Weapon {
    AutoShotgun,
    Railgun,
    AutoRifle,
    NadeLauncher,
    Minigun
}