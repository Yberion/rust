mod a_primitive_data_types;
mod b_compound_data_types;
mod c_functions;
mod d_ownership;
mod e_borrowing_and_references;

fn main() -> () {
    println!("==================================================================================================================================================\n");

    a_primitive_data_types::main::test();
    println!("----------------------------------------------------------\n");
    b_compound_data_types::main::test();
    println!("----------------------------------------------------------\n");
    c_functions::main::test();
    println!("----------------------------------------------------------\n");
    d_ownership::main::test();
    println!("----------------------------------------------------------\n");
    e_borrowing_and_references::main::test();
    println!("----------------------------------------------------------\n");

    println!();
}
