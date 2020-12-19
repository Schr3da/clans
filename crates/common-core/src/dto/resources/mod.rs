#[derive(Clone)]
pub struct ResourcesDto {
    pub food: i32,
    pub materials: i32,
}

impl ResourcesDto {
    pub fn new() -> Self {
        ResourcesDto {
            food: 0,
            materials: 0,
        }
    }
}
