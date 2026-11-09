use std::io::{self,Write};
fn main() {
    let x=Nhap();
    if is_Chan(x)==true {println!("x la so chan");}
    else 
    {
        println!("x la so le");
    }
    Loop();
}




fn is_Chan(x: i32)->bool 
{
     if x%2==0
    {
        true 
    }
    else 
    {
        false 
    }
    
}
fn Nhap()->i32
{
    print!("Nhap 1 so nguyen: ");
    io::stdout().flush().unwrap();
    let mut input= String::new();
    io::stdin().read_line(&mut input).unwrap();
    let x: i32=input.trim().parse().unwrap();
    return x; 
}

fn Loop()
{
let mut counter= 0;
let result = loop
{
    counter +=1;
    if counter==5 {break counter *2}
};
println!("{}",result); 

let mut n=0;
while n!=5
{
    n+=1;
    println!("{}",n);
}
for i in 1..5 
{
    println!("{}",i);
}


}
