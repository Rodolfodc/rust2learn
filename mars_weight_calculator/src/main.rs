use std::io;

fn main() {
    println!("---------------------------------------");
    println!("## Welcome to mars weight calculator ##");
    println!("---------------------------------------");

    let mut weight = String::new();

    println!("Enter your earth weight (Kg)");

    io::stdin()
        .read_line(&mut weight)
        .expect("Failed to read weight");
    
    println!("Your Earth weight {}", weight);

    let weight: f32 = weight.trim().parse().unwrap();
    dbg!(find_weight_on_mars(weight));
}

fn find_weight_on_mars(_weight: f32) -> f32 {
    return (_weight / 9.81) * 3.711;
}

/*
*      Ownership rules in RUST
* 1. Each value in Rust is owned by a variable.
*
* 2. When the owner goes out of scope, the value will be deallocated
*
* 3. There can only be ONE owner at a time.
- If a complex type variable (like String) is declared by name var_a and then, a second variable var_b is created, pointing to var_a, so, 
Rust will invalidate var_a and the owner will be var_b. This is important to avoid memory deallocation problems.
- When its a primitive type variable, Rust compiler just copy the value of the first to the second one, because they are not pointers.
- Creating a pointer to the var_a, doesnt transfer the ownership to the new pointer, it only happens when you try to declare a new variable to complex type and atribute 
to its value the other variable (without operator &);
*/