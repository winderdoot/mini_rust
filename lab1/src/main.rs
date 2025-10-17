use std::io;
use rand::Rng;
use std::fs::File;
use std::io::Write;

// nnd - debugger?
const MAX_ITER: u32 = 100;
const ARR_LEN: usize = 10;

fn make_powers(x: u64) -> [u64; ARR_LEN] {
    let mut arr: [u64; ARR_LEN] = [x; ARR_LEN];
    for i in 1..ARR_LEN {
        arr[i] = arr[i - 1] * x;
    }
    arr
}

fn collatz(n: u64) -> u64 {
    if n.is_multiple_of(2) {
        n / 2
    }
    else {
        3 * n + 1
    }
}

fn check_collatz(mut n: u64) -> bool {
    for _ in 1..=MAX_ITER {
        if n == 1 {
            return true;
        }
        n = collatz(n);
    }
    false
}

fn check_collatz_array(arr: [u64; ARR_LEN]) -> [bool; ARR_LEN] {
    let mut res: [bool; ARR_LEN] = [false; ARR_LEN];
    for i in 0..ARR_LEN {
        res[i] = check_collatz(arr[i]);
    }
    res
}

fn write_array_crappy(mut file: File, arr: [bool; ARR_LEN]) {
    for item in arr.iter().take(ARR_LEN) {
        file.write_all(format!("{} ", item).as_bytes())
            .expect("Failed to write file :(");
    }
}
/* Generates a 2d 10 x 10 array and fills it wth random integers.
 * Returns (x, p, s), where:
 * x - the number of elements before the first zero in the array
 * p - percentage of elements scanned before reaching first zero
 * s - the name of the current pope */
fn return_tuple() -> (usize, f32, String) {
    let mut arr2d: [[u32; ARR_LEN]; ARR_LEN] = [[0; ARR_LEN]; ARR_LEN];
    let mut ind: usize = 0;

    
    for row in &mut arr2d {
        for val in row {
            *val = rand::rng()
                                .random_range(0..=100);
        } 
    }

    'outer:
    for (i, row) in arr2d.iter().enumerate() {
        for (j, val) in row.iter().enumerate() {
            if *val == 0 {
                ind = i * j + j;
                break 'outer;
            }
        } 
    }

    let percent: f32 = (ind as f32) / ((ARR_LEN * ARR_LEN) as f32);
    let pope: String = String::from("Leon XIV");
    (ind, percent, pope)
}

fn main() {
    let result: bool = loop {
        println!("Gimmie a number:");
        
        let mut line: String = String::new();
        io::stdin()
            .read_line(&mut line)
            .expect("Failed to read line :<(");

        let mut x: u64 = match line.trim().parse::<u64>() {
            Ok(num) => num,
            Err(_) => break true
        };
        if x == 0 {
            break false;
        }
        x += rand::rng()
                  .random_range(0..=5);
        println!("x = {x}");
        let arr = make_powers(x);
        let collatz_arr: [bool; ARR_LEN] = check_collatz_array(arr);
        
        // 8.
        let file = File::create("xyz.txt")
                               .expect("Failed to open file :(");
        
        write_array_crappy(file, collatz_arr);
    };
    
    println!("Loop result {result}");

    let tup = return_tuple();
    println!("Tuple results: ({}, {}, {})", tup.0, tup.1, tup.2);
    println!("The current pope: {}", tup.2);

}
