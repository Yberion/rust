fn main() {
    println!("//////////////// ======================= Compound data types");

    // Array
    const A: [i32; 5] = [1, 2, 3, 4, 5];
    const FRUITS: [&str; 3] = ["Apple", "Banana", "Orange"];

    println!("array: {:?} {:?}, first fruit: {}", A, FRUITS, FRUITS[0]);

    // Tuple
    let human: (String, &str, i32, bool) = ("Alice".to_string(), "Anderson", 30, true);

    println!("tuple: {:?}", human);

    // Slice
    const NUMBER_SLICE: &[i32] = &[1, 2, 3, 4, 5];
    const ANIMAL_SLICE: &[&str] = &["Cat", "Dog"];
    let book_slice: &[&String] = &[&"Harry Potter".to_string(), &"IT".to_string()];

    println!(
        "number slice: {:?}, animal slice: {:?}, book slice: {:?}",
        NUMBER_SLICE, ANIMAL_SLICE, book_slice
    );

    // String / String slice (&str)

    // String
    // growable, mutable, owned string type, on the heap
    let mut johnny_john: String = String::from("Hell, ");
    johnny_john.push_str("yeah!");

    println!("Johnny John says: {}", johnny_john);

    // String slice (&str)
    // immutable, on the stack
    let hello_world: String = String::from("Hello, world!");
    let hello_world_slice: &str = &hello_world;

    println!("hello slice: {}", hello_world_slice);
}
