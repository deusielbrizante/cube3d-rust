mod structs;
mod functions;

use std::thread::sleep;
use std::time::Duration;
use crate::structs::sides_cube::{IncrementalValuesABC, SidesCubeXYZ, StaticValues, ValuesToCalculateIJK};
use crate::functions::calculations::{calculate_for_surface};

const STATIC_VALUES: StaticValues = StaticValues::new();

fn main() {
    let mut buffer: Vec<char> = vec![STATIC_VALUES.background_ascii_code; (STATIC_VALUES.width * STATIC_VALUES.height) as usize];
    let mut z_buffer: Vec<f32> = vec![0.0; (STATIC_VALUES.width * STATIC_VALUES.height) as usize];

    let mut incremental_values_abc: IncrementalValuesABC = IncrementalValuesABC::new();
    let mut cube_xyz: SidesCubeXYZ = SidesCubeXYZ::new();

    println!("\x1b[2J");

    loop {
        buffer.fill(STATIC_VALUES.background_ascii_code);
        z_buffer.fill(0.0);

        let mut cube_width: f32 = STATIC_VALUES.cube_width;
        let mut cube_x = -cube_width;

        while cube_x < cube_width {
            let mut cube_y = -cube_width;

            while cube_y < cube_width {
                calculate_for_surface(&incremental_values_abc, &mut cube_xyz,
                                      &ValuesToCalculateIJK { i: cube_x, j: cube_y, k: -cube_width },
                                      '.', &STATIC_VALUES, &mut z_buffer, &mut buffer, cube_width);
                calculate_for_surface(&incremental_values_abc, &mut cube_xyz,
                                      &ValuesToCalculateIJK { i: cube_width, j: cube_y, k: cube_x },
                                      '$', &STATIC_VALUES, &mut z_buffer, &mut buffer, cube_width);
                calculate_for_surface(&incremental_values_abc, &mut cube_xyz,
                                      &ValuesToCalculateIJK { i: -cube_width, j: cube_y, k: -cube_x },
                                      '~', &STATIC_VALUES, &mut z_buffer, &mut buffer, cube_width);
                calculate_for_surface(&incremental_values_abc, &mut cube_xyz,
                                      &ValuesToCalculateIJK { i: -cube_x, j: cube_y, k: cube_width },
                                      '#', &STATIC_VALUES, &mut z_buffer, &mut buffer, cube_width);
                calculate_for_surface(&incremental_values_abc, &mut cube_xyz,
                                      &ValuesToCalculateIJK { i: cube_x, j: -cube_width, k: -cube_y },
                                      ';', &STATIC_VALUES, &mut z_buffer, &mut buffer, cube_width);
                calculate_for_surface(&incremental_values_abc, &mut cube_xyz,
                                      &ValuesToCalculateIJK { i: cube_x, j: cube_width, k: cube_y },
                                      '+', &STATIC_VALUES, &mut z_buffer, &mut buffer, cube_width);
                cube_y += STATIC_VALUES.increment_speed;
            }

            cube_x += STATIC_VALUES.increment_speed;
        }

        print!("\x1b[H");
        for k in 0..(STATIC_VALUES.width * STATIC_VALUES.height) {
            if k % STATIC_VALUES.width == 0 {
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