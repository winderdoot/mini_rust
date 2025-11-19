use lab6::*;

fn main() {
    let edges = [(1, 2), (2, 1), (3, 4), (4, 3), (5, 5), (2, 3)];

    println!("{:#?}", cycles_2_loop(&edges));

}