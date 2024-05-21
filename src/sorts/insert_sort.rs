use std::{fmt::Display, io};

fn print_line<T>(values: &T) 
where
    T: Clone + IntoIterator,
    <T as IntoIterator>::Item: Display,
{
    let mut element = values.clone().into_iter();
    
    loop {
        match element.next() {
            Some(value) => print!("{} ", value),
            None => {
                print!("\n");
                break;
            }
        }
    }
}

// 挿入ソート
fn insert_sort(value: &mut Vec<i32>) {
    for i in 1..value.len() {
        let mut j = 0;
        while j < i {
            if value[j] > value[i] {
                let temp = value[j];
                value[j] = value[i];
                value[i] = temp;
            }
            j += 1;
        }

        // どのようにソートされているか経過出力
        print_line(value);
    }
}

fn main() {
    // 長さ入力
    let mut text_buf = String::new();
    io::stdin().read_line(&mut text_buf).unwrap();
    let _length = text_buf.trim().parse::<usize>().unwrap();

    // 数字入力
    let mut text_buf = String::new();
    io::stdin().read_line(&mut text_buf).unwrap();
    let text_numbers: Vec<&str> = text_buf.trim().split(" ").collect();

    // 数字変換
    let mut numbers: Vec<i32> = Vec::new();
    for e in text_numbers {
        numbers.push(e.parse::<i32>().unwrap());
    }
    
    // ソート前出力
    print_line(&numbers);

    // ソート実行
    insert_sort(&mut numbers);
}
