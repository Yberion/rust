// Function are hoisted

pub fn test() -> () {
    println!("//////////////// ======================= Functions");

    tell_height(180);
    human_id("Brandon", 31, 1.80);

    const A: u32 = {
        const B: u32 = 5;
        const C: u32 = 10;
        B + C
    };

    println!("Results are {}, {}", A, add(6, 6));
    println!("Your BMI is {:.2}", calculate_bmi(71.0, 1.80));
}

fn tell_height(height: u32) -> () {
    println!("Your height is {} cm.", height);
}

fn human_id(name: &str, age: u8, height: f32) -> () {
    println!("My name is {:?}, I am {} years old and my height is {:.2} m.", name, age, height);
}

fn add(a: u32, b: u32) -> u32 {
    a + b
}

// Body Mass Index (BMI) : weight(kg) / height(m)^2
fn calculate_bmi(weight_in_kg: f32, height_in_meter: f32) -> f32 {
    weight_in_kg / (height_in_meter * height_in_meter)
}
