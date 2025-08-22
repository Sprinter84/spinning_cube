use macroquad::prelude::*;

fn get_x(x:f32) -> f32 {
    let scale = screen_width().min(screen_height());
    let offset_x = (screen_width() - scale) / 2.0;
    let x_point = offset_x + ((x / 2.0) + 0.5) * scale;
    return x_point
}

fn get_y(y:f32) -> f32 {
    let scale = screen_height().min(screen_width());
    let offset_y = (screen_height() - scale) / 2.0;
    let y_point = offset_y + (-(y / 2.0) + 0.5) * scale;
    return y_point
}

fn project_point(point: [f32; 3], fov_deg: f32, aspect_ratio: f32, near: f32, far: f32) -> Option<[f32; 2]> {
    let fov_rad = fov_deg * std::f32::consts::PI / 180.0;
    let f = 1.0 / (fov_rad / 2.0).tan();

    let x = point[0];
    let y = point[1];
    let z = point[2];

    // w_proj = z — camera looks towards positive Z
    if z <= 0.0 {
        return None;
    }

    let x_proj = f / aspect_ratio * x;
    let y_proj = f * y;
    let z_proj = (far + near) / (near - far) * z + (2.0 * far * near) / (near - far);
    let w_proj = z;

    // Normalize screen coordinates
    let x_ndc = x_proj / w_proj;
    let y_ndc = y_proj / w_proj;

    Some([x_ndc, y_ndc])
}

fn rotate_cube(cube: &mut [f32; 3], alpha: f32, beta: f32, gamma: f32) -> [f32; 3] {
    // Move cube along the Z axis
    cube[2] -= 1.0;

    let (x, y, z) = (cube[0], cube[1], cube[2]);

    // Rotate by X
    let sin_a = alpha.sin();
    let cos_a = alpha.cos();
    let y1 = y * cos_a - z * sin_a;
    let z1 = y * sin_a + z * cos_a;

    // Rotate by Y
    let sin_b = beta.sin();
    let cos_b = beta.cos();
    let x2 = x * cos_b + z1 * sin_b;
    let z2 = -x * sin_b + z1 * cos_b;

    // Rotate by Z
    let sin_g = gamma.sin();
    let cos_g = gamma.cos();
    let x3 = x2 * cos_g - y1 * sin_g;
    let y3 = x2 * sin_g + y1 * cos_g;

    // Update coordinates
    cube[0] = x3;
    cube[1] = y3;
    cube[2] = z2;

    // Return cube
    cube[2] += 1.0;

    *cube
}

fn draw_cube2d(cube: &[[f32; 2]; 8]) {

    // Front face
    draw_line(get_x(cube[0][0]), get_y(cube[0][1]), get_x(cube[1][0]), get_y(cube[1][1]), 2.0, BLUE);
    draw_line(get_x(cube[0][0]), get_y(cube[0][1]), get_x(cube[2][0]), get_y(cube[2][1]), 2.0, BLUE);
    draw_line(get_x(cube[3][0]), get_y(cube[3][1]), get_x(cube[1][0]), get_y(cube[1][1]), 2.0, BLUE);
    draw_line(get_x(cube[3][0]), get_y(cube[3][1]), get_x(cube[2][0]), get_y(cube[2][1]), 2.0, BLUE);

    // Back face
    draw_line(get_x(cube[4][0]), get_y(cube[4][1]), get_x(cube[5][0]), get_y(cube[5][1]), 2.0, BLUE);
    draw_line(get_x(cube[4][0]), get_y(cube[4][1]), get_x(cube[6][0]), get_y(cube[6][1]), 2.0, BLUE);
    draw_line(get_x(cube[7][0]), get_y(cube[7][1]), get_x(cube[5][0]), get_y(cube[5][1]), 2.0, BLUE);
    draw_line(get_x(cube[7][0]), get_y(cube[7][1]), get_x(cube[6][0]), get_y(cube[6][1]), 2.0, BLUE);

    // Side edges
    draw_line(get_x(cube[0][0]), get_y(cube[0][1]), get_x(cube[4][0]), get_y(cube[4][1]), 2.0, BLUE);
    draw_line(get_x(cube[1][0]), get_y(cube[1][1]), get_x(cube[5][0]), get_y(cube[5][1]), 2.0, BLUE);
    draw_line(get_x(cube[2][0]), get_y(cube[2][1]), get_x(cube[6][0]), get_y(cube[6][1]), 2.0, BLUE);
    draw_line(get_x(cube[3][0]), get_y(cube[3][1]), get_x(cube[7][0]), get_y(cube[7][1]), 2.0, BLUE);
}

#[macroquad::main("SpinningCube")]
async fn main() {

    let cam: [f32; 3] = [0.0, 0.0, 0.0];
    let fov: f32 = 120.0;

     // [x; y; z]
    let mut cube: [[f32; 3]; 8] = [
        //front
        [-0.5, -0.5, 0.5],
        [-0.5,  0.5, 0.5],
        [ 0.5, -0.5, 0.5],
        [ 0.5,  0.5, 0.5],
        //back
        [-0.5, -0.5, 1.5],
        [-0.5,  0.5, 1.5],
        [ 0.5, -0.5, 1.5],
        [ 0.5,  0.5, 1.5],
    ];

    let mut proj_cube: [[f32; 2]; 8] = [[0.0; 2]; 8];

    loop {
        clear_background(RED);

        for index in 0..8 {
            cube[index] = rotate_cube(&mut cube[index], 0.01, 0.02, 0.015);
        }

        for index in 0..8 {
            proj_cube[index] = project_point( cube[index], fov, 1.0, 0.1, 100.0).expect("Point position has z <= 0; change cube parameters");
        }

        draw_cube2d(&proj_cube);

        draw_fps();

        next_frame().await
    }
}
