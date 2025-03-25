use raylib::prelude::*;

const SCREEN_WIDTH: i32 = 1280;
const SCREEN_HEIGHT: i32 = 720;

const TEXTURE_WIDTH: f32 = 128.0;

fn to_isometric(cartesian: Vector2) -> Vector2 {
    Vector2 {
        x: cartesian.x - cartesian.y,
        y: (cartesian.x + cartesian.y) / 2.0,
    }
}

// fn to_cartesian(isometric: Vector2) -> Vector2 {
//     Vector2 {
//         x: (isometric.x / 2.0) + isometric.y,
//         y: isometric.y - (isometric.x / 2.0),
//     }
// }

fn main() {
    let (mut rl, thread) = init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("raylib example - isometric")
        .build();

    let texture = rl.load_texture(&thread, "resources/dirt_E.png").unwrap();
    let texture_width = texture.width();
    let texture_scale = TEXTURE_WIDTH / texture_width as f32;

    let texture2 = rl
        .load_texture(&thread, "resources/chimneyBase_E.png")
        .unwrap();
    let texture2_width = texture.width();
    let texture2_scale = TEXTURE_WIDTH / texture_width as f32;

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);

        d.clear_background(Color::RAYWHITE);

        // Draw first layer
        for y in 0..10 {
            for x in 0..10 {
                let cartesian = Vector2::new(x as f32, y as f32);

                let mut isometric = to_isometric(cartesian);
                isometric *= texture_width as f32 * texture_scale / 2.0;
                isometric.x -= texture_width as f32 * texture_scale / 2.0;
                isometric.x += SCREEN_WIDTH as f32 / 2.0;

                d.draw_texture_ex(&texture, isometric, 0.0, texture_scale, Color::WHITE);
            }
        }

        // Draw second layer
        for y in 0..10 {
            for x in 0..10 {
                let cartesian = Vector2::new(x as f32, y as f32);

                let mut isometric = to_isometric(cartesian);
                isometric *= texture2_width as f32 * texture2_scale / 2.0;
                isometric.x -= texture2_width as f32 * texture2_scale / 2.0;
                isometric.x += SCREEN_WIDTH as f32 / 2.0;

                if y % 3 == 0 && x % 3 == 0 {
                    d.draw_texture_ex(&texture2, isometric, 0.0, texture2_scale, Color::WHITE);
                }
            }
        }
    }
}
