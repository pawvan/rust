
fn main(){
let div_value=div(1,2);
println!("{}",div_value );

say_hello();
    greet("pawan");
    recieved_message("hi");

fn 

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

