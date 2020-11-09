fn main() {
    
    let stack_1 = 7;
    let stack_2 = stack_1;

    show_value(stack_1);
    show_value(stack_2);

    let heap_p1 = Box::new(5);
    let heap_p2 = &heap_p1;

    show_value_heap(&heap_p1);
    show_value_heap(&heap_p2);

    let mut dog = Cachorro {
        nome: String::from("laika"),
        raca: String::from("shinouzer"),
        posicao_atual: 10
    };

    dog.walk();
    println!("{}", dog.walk());
}

fn show_value(val: i32){
    println!("Its value is: {}", val);
}

fn show_value_heap(val: &Box<i32>){
    println!("Its value is: {}", val);
}

pub trait animal {
    fn walk(&mut self) -> String;
}

pub struct Cachorro {
    pub nome: String,
    pub raca: String,
    pub posicao_atual: i32
}

impl animal for Cachorro {
    fn walk(&mut self) -> String {
        self.posicao_atual += 12;
        return format!("Dog walked {} and his position now is: {}", 12, self.posicao_atual);
    }
}