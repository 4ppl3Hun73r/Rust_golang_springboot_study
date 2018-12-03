use std::slice;

static HELLO_WORLD: &str = "Hello, world!";
static mut COUNTER: u32 = 0;
fn add_to_count(inc: u32) {
    unsafe {
        COUNTER += inc;
    }
}

fn main() {
    let mut num = 5;

    let r1 = &num as *const i32;
    let r2 = &mut num as *mut i32;

    let address = 0x012345usize;
    let r = address as *const i32;

    /* compile error unsafe block을 사용하지 않음
    println!("{}", *r1);
    println!("{}", *r2);
    */
    unsafe {
        // 포인터 생성은 상과없지만 사용하려면 unsafe로 감싸야만 함
        println!("{}", *r1);
        println!("{}", *r2);
    }

    // unsafe 함수를 사용하려면 unsafe로 감싸야 함 dangerous();
    unsafe {
        dangerous();
    }


    // 표준라이브러리의 split_at_mut는 unsafe를 사용하지 않고는 구현 불가능
    let mut v = vec![1, 2, 3, 4, 5, 6];

    let r = &mut v[..];

    let (a, b) = r.split_at_mut(3);

    assert_eq!(a, &mut [1, 2, 3]);
    assert_eq!(b, &mut [4, 5, 6]);

    // 불변 정적 변수의 접근은 안점함
    println!("name is: {}", HELLO_WORLD);

    add_to_count(3);

    unsafe {
        // 가변 정적 변수의 접근은 불안정함
        println!("COUNTER: {}", COUNTER);
    }
}

// unsafe를 함수 앞에 선언해 줌으로써 이 함수를 사용할때 개발자가 충분히 상황을 인지 했음을 보장함
unsafe fn dangerous() {
    // unsafe 함수 안에서는 자동으론 unsafe{} 로 감싸져 있다고 생각하면 됨

}

fn split_at_mut(slice: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) {
    let len = slice.len();
    let ptr = slice.as_mut_ptr();

    assert!(mid <= len);

    // 일반 rust의 컴파일러는 두번 빌리는 걸로 파악한다.
    // 개발자는 겹치지 않게 데이터를 분리 했음을 알지만 컴파일러는 알지 못한다.
    //(&mut slice[..mid],
    // &mut slice[mid..])

    unsafe {
        // slice의 포인터를 가지고 2개로 쪼갬
        // 이 코드는 정확한 길이를 측정하지 않는다면 위험하지만
        // 위에서 길이를 측정 하기 때문에 믿을 만 하게 된다.
        (slice::from_raw_parts_mut(ptr, mid),
         slice::from_raw_parts_mut(ptr.offset(mid as isize), len - mid))
    }
}

// trait 내부에 불변성을 유지할 수 없다면
// unsafe를 붙여 줘야함
unsafe trait Foo {
    // methods go here
}

unsafe impl Foo for i32 {
    // method implementations go here
}