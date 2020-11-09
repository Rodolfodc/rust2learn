fn main() {
    println!("Hello, world!");

    let stack_1 = 6;
    let mut stack_2 = 8;

    println!("{}", stack_1);
    stack_2 = stack_1;
    show_value(stack_1);
    show_value(stack_2);

    let heap_1 = Box::new(7); 
    let heap_2 = heap_1;

    println!("{}",heap_1);
    println!("{}",heap_1);
}

fn show_value(var: i32){
    println!("O valor eh {}", var);
}

// fn show_value_heap(var: Box){
//     println!("O valor eh {}", var);
// }