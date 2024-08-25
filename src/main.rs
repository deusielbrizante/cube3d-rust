mod structs;
mod functions;

use std::thread::sleep;
use std::time::Duration;
use crate::structs::sides_cube::{IncrementalValuesABC, SidesCubeXYZ, ConstValues, ValuesToCalculateIJK};
use crate::functions::calculations::{calculate_for_surface};


fn main() {
    const CONST_VALUES: ConstValues = ConstValues::new();
    let mut buffer: Vec<char> = vec![CONST_VALUES.background_ascii_code; (CONST_VALUES.width * CONST_VALUES.height) as usize];
    let mut z_buffer: Vec<f32> = vec![0.0; (CONST_VALUES.width * CONST_VALUES.height) as usize];

    let mut incremental_values_abc: IncrementalValuesABC = IncrementalValuesABC::new();
    let mut cube_xyz: SidesCubeXYZ = SidesCubeXYZ::new();

    println!("\x1b[2J");

    loop {
        buffer.fill(CONST_VALUES.background_ascii_code);
        z_buffer.fill(0.0);

        let mut cube_width: f32 = CONST_VALUES.cube_width;
        let mut cube_x = -cube_width;

        while cube_x < cube_width {
            let mut cube_y = -cube_width;

            while cube_y < cube_width {
                calculate_for_surface(&incremental_values_abc, &mut cube_xyz,
                                      &ValuesToCalculateIJK { i: cube_x, j: cube_y, k: -cube_width },
                                      '.', &CONST_VALUES, &mut z_buffer, &mut buffer, cube_width);
                calculate_for_surface(&incremental_values_abc, &mut cube_xyz,
                                      &ValuesToCalculateIJK { i: cube_width, j: cube_y, k: cube_x },
                                      '$', &CONST_VALUES, &mut z_buffer, &mut buffer, cube_width);
                calculate_for_surface(&incremental_values_abc, &mut cube_xyz,
                                      &ValuesToCalculateIJK { i: -cube_width, j: cube_y, k: -cube_x },
                                      '~', &CONST_VALUES, &mut z_buffer, &mut buffer, cube_width);
                calculate_for_surface(&incremental_values_abc, &mut cube_xyz,
                                      &ValuesToCalculateIJK { i: -cube_x, j: cube_y, k: cube_width },
                                      '#', &CONST_VALUES, &mut z_buffer, &mut buffer, cube_width);
                calculate_for_surface(&incremental_values_abc, &mut cube_xyz,
                                      &ValuesToCalculateIJK { i: cube_x, j: -cube_width, k: -cube_y },
                                      ';', &CONST_VALUES, &mut z_buffer, &mut buffer, cube_width);
                calculate_for_surface(&incremental_values_abc, &mut cube_xyz,
                                      &ValuesToCalculateIJK { i: cube_x, j: cube_width, k: cube_y },
                                      '+', &CONST_VALUES, &mut z_buffer, &mut buffer, cube_width);
                cube_y += CONST_VALUES.increment_speed;
            }

            cube_x += CONST_VALUES.increment_speed;
        }

        print!("\x1b[H");
        for k in 0..(CONST_VALUES.width * CONST_VALUES.height) {
            if k % CONST_VALUES.width == 0 {
                println!();
            } else {
                print!("{}", buffer[k as usize]);
            }
        }

        incremental_values_abc.a += 0.005;
        incremental_values_abc.b += 0.005;
        sleep(Duration::from_secs_f32(0.00000000001));
    }
}