enum Direction{
    Up,
    Down,
    Left,
    Right,
}
fn main(){
    let x = String::from("hello world");
    println!("{}",x);  
let div_value=div(1,2);
println!("{}",div_value );
let 
say_hello();
let s1= String::from("hello world");
let s_ref=&s1;
let s_ref2=&s1;
println!("{}",s_ref2);
println!("{}",s_ref);
    greet("pawan");
    recieved_message("hi");
check_number(8);
compare_numbers(8,9);
count_to_ten();
process_input(9);
count_down();
print_range();
print_vector();
number_to_string(1);
check_range(15);
move_player(Direction::Up);
}
fn move_player(direction:Direction){
    match direction{
        Direction::Up =>println!("moving up"),
        Direction::Down=>println!("moving down"),
    Direction::Left=>println!("moving  left"),
        Direction::Right =>println!("moving right"),
    }
}
fn compare_numbers(x:i32,y:i32)->i32{
    if x>y{
        x
    }
    else{
        y
    }
}
fn check_range(x:i32)->&'static str{
    match x{
        1..=10=>"between 1 and 10",
       11..=20=>"between 11 and 20",
        _=>"out of range"
    }
}
fn print_range(){
    for i in 1..6{
        println!("{}",i);
    }
    for i in 9..10{
        println!("{}",i);
    
        }
}
fn count_down(){
    let mut count =10;
    while count >0{
        println!("{}",count);
        count-=1;
    }
    println!("liftoff");
}
fn print_vector(){
    let numbers  =vec![10,20,30,40,50];
    for num in numbers{
        println!("{}",num)
    }
}
fn number_to_string(x:i32)->String{
    match x{
        1 => String::from("one"),
        2 => String::from("two"),
        3=> String::from("three"),
        _=>String::from("unknown"),
    }
}
fn count_to_ten(){
    let mut count =1;
    loop{
        if count >10{
            break;
        }
        println!("{}",count);
        count+=1;
    }
}
fn check_number(x:i32){
    if x>0{
        println!("positive number");
    }
    else if x <0{
        println!("negative number");
    }
    else{
        println!("zero");
    }
}
//type inference is there in rust 
//constants
//default cant be changed
fn say_hello(){
    println!("hello rust!");
}
fn greet(name:&str){
    println!("hello{}",name)
}
fn recieved_message(message:&str){
    println!("recieved message {}",message);
}
fn div(x:i32,y:i32)->i32{
    return x/y;
}
fn process_input(input:i32){
    match input{
        1..=10=>{
            println!("Input is in the range 1-10");
 
        
        if input%2 == 0{
            println!("even number");
        }
        else{
            println!("odd number");
        }
    }
        _=>{
            println!("input is out of range");
        }
    }
}