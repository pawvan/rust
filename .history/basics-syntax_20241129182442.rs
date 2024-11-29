 MAX_POINTS :u32 =100_000;

fn main(){
    add(1,2);say_hello("")
}
let x =5;
let mut x =5;
x=10
//type inference is there in rust 
let x=5;
let x:i32 =5;
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
fn add(x:i32,y:i32)->i32{
    return x/y;
}

