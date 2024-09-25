fn main() {
    println!("//////////////// ======================= Primitive data types");

    // Signed integer
    // i8 i16 i32 i64 i128
    const A: i8 = -125;

    // Unsigned integer
    // u8 u16 u32 u64 u128
    const B: u8 = 200;

    // Float
    // f32 f64
    const C: f64 = 3.14;

    // Boolean
    // bool
    const D: bool = true;

    // Char
    // char
    const E: char = 'y';

    println!("i8: {}, u8: {}, f64: {}, bool: {}, char: {}", A, B, C, D, E);
}
