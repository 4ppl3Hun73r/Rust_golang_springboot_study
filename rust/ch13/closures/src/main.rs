use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("calculating slowly...");
    thread::sleep(Duration::from_secs(2));
    intensity
}

fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!(
            "Today, do {} pushups!",
            simulated_expensive_calculation(intensity)
        );
        println!(
            "Next, do {} situps!",
            simulated_expensive_calculation(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",
                simulated_expensive_calculation(intensity)
            );
        }
         
    }
}

// 리펙토링. 비싼 함수를 1회만 호출되게
// 단 비싼 함수가 호출이 필요 없는 경우에도 1회가 호출 된다
fn _generate_workout1(intensity: u32, random_number: u32) {
    let expensive_result = simulated_expensive_calculation(intensity);
    if intensity < 25 {
        println!(
            "Today, do {} pushups!", expensive_result
        );
        println!(
            "Next, do {} situps!", expensive_result
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!", expensive_result
            );
        }
         
    }
}

// 최대 1회, 필요없는 경우 호출이 되지 않게 만들고 싶다.
// closure 를 사용했지만 다시 1번 문제인 중복 문제로 돌아 왔다.
fn _generate_workout2(intensity: u32, random_number: u32) {
    let expensive_closure = |num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        intensity
    };

    if intensity < 25 {
        println!(
            "Today, do {} pushups!", expensive_closure(intensity)
        );
        println!(
            "Next, do {} situps!",  expensive_closure(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",  expensive_closure(intensity)
            );
        }
         
    }
}

struct Cacher<T>
    where T: Fn(u32) -> u32
{
    calculation: T,
    value: Option<u32>, 
}
impl<T> Cacher<T>
    where T: Fn(u32) -> u32
{
    fn new(calculation: T) -> Cacher<T> {
        Cacher {
            calculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.calculation)(arg);
                self.value = Some(v);
                v
            },
        }
    }

}

// 최대 1회, 필요없는 경우 호출이 되지 않게 만들고 싶다.
// closure 를 사용하며 함수를 저장하는 구조체 Cacher를 생성해서 1회 호출하게 한다.
fn _generate_workout3(intensity: u32, random_number: u32) {
    let mut expensive_result = Cacher::new(|num| {
        println!("calculating slowly...");
        thread::sleep(Duration::from_secs(2));
        intensity
    });

    if intensity < 25 {
        println!(
            "Today, do {} pushups!", expensive_result.value(intensity)
        );
        println!(
            "Next, do {} situps!",  expensive_result.value(intensity)
        );
    } else {
        if random_number == 3 {
            println!("Take a break today! Remember to stay hydrated!");
        } else {
            println!(
                "Today, run for {} minutes!",  expensive_result.value(intensity)
            );
        }
         
    }
}

fn main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    _generate_workout3(
        simulated_user_specified_value,
        simulated_random_number
    );

    
    let example_closure = |x| x;
    let s = example_closure(String::from("hello"));
    // compile error 이미 string으로 추론이 되어서 number를 넣을 수 없음 let n = example_closure(5);

    let x = 4;
    // x에 접근 가능
    let equal_to_x = |z| z == x;
    let y = 4;

    assert!(equal_to_x(y));

    // 함수로는 x에 접근 불가능 해서 컴파일 오류가 발생한다. 
    // fn equal_to_x(z: i32) -> bool { z == x }
    // assert!(equal_to_x(y));


    // 클로저는 세가지 방식으로 값을 캡쳐 할 수 있다. 
    // 소유권 받기, 불변으로 빌려오기, 가변으로 빌려오기
    // FnOnce => 동일한 변수에서 한번이상 소유권을 가져 올 수 없고. 한번만 호출 가능하다.
    // Fn 환경으로 부터 값을 불변으로 빌려 옴
    // FnMut 값들을 가변으로 빌려옴, 환경 변화가 가능함

    let x = vec![1, 2, 3];
    let equal_to_x  = move |z| z == x;

    // move 를 사용해서 x에 대한 소유권이 클로저 내부로 이동 main에서는 더이상 x를 참조할수 없게 된다. 
    println!("can't use x here: {:?}", x);

    let y = vec![1, 2, 3];
    assert!(equal_to_x(y));

}

/* 모두 동일
 * fn  add_one_v1   (x: u32) -> u32 { x + 1 }
 let add_one_v2 = |x: u32| -> u32 { x + 1 };
 let add_one_v3 = |x|             { x + 1 };
 let add_one_v4 = |x|               x + 1  ;
 */


#[test]
fn call_with_different_values() {
    // clouser 환겨이 캡쳐 되어 있지 않아서 이 테스트는 실패 한다.
    let mut c = Cacher::new(|a| a);

    let v1 = c.value(1);
    let v2 = c.value(2);

    assert_eq!(v2, 2);
}
