use std::io;

enum BaiscMathOp {
    Sum,
    Subtract,
    Multiply,
    Division,
    Invalid
}

fn main() {
    println!("Hi, this is the cRUSTlator!");
    
    loop{

        let mut option_choice = String::new();

        println!("please select your option: ");
        println!("1 - Sum ");
        println!("2 - Subtraction ");
        println!("3 - Multiply ");
        println!("4 - Division ");
        println!("5 - Leave ");

        io::stdin()
            .read_line(&mut option_choice)
            .expect("Failed to read Option");
        
        let number_option: u32 = match option_choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        match number_option {
            1 => println!("The Value of the sum is "),
            2 => println!("The Value of the subtraction is "),
            5 => break,
            _ => println!("please, select a valid option."),
        };
    };

    println!("Bye my friend!");
}

fn convert_op_to_int32(num: String) -> i32 {
    return num.trim().parse::<i32>().unwrap();
}

fn identify_operation(op: String) -> BaiscMathOp {
    return match op.trim() {
        "+" => BaiscMathOp::Sum,
        "-" => BaiscMathOp::Subtract,
        "*" => BaiscMathOp::Multiply,
        "/" => BaiscMathOp::Division,
        _ =>   BaiscMathOp::Invalid
    };
}

fn perform_operation(a: i32, b: i32, op: BaiscMathOp) -> i32 {

    return match op {
        BaiscMathOp::Sum        => a + b,
        BaiscMathOp::Subtract   => a - b,
        BaiscMathOp::Multiply   => a * b,
        BaiscMathOp::Division   => a / b,
        BaiscMathOp::Invalid    => 0
    };
}

// fn read_to_integer