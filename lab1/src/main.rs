use std::fmt::Error;
use std::io;
use rand::Rng;
use std::fs::File;
use std::io::Write;

// nnd - debugger?
const MAX_ITER: u32 = 100;
const ARR_LEN: usize = 10;
const MAX_N: usize = 10000;

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
/* Takes arguments n, m and returns (x, p, s), where:
 * x - 
 * p - 
 * s - the name of the current pope */
fn return_tuple(n: u64) -> (u64, f32, String) {
    let is_prime: [bool; MAX_N+1] = [true; MAX_N+1];
    is_prime[0] = false;
    is_prime[1] = false;
    // is_prime[2] = false;
    for i in 1..MAX_N {
        if is_prime[i] {
            continue;
        }
        else {

        }

    }
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


}
