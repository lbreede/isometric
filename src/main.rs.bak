use raylib::prelude::*;

const SCREEN_WIDTH: i32 = 1280;
const SCREEN_HALF_WIDTH: i32 = SCREEN_WIDTH / 2;
const SCREEN_HEIGHT: i32 = 720;

const TEX_SIZE: i32 = 32;
const TEX_SCALE: f32 = 128.0 / TEX_SIZE as f32;

fn cartesian_to_isometric(cartesian: Vector2) -> Vector2 {
    Vector2 {
        x: cartesian.x - cartesian.y,
        y: (cartesian.x + cartesian.y) / 2.0,
    }
}

fn isometric_to_cartesian(isometric: Vector2) -> Vector2 {
    Vector2 {
        x: (isometric.x / 2.0) + isometric.y,
        y: isometric.y - (isometric.x / 2.0),
    }
}

fn main() {
    let (mut rl, thread) = init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("raylib example - isometric")
        .build();

    let filename = format!("resources/iso{}.png", TEX_SIZE);
    let tile = rl.load_texture(&thread, &filename).unwrap();
    let tile_width = tile.width as f32 * TEX_SCALE;
    let tile_half_width = tile_width / 2.0;

    rl.set_target_fps(60);

    let mut destroyed: Vec<(i32, i32)> = Vec::new();

    let mut u = 0.0;
    let mut amplitude = 16.0;
    while !rl.window_should_close() {
        let mut mouse_screen_position = rl.get_mouse_position();
        mouse_screen_position.x -= SCREEN_HALF_WIDTH as f32;

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::RAYWHITE);

        let mouse_grid_position = isometric_to_cartesian(mouse_screen_position) / tile_half_width;
        // NOTE: Make sure it's _floored_ and not rounded since the origin of the grid is in the
        // top left corner of the screen.
        let floored_mouse_grid_position = (
            mouse_grid_position.x.floor() as i32,
            mouse_grid_position.y.floor() as i32,
        );

        // Destroy tiles
        if d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_LEFT) {
            destroyed.push(floored_mouse_grid_position);
        }

        // Restore tiles
        if d.is_mouse_button_down(MouseButton::MOUSE_BUTTON_RIGHT) {
            destroyed.retain(|&x| x != floored_mouse_grid_position);
        }

        amplitude = (amplitude - d.get_mouse_wheel_move()).clamp(0.0, 64.0);

        for i in -8..16 {
            for j in -8..16 {
                if destroyed.contains(&(i, j)) {
                    continue;
                }

                let cartesian = Vector2::new(i as f32, j as f32);
                let mut isometric = cartesian_to_isometric(cartesian) * tile_half_width;
                isometric.x -= tile_half_width;
                isometric.x += SCREEN_HALF_WIDTH as f32;

                isometric.y += ((i as f32 + u + 1.0) / 2.0).sin() * amplitude;

                let mut tint = Color::LIGHTGRAY;
                if (i, j) == floored_mouse_grid_position {
                    isometric.y -= 32.0;
                    tint = Color::WHITE;
                }

                // Snap to pixel
                isometric.y /= TEX_SCALE;
                isometric.y = isometric.y.round();
                isometric.y *= TEX_SCALE;

                d.draw_texture_ex(&tile, isometric, 0.0, TEX_SCALE, tint);
            }
        }
        u += 0.1;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cartesian_to_isometric() {
        let cartesian = Vector2 { x: 3.0, y: 1.0 };
        let isometric = cartesian_to_isometric(cartesian);
        assert_eq!(isometric, Vector2 { x: 2.0, y: 2.0 });

        let cartesian = Vector2::new(3.5, 4.5);
        let isometric = cartesian_to_isometric(cartesian);
        assert_eq!(isometric, Vector2::new(-1.0, 4.0));

        let cartesian = Vector2::new(3.75, 4.75);
        let isometric = cartesian_to_isometric(cartesian);
        assert_eq!(isometric, Vector2::new(-1.0, 4.25));
    }

    #[test]
    fn test_isometric_to_cartesian() {
        let isometric = Vector2 { x: 2.0, y: 2.0 };
        let cartesian = isometric_to_cartesian(isometric);
        assert_eq!(cartesian, Vector2 { x: 3.0, y: 1.0 });

        let isometric = Vector2::new(-1.0, 4.0);
        let cartesian = isometric_to_cartesian(isometric);
        assert_eq!(cartesian, Vector2::new(3.5, 4.5));

        let isometric = Vector2::new(-1.0, 4.25);
        let cartesian = isometric_to_cartesian(isometric);
        assert_eq!(cartesian, Vector2::new(3.75, 4.75));
    }
}
