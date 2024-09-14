//#[expect(dead_code)]
#[allow(dead_code, unused_variables)]
#[allow(clippy::vec_init_then_push)]
#[allow(clippy::useless_vec)]
pub fn test() {
    // ------------------------------------------------------------------
    // 8.1 Storing Lists of Values with Vectors
    // ------------------------------------------------------------------

    let vector: Vec<u8> = Vec::new();
    let vector_with_initial_value: Vec<u8> = vec![1, 2, 3];

    let mut vector2: Vec<u8> = Vec::new();

    vector2.push(6);
    vector2.push(8);

    let val1: &u8 = &vector2[1];
    
    println!("{val1}");

    #[allow(clippy::manual_unwrap_or)]
    let val2 = match vector2.get(5) {
        Some(val) => val,
        None => &0
    };

    let val3 = vector2.get(5).unwrap_or(&0);
    let val4 = vector2.get(1).unwrap_or(&0);

    println!("{val2}");
    println!("{val3}");
    println!("{val4}");

    for val in &vector2 {
        println!("{val}");
    }

    enum SpreadsheetCell {
        Int(i32),
        Float(f32),
        Text(String)
    }

    let vector3 = vec![
        SpreadsheetCell::Int(5),
        SpreadsheetCell::Float(3.148),
        SpreadsheetCell::Text(String::from("Test"))
    ];
}
