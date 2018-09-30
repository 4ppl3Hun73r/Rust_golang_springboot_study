fn main() {

	let x = 5;
	let mut x = x + 1;
	let x = x * 2;

	println!("The value of x is: {}", x);
}


fn _main2() {

	const MAX_POINTS: u32 = 100_000;

	let mut x = 5;
	println!("The value of x is: {}", x);
	x = 6;
	println!("The value of x is: {}", x);

	println!("The value of MAX_POINTS is: {}", MAX_POINTS);
}
