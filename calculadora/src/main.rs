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

        let mut splitted: Vec<&str> = option_choice.trim().split(|c| c == '+' || c == '-' || c == '*' || c == '/').collect();

        if option_choice.contains(|c| c == '+' || c == '-' || c == '*' || c == '/'){
            println!("OK");
        }


        println!("{}", splitted.len());

        if option_choice.trim() == "q"{
            break;
        }
    
            

        let op = identify_operation(&option_choice);

        let mut n1 = String::new();

        println!("Please enter the first element to the operation");
        io::stdin()
            .read_line(&mut n1)
            .expect("Failed to read number");

        let n1: i32 = match n1.trim().parse() {
                Ok(num) => num,
                Err(_) => panic!("erro de leitura")
            };

        println!("Please enter the second element to sum");

        let mut n2= String::new();

        io::stdin()
            .read_line(&mut n2)
            .expect("Failed to read number");

        let n2: i32 = match n2.trim().parse() {
            Ok(num) => num,
            Err(_) => panic!("erro de leitura")
        };
        
        let res = match perform_operation(n1, n2, op) {
            Some(num) => num,
            None => std::process::exit(1)
        };

        println!("The result of the operation is {}", res);
    };

    println!("Bye my friend!");
}


fn identify_operation(op: &String) -> BaiscMathOp {
    return match op.trim() {
        "+" => BaiscMathOp::Sum,
        "-" => BaiscMathOp::Subtract,
        "*" => BaiscMathOp::Multiply,
        "/" => BaiscMathOp::Division,
        "q" => BaiscMathOp::Leave,
        _ =>   BaiscMathOp::Invalid
    };
}

fn perform_operation(a: i32, b: i32, op: BaiscMathOp) -> Option<i32> {

    return match op {
        BaiscMathOp::Sum        => Some(a + b),
        BaiscMathOp::Subtract   => Some(a - b),
        BaiscMathOp::Multiply   => Some(a * b),
        BaiscMathOp::Division   => Some(a / b),
        BaiscMathOp::Leave      => None,
        BaiscMathOp::Invalid    => Some(0)
    };
}



// fn read_to_integer
#[test]

fn testperform_operation(){
    assert_eq!(perform_operation(1,2, BaiscMathOp::Sum), Some(3));
}