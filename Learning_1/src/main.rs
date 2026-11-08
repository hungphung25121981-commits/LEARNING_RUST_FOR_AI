const PI: f32=3.14; // constant variable (cannot be changed)



fn main()
{

let x=5; // read-only variable ( cannot be changed)
let mut y=0; // mutable variable(can be changed))
println!("x is {}",x);
println!("y is {}",y);
y=2;
println!("y is now {}",y);
println!("Gia tri cua PI la {}",PI);


let x=x+1; // shadowing variable (can be changed)
println!("x la {}",x);
let mut y=y*2; 
println!("y la {}",y);
y=y+1;
println!("y la {}",y);
}