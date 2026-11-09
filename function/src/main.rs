
fn greet(name: &str)
{
    println!("hello {}",name);
}

fn add( a:i32, b:i32)-> i32
{
    a+b // nào mà kh có dấu thì nó là return 
}

fn main() {
    
    greet("hung");
    println!("{}",add(4,3))
}