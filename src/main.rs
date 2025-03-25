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

struct IsoTexture {
    texture: Texture2D,
    scale: f32,
}

impl IsoTexture {
    fn new(rl: &mut RaylibHandle, thread: &RaylibThread, path: &str) -> Self {
        let texture = rl.load_texture(thread, path).unwrap();
        let scale = TEXTURE_WIDTH / texture.width() as f32;
        Self { texture, scale }
    }
}

fn draw_layer(
    d: &mut RaylibDrawHandle,
    iso_tex: &IsoTexture,
    condition: impl Fn(i32, i32) -> bool,
) {
    let texture_size = iso_tex.texture.width() as f32 * iso_tex.scale / 2.0;
    let x_offset = SCREEN_WIDTH as f32 / 2.0 - texture_size;

    for y in 0..10 {
        for x in 0..10 {
            if condition(x, y) {
                let cartesian = Vector2::new(x as f32, y as f32);
                let mut isometric = to_isometric(cartesian) * texture_size;
                isometric.x += x_offset;

                d.draw_texture_ex(
                    &iso_tex.texture,
                    isometric,
                    0.0,
                    iso_tex.scale,
                    Color::WHITE,
                );
            }
        }
    }
}

fn main() {
    let (mut rl, thread) = init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("raylib example - isometric")
        .build();

    let dirt_texture = IsoTexture::new(&mut rl, &thread, "resources/dirt_E.png");
    let chimney_texture = IsoTexture::new(&mut rl, &thread, "resources/chimneyBase_E.png");

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);

        // Draw first layer (always drawn)
        draw_layer(&mut d, &dirt_texture, |_, _| true);

        // Draw second layer (only on certain tiles)
        draw_layer(&mut d, &chimney_texture, |x, y| x % 3 == 0 && y % 3 == 0);
    }
}
