use std::io;

enum BaiscMathOp {
    Sum,
    Subtract,
    Multiply,
    Division,
    Leave,
    Invalid
}

fn main() {
    println!("Hi, this is the cRUSTlator!");
    
    loop{

        let mut option_choice = String::new();

        println!("please select your option: ");
        println!("[+] Sum ");
        println!("[-] - Subtraction ");
        println!("[*] - Multiply ");
        println!("[/] - Division ");
        println!("q - Leave ");

        io::stdin()
            .read_line(&mut option_choice)
            .expect("Failed to read Option");
            
        let number_option: u32 = match option_choice.trim().parse() {
            Ok(num) => num,
            Err(_) => continue
        };

        let op = identify_operation(option_choice);

        if option_choice == "q" {
            break;
        }

        println!("Please enter the first element to sum");

        io::stdin()
            .read_line(&mut option_choice)
            .expect("Failed to read number");

        let n1 = convert_op_to_int32(option_choice);

        println!("Please enter the second element to sum");

        io::stdin()
            .read_line(&mut option_choice)
            .expect("Failed to read number");

        let n2 = convert_op_to_int32(option_choice);
        
        let res = perform_operation(n1, n2, op);

        println!("The result of the operation is{}", res);
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
        "q" => BaiscMathOp::Leave,
        _ =>   BaiscMathOp::Invalid
    };
}

fn perform_operation(a: i32, b: i32, op: BaiscMathOp) -> i32 {

    return match op {
        BaiscMathOp::Sum        => a + b,
        BaiscMathOp::Subtract   => a - b,
        BaiscMathOp::Multiply   => a * b,
        BaiscMathOp::Division   => a / b,
        BaiscMathOp::Leave      => 0,
        BaiscMathOp::Invalid    => 0
    };
}



// fn read_to_integer