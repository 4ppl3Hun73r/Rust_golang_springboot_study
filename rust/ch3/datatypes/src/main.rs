fn main() {
//	let guess = "42".parse() .expect("Not a number!");

	let guess: u32 = "42".parse()
		.expect("Not a number!");

	let x = 2.0; // f64

	let y: f32 = 3.0; // f32

	println!("x is {}, y is {}, guess is, {}", x, y, guess);

	let sum = 4 + 10;

	let difference = 95.4 - 4.3;

	let product = 4 * 30;

	let quotient = 56.7 / 32.2;

	let remainder = 43 % 5;

	let t = true;
	let f: bool = false;

        let c = 'z';
        let z = 'ℤ';
        let heart_eyed_cat = '😻';

        println!("{}, {}, {}", c, z, heart_eyed_cat);

        let tup: (i32, f64, u8) = (500, 6.4, 1);
        let (x, y, z) = tup;
        println!("The value of y is:{}", y);
// 이건 안되네...        println!("The value of tup is:{}", tup);

        let x = tup;
        let five_hundred = x.0;
        let six_point_four = x.1;
        let one = x.2;

        println!("{}, {}, {}", five_hundred, six_point_four, one);

        let a = [1, 2, 3, 4, 5];
        let first = a[0];
        let second = a[1];

        // compile 레벨에서 잡힘 let element = a[10];
        let index = 10;
        let element = a[index];

// 이것도 안되는군..        println!("{}", a);
        println!("{}, {}, {}", first, second, element);


}
