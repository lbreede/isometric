use rand::rngs::SmallRng;
use rand::{Rng, SeedableRng};
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
    /// Load a texture and calculate the scale factor
    fn new(rl: &mut RaylibHandle, thread: &RaylibThread, path: &str) -> Self {
        let texture = rl.load_texture(thread, path).unwrap();
        let scale = TEXTURE_WIDTH / texture.width() as f32;
        Self { texture, scale }
    }
}

/// Generates a pseudo-random but deterministic index based on (x, y)
fn get_tile_index(x: i32, y: i32, num_variants: usize) -> usize {
    let seed = (x * 73856093) ^ (y * 19349663); // Hash function
    let mut rng = SmallRng::seed_from_u64(seed as u64);
    rng.random_range(0..num_variants)
}

fn draw_layer_fixed(d: &mut RaylibDrawHandle, textures: &[IsoTexture]) {
    let texture_size = textures[0].texture.width() as f32 * textures[0].scale / 2.0;
    let x_offset = SCREEN_WIDTH as f32 / 2.0 - texture_size;

    for y in 0..10 {
        for x in 0..10 {
            let cartesian = Vector2::new(x as f32, y as f32);
            let mut isometric = to_isometric(cartesian) * texture_size;
            isometric.x += x_offset;

            let index = get_tile_index(x, y, textures.len()); // Fixed texture index per (x, y)
            let selected_texture = &textures[index];

            d.draw_texture_ex(
                &selected_texture.texture,
                isometric,
                0.0,
                selected_texture.scale,
                Color::WHITE,
            );
        }
    }
}

fn main() {
    let (mut rl, thread) = init()
        .size(SCREEN_WIDTH, SCREEN_HEIGHT)
        .title("raylib example - isometric")
        .build();

    let dirt_textures = vec![
        IsoTexture::new(&mut rl, &thread, "resources/dirt_N.png"),
        IsoTexture::new(&mut rl, &thread, "resources/dirt_E.png"),
        IsoTexture::new(&mut rl, &thread, "resources/dirt_S.png"),
        IsoTexture::new(&mut rl, &thread, "resources/dirt_W.png"),
    ];

    rl.set_target_fps(60);

    while !rl.window_should_close() {
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::RAYWHITE);

        // Draw first layer with deterministic random textures
        draw_layer_fixed(&mut d, &dirt_textures);
    }
}
