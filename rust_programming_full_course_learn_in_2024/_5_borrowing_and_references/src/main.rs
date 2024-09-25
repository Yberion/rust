// borrowing = emprunter

fn main() {
    let mut x: i32 = 5;
    let r: &mut i32 = &mut x;

    *r = 6;

    println!("x {}", x);
    //println!("x {}", r);
}
