extern crate rand; // 외부 참조 라이브러리를 참조에 넣는다.

use std::io;
use std::cmp::Ordering;
use rand::Rng; // Rng trait defines methods that random number generators impletment
	// extern 과 use의 차이는? 
	// std는 extern 할 필요가 없나? extern 할 필요 없는 라이브러리의 종류는?
	// dependency 정의한것만 extern 하면 될것 같긴 한데....

fn main() {
	println!("Guess the number!");

	let secret_number = rand::thread_rng().gen_range(1, 101);
	println!("The secret number is: {}", secret_number);


	loop {
		println!("Please input your guess.");

		let mut guess = String::new();
		// let guess = String::new(); error immutable

		io::stdin().read_line(&mut guess) // ypes into statndard input an place that into a string
				// & indicates is a reference 
			.expect("Failed to read line"); // Fail ... 

		// io::stdin().read_line(&mut guess); // Don't use 'Result' compile warn!

		println!("You guessed: {}", guess);

//		let guess: u32 = guess.trim().parse() // Shadowing 을 통한 동일 변수명을 형식만 변경해서 사용
//			.expect("Plaease type a number!"); // parse는 Result 반환, expect를 통해서 Err발생시 처리

		let guess: u32 = match guess.trim().parse() {
			Ok(num) => num,
			Err(_) => continue,	// Error 가 발생핻 loop가 정상적으로 동작하게 수정
						// _ catchall value
		};


		match guess.cmp(&secret_number) {
			Ordering::Less => println!("Too small!"),
			Ordering::Greater => println!("Too big!"),
			Ordering::Equal => {
				println!("You win!");
				break;
			}
		}
	}

	main2();
}

fn main2() {
	let foo = 5; // immutable value
	let mut bar = 10; // mutable 

	println!("foo : {}", foo);
	println!("bar : {}", bar);
	// foo = 10; // error
	bar = 11;

	println!("bar : {}", bar);

}

