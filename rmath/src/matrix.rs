
/// `Matrix` type (OpenGL style 4x4 - right handed, column major)
#[derive(Copy, Clone, Debug)]
pub struct Matrix {
    m0: f64,
    m4: f64,
    m8: f64,
    m12: f64,
    // Matrix first row (4 components)

    m1: f64,
    m5: f64,
    m9: f64,
    m13: f64,
    // Matrix second row (4 components)

    m2: f64,
    m6: f64,
    m10: f64,
    m14: f64,
    // Matrix third row (4 components)

    m3: f64,
    m7: f64,
    m11: f64,
    m15: f64, // Matrix fourth row (4 components)
}

