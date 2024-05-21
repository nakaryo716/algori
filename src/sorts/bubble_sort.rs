use std::io;

fn main() {
    let mut buf = String::new();
    io::stdin().read_line(&mut buf).unwrap();
    let unparsed_numbers = buf.split(" ").collect::<Vec<&str>>();

    let mut parsed_numbers = Vec::new();

    for number in unparsed_numbers {
        parsed_numbers.push(number.trim().parse::<i32>().unwrap());
    }

    bubble_sort(&mut parsed_numbers);

    println!("{:?}", parsed_numbers);
}

fn bubble_sort(values: &mut [i32]) {
    for i in 1..values.len() {
        for j in (i..values.len()).rev() {
            if values[j] < values[j - 1] {
                let tmp = values[j - 1];
                values[j - 1] = values[j];
                values[j] = tmp;
            }
        }
    }
}