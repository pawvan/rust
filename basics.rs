let x=5;
let mut  y=5;

let a:i32 =42;
let b:f64 =3.14;
let c:bool = true;

let d:char ='a'

let tuple :(i32,f64) = (1,3.14);
let array :[i32,3] =[1,2,3]
let tuple1:(i32,f64) =(1,3.14);
let array1 :[i32,1] =[1];

let  number =5;
if number <10{
    printn!("less ahtn 10");
}
else{
    println!("10 or more");
}
let mut age =10;
if age<10{
    println!("can voote")

}

else {
    println!("cant vote");
}

//loop
let mut count =0;
loop{
    if count>=5{
        break;
    }
    count+=1;
}
let mut age=18;
loop{
    if age>=28{
        break;
    }
    age+=1;
}
while count >0{
    println!("{}",count);
    count-=1;
}
//while loop


for i in 0..5{
    println!("{}",i);
}
//of rllop

fn add(x:i32,y:32) ->i32{
    x+y;
}
fn main(){
    let reuslt add(5,10);
    println("the sum is :{}",reuslt);
}

k
