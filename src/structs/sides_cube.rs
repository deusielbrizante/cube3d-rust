pub struct IncrementalValuesABC {
    pub a: f32,
    pub b: f32,
    pub c: f32,
}

impl IncrementalValuesABC {
    pub fn new() -> Self {
        Self { a: 0.0, b: 0.0, c: 0.0 }
    }

    pub fn new_abc(a: f32, b: f32, c: f32) -> Self {
        Self { a, b, c }
    }
}

pub struct SidesCubeXYZ {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl SidesCubeXYZ {
    pub fn new() -> Self {
        Self { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn new_xyz(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
}

pub struct ValuesToCalculateIJK {
    pub i: f32,
    pub j: f32,
    pub k: f32,
}

impl ValuesToCalculateIJK {
    pub fn new() -> Self {
        Self { i: 0.0, j: 0.0, k: 0.0 }
    }

    fn new_ijk(i: f32, j: f32, k: f32) -> Self {
        Self { i, j, k }
    }
}

pub struct ConstValues {
    pub width: i32,
    pub height: i32,
    pub cube_width: f32,
    pub background_ascii_code: char,
    pub increment_speed: f32,
    pub distance_from_cam: i32,
    pub k1: f32,
}

impl ConstValues {
    pub const fn new() -> Self {
        Self {
            width: 280,
            height: 50,
            cube_width: 15.0,
            background_ascii_code: ' ',
            increment_speed: 0.6,
            distance_from_cam: 60,
            k1: 50.0,
        }
    }
}