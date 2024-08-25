use crate::structs::sides_cube::{IncrementalValuesABC, SidesCubeXYZ, ConstValues, ValuesToCalculateIJK};

pub fn calculate_x(incremental_values_abc: &IncrementalValuesABC, values_calc_ijk: &ValuesToCalculateIJK) -> f32 {
    values_calc_ijk.j * incremental_values_abc.a.sin() * incremental_values_abc.b.sin() * incremental_values_abc.c.cos()
        - values_calc_ijk.k * incremental_values_abc.a.cos() * incremental_values_abc.b.sin() * incremental_values_abc.c.cos()
        + values_calc_ijk.j * incremental_values_abc.a.cos() * incremental_values_abc.c.sin()
        + values_calc_ijk.k * incremental_values_abc.a.sin() * incremental_values_abc.c.sin()
        + values_calc_ijk.i * incremental_values_abc.b.cos() * incremental_values_abc.c.cos()
}

pub fn calculate_y(incremental_values_abc: &IncrementalValuesABC, values_calc_ijk: &ValuesToCalculateIJK) -> f32 {
    values_calc_ijk.j * incremental_values_abc.a.cos() * incremental_values_abc.c.cos()
        + values_calc_ijk.k * incremental_values_abc.a.sin() * incremental_values_abc.c.cos()
        - values_calc_ijk.j * incremental_values_abc.a.sin() * incremental_values_abc.b.sin() * incremental_values_abc.c.sin()
        + values_calc_ijk.k * incremental_values_abc.a.cos() * incremental_values_abc.b.sin() * incremental_values_abc.c.sin()
        - values_calc_ijk.i * incremental_values_abc.b.cos() * incremental_values_abc.c.sin()
}

pub fn calculate_z(incremental_values_abc: &IncrementalValuesABC, values_calc_ijk: &ValuesToCalculateIJK) -> f32 {
    values_calc_ijk.k * incremental_values_abc.a.cos() * incremental_values_abc.b.cos()
        - values_calc_ijk.j * incremental_values_abc.a.sin() * incremental_values_abc.b.cos()
        + values_calc_ijk.i * incremental_values_abc.b.sin()
}

pub fn calculate_for_surface(incremental_values_abc: &IncrementalValuesABC, cube_xyz: &mut SidesCubeXYZ, values_calc_ijk: &ValuesToCalculateIJK,
                             ch: char, static_values: &ConstValues, z_buff: &mut Vec<f32>, buffer: &mut Vec<char>, cube_width: f32) {
    cube_xyz.x = calculate_x(incremental_values_abc, values_calc_ijk);
    cube_xyz.y = calculate_y(incremental_values_abc, values_calc_ijk);
    cube_xyz.z = calculate_z(incremental_values_abc, values_calc_ijk) + static_values.distance_from_cam as f32;

    let ooz: f32 = 1.0 / cube_xyz.z;
    let xp: i32 = (static_values.width as f32 / 2.0f32 - 2.0f32 * cube_width + static_values.k1 * ooz * cube_xyz.x * 2.0f32) as i32;
    let yp: i32 = (static_values.height as f32 / 2.0f32 + static_values.k1 * ooz * cube_xyz.y) as i32;

    let idx: i32 = xp + yp * static_values.width;

    if idx >= 0 && idx < static_values.width * static_values.height {
        if ooz > z_buff[idx as usize] {
            z_buff[idx as usize] = ooz;
            buffer[idx as usize] = ch;
        }
    }
}