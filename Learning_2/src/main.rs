


fn main()
{
	let age: u32=18; // biến này u32 là unsigned (>=0) 32 bit int 
	let num : i32=-100; // biến này là i32 là 32 bit int
	let temperature: f32=36.6; // biến này là f32 là 32 bit float， f64 thì là 64 bit float
	let is_active: bool =true; // biến này là bool 
	let check_bool =false; // biến này tự động nhận dạng kiểu dữ liệu là bool
	let grade: char='A'; // biến này là char
	println!("Age: {}, Num: {}, Temperature: {}, Is Active: {}, Check Bool: {}, Grade: {}",
	age, num, temperature, is_active, check_bool, grade); // print thi se kh xuong dong, println! thi se xuong dong
}