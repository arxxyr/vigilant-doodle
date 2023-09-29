
use bevy::prelude::*;
// use bevy_third_person_camera::{ThirdPersonCamera, Zoom};

pub struct WeaponPlugin;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum WeaponType {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Fist,
    Melee,
    Pistol,
    Rifle,
    Shotgun,
    MachineGun,
    Sniper,
    RocketLauncher,
    GrenadeLauncher,
    Laser,
    Sword,
    Bow,
    Crossbow,
    Staff,
}

#[derive(Component)]
pub struct Weapon{
    weapon_type: WeaponType,
    damage: f32,
}