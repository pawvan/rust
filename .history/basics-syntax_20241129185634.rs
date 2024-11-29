
fn main(){
let div_value=div(1,2);
println!("{}",div_value );

say_hello();
    greet("pawan");
    recieved_message("hi");

check_number(8);
compare_numbers(8,9);
}
fn compare_numbers(x:i3,y:i32)->i32{
    if x>y{
        x
    }
    else{
        y
    }
}
fn count_to_ten()
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

