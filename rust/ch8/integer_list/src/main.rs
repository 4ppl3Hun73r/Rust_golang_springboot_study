use std::collections::HashMap;

fn main() {

    let integer_list = vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 9, 8, 7, 6, 5, 6, 7, 8, 9, 10];

    println!("avarage : {}", avarage(&integer_list));
    println!("median : {}", median(&integer_list));
    println!("mode : {}", mode(&integer_list));
}


fn avarage(integer_list: &Vec<i32>) -> i32 {

    let mut sum_value: i32 = 0;
    for int_value in integer_list {
        sum_value += int_value; 
    }

    if sum_value == 0 {
        return 0
    }

    println!("{} / {}", sum_value, integer_list.len());

    sum_value / integer_list.len() as i32
}

fn median(integer_list: &Vec<i32>) -> i32 {
    let mut sort_list = integer_list.clone();
    sort_list.sort();

    println!("{:?}", sort_list);

    let len = sort_list.len();
    // 배열 길이가 짝수 
    if len % 2 == 0 {
        return sort_list[len / 2]
    }

    sort_list[(len + 1) / 2]
}

fn mode(integer_list: &Vec<i32>) -> i32 {
    let mut map = HashMap::new();

    let mut max_count = 0;
    let mut max_count_value : i32 = 0;

    for int_value in integer_list {
        let count = map.entry(int_value).or_insert(0);
        *count += 1;

        if *count > max_count {
            max_count = *count;
            max_count_value = *int_value;
        }
    }

    println!("{:?}", map);
    println!("{}, {}", max_count_value, max_count);

    // 최빈 값의 경우는 동일한 빈도로 발생했을떄 무엇을 반환할것인가가 
    // 정의되지 않아서 동일 빈도일시 처음으로 빈도를 넘은 값을 반환한다.
    // 좀더 제대로 반환 하려면 목록을 만들어서 반환하면 될것 같기도 하다.
    // 그럼 코드가 아예 변경되야 하겠지만..

    max_count_value
}
