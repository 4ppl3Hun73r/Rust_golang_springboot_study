enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

fn main() {
    let v: Vec<i32> = Vec::new();
    // compile error need mut v.push(1);

    let v = vec![1, 2, 3];
    // compile error need mut v.push(4);

    let mut v = Vec::new();
    v.push(1);

    let v = vec![1, 2, 3, 4, 5];
    let third: &i32 = &v[2];
    let third: Option<&i32> = v.get(2);

    // panic let does_not_exist = &v[100];
    let does_not_exist = v.get(100);

    let mut v = vec![1, 2, 3, 4, 5];
    let first = &v[0];
    // compile error v.push(6);

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
    let mut v = vec![100, 32, 57];
    for i in &mut v {
        *i += 50; // 가변 참조자의 값을 얻기 위해 * 사용
    }
    for i in &v {
        println!("{}", i);
    }

    let row = vec![
        SpreadsheetCell::Int(3),
        SpreadsheetCell::Text(String::from("blue")),
        SpreadsheetCell::Float(10.12),
    ];
} // drop v + inner values
