use bevy::prelude::*;
// use bevy_third_person_camera::{ThirdPersonCamera, Zoom};

pub struct WeaponPlugin;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum WeaponType {
    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Empty,
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

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum WeaponIndex {
    #[default]
    None = 0,
    One,
    Two,
    Three,
    Four,
    Five,
    Six,
}

#[derive(Component)]
pub struct Weapon {
    index: WeaponIndex,
    weapon_type: WeaponType,
}

impl Weapon {
    pub fn new(index: WeaponIndex, weapon_type: WeaponType) -> Self {
        Self { index, weapon_type }
    }
}

// impl default for Weapon
impl Default for Weapon {
    fn default() -> Self {
        Self {
            index: WeaponIndex::None,
            weapon_type: WeaponType::Empty,
        }
    }
}
