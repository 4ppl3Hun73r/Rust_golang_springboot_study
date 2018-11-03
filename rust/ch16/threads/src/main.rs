use std::thread;
use std::time::Duration;

fn main() {
    let handle = thread::spawn(move || {
        for i in 1..10 {
            println!("hi number {} from the spawned thread", i);
            thread::sleep(Duration::from_millis(1));
        }
    });

    for i in 1..5 {
        println!("hi number {} from the main thread", i);
        thread::sleep(Duration::from_millis(3));
    }

    handle.join().unwrap();

    /* 컴파일 에러
    main thread에서 생성한 v 에 대한 값을 thread에서 참조 불가
    let v = vec![1, 2, 3];
    let handle = thread::spawn(|| {
        println!("Here's a vection {:?}", v);
    });
    drop(v);

    handle.join().unwrap();
    */

    let v = vec![1, 2, 3];
    let handle = thread::spawn(move || {
        println!("Here's a vection {:?}", v);
    });
    // compile error. move를 통해서 v 가 thraed내부로 소유권이 이전 되었기 때문에
    // 이제 main thread에서 drop을 시킬 수 없다. 
    //drop(v);

    handle.join().unwrap();


}
