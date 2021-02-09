#[derive(Clone)]
pub struct PathBuilderDto {
    pub steps: Vec<usize>,
}

impl PathBuilderDto {
    pub fn new(steps: Vec<usize>) -> Self {
        PathBuilderDto { steps }
    }
}
