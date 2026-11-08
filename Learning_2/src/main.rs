


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
	


	// tuple
	let person: (&str, u32, f32)=("Hung",18, 1.74); // tuple là 1 kiểu dữ liệu có thể chứa nhiều giá trị khác nhau, nhưng không thể thay đổi giá trị của nó
	println!("Name: {}, Age: {}, Height(meter): {}",
	person.0,person.1,person.2); // truy cập giá trị của tuple bằng cách sử dụng chỉ số của nó
	//person.1=19;  thay đổi giá trị của tuple bằng cách sử dụng chỉ số của nó, nhưng tuple là immutable nên sẽ báo lỗi

	let mut person_2:(&str,u16,f32)=("han",18,1.74); 
	println!("Name: {}, Age: {}, Height(meter): {}",person_2.0,person_2.1,person_2.2); 
	person_2.1=19;
	println!("Updated Age: {}", person_2.1);


	//array 
	let scores: [i16; 5]=[90,80,70,60,50]; // array là 1 kiểu dữ liệu có thể chứa nhiều giá trị giống nhau, nhưng không thể thay đổi giá trị của nó]
	println!("scores one {}",scores[0]); // truy cập giá trị của array bằng cách sử dụng chỉ số của nó
	println!("scores: {:?}",scores); //{:?} in full 

	let mut scores_2 :[&str;5]=["helu","my","name","is","Hung"];
	scores_2[0]="hola";
	println!("{:?}",scores_2);


	// vector
	let mut numbers= vec![1,2,3];
	println!("{:?}",numbers);
	numbers.push(4);
	println!("numbers vua dc them: {:?}",numbers);
	numbers.pop();
	println!("{:?}",numbers);
	let x= numbers.get(0);
	// println!("x la {}: ",x);  loi cach no tra ve option
	println!(" x la {:?}",x);
	println!("x la {}",x.unwrap());


	let mut v= Vec::with_capacity(5);
	for num in 1..5
	{
		v.push(num);
	}
	v.insert(2,100);
	
	println!("so luong phan tu: {}",v.len());
	println!("{:?}",v);
}

