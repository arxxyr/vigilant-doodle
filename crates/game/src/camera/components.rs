use bevy::prelude::*;

/// 斜向俯视相机配置
#[derive(Component)]
pub struct IsometricCamera {
    /// 相机距离目标的偏移量（相对坐标）
    pub offset: Vec3,
    /// 跟随速度（数值越大越快，0 = 瞬移）
    pub follow_speed: f32,
}

impl Default for IsometricCamera {
    fn default() -> Self {
        Self {
            // 默认：后上方 45 度俯视
            // Y=12 是高度，Z=10 是后方距离
            offset: Vec3::new(0.0, 12.0, 10.0),
            follow_speed: 5.0,
        }
    }
}

/// 相机跟随目标标记
#[derive(Component)]
pub struct CameraTarget;
