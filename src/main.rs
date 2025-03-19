use raylib::prelude::*;

const SCREEN_WIDTH: i32 = 1280;
const SCREEN_HALF_WIDTH: i32 = SCREEN_WIDTH / 2;
const SCREEN_HEIGHT: i32 = 720;

const TEX_SCALE: f32 = 4.0;

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

    let tile = rl.load_texture(&thread, "resources/iso_tile.png").unwrap();
    let tile_width = tile.width as f32 * TEX_SCALE;

    let mut u = 0.0;
    while !rl.window_should_close() {
        let mut mouse_screen_position = rl.get_mouse_position();
        mouse_screen_position.x -= SCREEN_HALF_WIDTH as f32;

        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::RAYWHITE);

        let mouse_grid_position =
            isometric_to_cartesian(mouse_screen_position) / (tile_width / 2.0);

        for i in 0..10 {
            for j in 0..10 {
                let cartesian = Vector2::new(i as f32, j as f32);
                let mut isometric = cartesian_to_isometric(cartesian) * (tile_width / 2.0);
                isometric.x -= tile_width / 2.0;
                isometric.x += SCREEN_HALF_WIDTH as f32;

                // isometric.y += ((i as f32 + u + 1.0) / 2.0).sin() * 32.0;
                // isometric.y += ((j as f32 + u + 2.0) / 1.0).sin() * 32.0;

                if (i, j) == (mouse_grid_position.x as i32, mouse_grid_position.y as i32) {
                    isometric.y -= 16.0;
                }

                d.draw_texture_ex(&tile, isometric, 0.0, TEX_SCALE, Color::RAYWHITE);
            }
        }
        u += 0.001;
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
