use raylib::prelude::*;

// These are the four numbers that define the transform, i hat and j hat
const I_X: f32 = 1.0;
const I_Y: f32 = 0.5;
const J_X: f32 = -1.0;
const J_Y: f32 = 0.5;
//
// Sprite size
const W: f32 = 32.0;
const H: f32 = 32.0;
fn to_screen_coordinate(tile: Vector2) -> Vector2 {
    Vector2 {
        x: tile.x * I_X * 0.5 * W + tile.y * J_X * 0.5 * W,
        y: tile.x * I_Y * 0.5 * H + tile.y * J_Y * 0.5 * H,
    }
}

struct Matrix2x2 {
    a: f32,
    b: f32,
    c: f32,
    d: f32,
}

fn invert_matrix(m: Matrix2x2) -> Matrix2x2 {
    let det = 1.0 / (m.a * m.d - m.b * m.c);
    Matrix2x2 {
        a: m.d * det,
        b: -m.b * det,
        c: -m.c * det,
        d: m.a * det,
    }
}

fn to_grid_coordinate(screen: Vector2) -> Vector2 {
    let m = Matrix2x2 {
        a: I_X * 0.5 * W,
        b: J_X * 0.5 * W,
        c: I_Y * 0.5 * H,
        d: J_Y * 0.5 * H,
    };
    let m_inv = invert_matrix(m);

    let x = screen.x * m_inv.a + screen.y * m_inv.b;
    let y = screen.x * m_inv.c + screen.y * m_inv.d;

    Vector2 { x, y }
}
