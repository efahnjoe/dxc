#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Transform {
    pub scale: f64,
    pub deg: f64,
    pub offset_x: f64,
    pub offset_y: f64,
    pub enable_transition: bool,
}

impl Transform {
    pub fn default() -> Self {
        Self {
            scale: 1.0,
            deg: 0.0,
            offset_x: 0.0,
            offset_y: 0.0,
            enable_transition: false,
        }
    }
}
