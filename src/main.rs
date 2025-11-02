use std::io;

fn main() {

    let mut total = 0;
    let mut flag = false;
    loop {
        let mut current_number_str = String::new();

        io::stdin()
            .read_line(&mut current_number_str)
            .expect("Failed to read line");
        if let Ok(current_number) = current_number_str.trim().parse::<i32>() {
            if current_number == -1 {
                break;
            }
            else {
                total += current_number;
            }
        }
        else {
            println!("NaN");
            flag = true;
            break;
        }
    }

    if !(flag) {
        println!("{total}");
    }
}
