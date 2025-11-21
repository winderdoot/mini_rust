use itertools::Itertools;
use std::collections::HashSet;


#[allow(dead_code)]
fn make_counter(start: i64) -> impl FnMut() -> i64 {
    let mut counter = start;
    move || {
        let ret = counter;
        counter += 1;
        ret
    }
}

// Nie zmieniaj ciała tej funkcji — jedynie typy.
pub fn wrap_call(f1: impl Fn(u32) -> u32, f2: impl FnOnce(u32, u32) -> u32) -> u32 {
    let f1_rename = f1;
    f2(f1_rename(1), f1_rename(2))
}

pub fn sum_squares_odd_loop(list: &[u32]) -> u32 {
    let mut res = 0;
    for val in list {
        if !val.is_multiple_of(2) {
            res  += val * val;
        }
    }
    res
}

pub fn sum_squares_odd(list: &[u32]) -> u32 {
    list.iter()
        .filter(|v| !(**v).is_multiple_of(2))
        .map(|v| v * v)
        .sum()
}

pub fn vertices_loop(edges: &[(u32, u32)]) -> Vec<u32> {
    let mut set = HashSet::<u32>::new();
    for (u, v) in edges {
        set.insert(*u);
        set.insert(*v);
    }
    let mut vec = Vec::<u32>::new();
    for u in set {
        vec.push(u);
    }
    vec.sort();
    vec
}

pub fn vertices(edges: &[(u32, u32)]) -> Vec<u32> {
    edges.iter()
         .flat_map(|(u, v)| vec![*u, *v])
         .unique()
         .sorted()
         .collect()
}

// Zwraca posortowany rosnąco wektor wierzchołków uczestniczących w jakimkolwiek
// cyklu długości 2 (u->v oraz v->u, u!=v), bez duplikatów.
pub fn cycles_2_loop(edges: &[(u32, u32)]) -> Vec<u32> {
    let m = edges.len();
    let mut set = HashSet::<u32>::new();
    for i in 0..m {
        for j in (i+1)..m {
            if edges[i].0 == edges[j].1 && edges[i].1 == edges[j].0 {
                set.insert(edges[i].0);
                set.insert(edges[i].1);
            }
        }
    }
    let mut vec = Vec::<u32>::new();
    for v in set {
        vec.push(v);
    }
    vec.sort();

    vec
}

pub fn cycles_2(edges: &[(u32, u32)]) -> Vec<u32> {
    edges.iter()
         .cartesian_product(edges.iter())
         .filter(|((u, v), (x, y))| u == y && v == x && u != v)
         .flat_map(|((u, v), _)| vec![*u, *v])
         .unique()
         .sorted()
         .collect()
}

/* W tej funkcji nie wolno nam iteratora użwać  */
#[allow(clippy::needless_range_loop)]
pub fn primes_loop(n: u32) -> Vec<u32> {
    let n: usize = n as usize;
    if n <= 2 {
        return Vec::new();
    }
    let mut is_prime: Vec<bool> = vec![true; n + 1];
    is_prime[0] = false;
    is_prime[1] = false;
    let mut i: usize = 2;
    while i * i <= n {
        if is_prime[i] {
            let mut j = i * i;
            loop {
                is_prime[j] = false;
                j += i;
                if j >= n {
                    break;
                }
            }
        }
        if i == 2 {
            i += 1;
        } 
        else {
            i += 2;
        }
    }
    let mut res = Vec::new();
    for i in 2..n {
        if is_prime[i] {
            res.push(i as u32);
        }
    }
    res
}

pub fn primes(n: u32) -> Vec<u32> {
    (2..n)
        .scan(Vec::<u32>::new(), |primes, v| {
            primes
                .iter()
                .filter(|p| (*p) * (*p) <= v)
                .all(|p| v % p != 0)
                .then(|| {
                    primes.push(v);
                    Some(v)
                })
                .or(Some(None))
        })
        .flatten()
        .collect()
}

#[allow(clippy::needless_range_loop)]
pub fn run_length_encode_loop(list: &[u32]) -> Vec<(u32, usize)> {
    if list.is_empty() {
        return Vec::new();
    }
    let mut runs = Vec::new();
    let mut val: u32 = list[0];
    let mut len: usize = 1;
    for i in 1..list.len() {
        if list[i] == val {
            len += 1;
        }
        else {
            runs.push((val, len));
            val = list[i];
            len = 1;
        }
    }
    runs.push((val, len));
    runs
}

pub fn run_length_encode(list: &[u32]) -> Vec<(u32, usize)> {
    list.chunk_by(|x, y| *x == *y)
        .map(|chunk| (chunk[0], chunk.len()))
        .collect()
}

pub fn compose_all_loop(fns: &[fn(i32) -> i32]) -> impl Fn(i32) -> i32 {
    move |x| {
        let mut res = x;
        for f in fns {
            res = f(res);
        }
        res
    }
}

pub fn compose_all(fns: &[fn(i32) -> i32]) -> impl Fn(i32) -> i32 {
    |x| {
        fns
            .iter()
            .fold(x, |v, f| f(v))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn nasty_test() {
        let f1 = |x| x * 100;
        let mut vec = Vec::new();
        let f2 = move |v1, v2| {
            vec.push(v1 + v2);
            let val = vec[0];
            std::mem::drop(vec);
            val
        };
        let val = super::wrap_call(f1, f2);
        assert_eq!(val, 300);
    }

    #[test]
    fn counter_basic() {
        let mut c = make_counter(10);
        assert_eq!(c(), 10);
        assert_eq!(c(), 11);
        assert_eq!(c(), 12);
        let mut c2 = make_counter(-3);
        assert_eq!(c2(), -3);
        assert_eq!(c2(), -2);
        assert_eq!(c(), 13); // niezależne liczniki
    }

    #[test]
    fn sum_squares_odd_cases() {
        let empty: &[u32] = &[];
        assert_eq!(sum_squares_odd_loop(empty), 0);
        assert_eq!(sum_squares_odd(empty), 0);
        let evens = [2, 4, 6];
        assert_eq!(sum_squares_odd_loop(&evens), 0);
        assert_eq!(sum_squares_odd(&evens), 0);
        let nums = [1, 2, 3, 4, 5];
        assert_eq!(sum_squares_odd_loop(&nums), 35);
        assert_eq!(sum_squares_odd(&nums), 35);
    }

    #[test]
    fn vertices_and_cycles() {
        let edges = [(1, 2), (2, 1), (3, 4), (4, 3), (5, 5), (2, 3)];
        let v_loop = vertices_loop(&edges);
        let v_iter = vertices(&edges);
        assert_eq!(v_loop, v_iter);
        assert_eq!(v_loop, vec![1, 2, 3, 4, 5]);
        let c_loop = cycles_2_loop(&edges);
        let c_iter = cycles_2(&edges);
        assert_eq!(c_loop, c_iter);
        assert_eq!(c_loop, vec![1, 2, 3, 4]);
    }

    #[test]
    fn cycles_2_duplicates() {
        let edges = [(1, 2), (2, 1), (1, 2), (2, 1), (2, 2)];
        assert_eq!(cycles_2_loop(&edges), vec![1, 2]);
        assert_eq!(cycles_2(&edges), vec![1, 2]);
    }

    #[test]
    fn empty_graph() {
        let edges: [(u32, u32); 0] = [];
        assert_eq!(vertices_loop(&edges), Vec::<u32>::new());
        assert_eq!(vertices(&edges), Vec::<u32>::new());
        assert_eq!(cycles_2_loop(&edges), Vec::<u32>::new());
        assert_eq!(cycles_2(&edges), Vec::<u32>::new());
    }

    #[test]
    fn primes_examples() {
        assert_eq!(primes_loop(0), Vec::<u32>::new());
        assert_eq!(primes(0), Vec::<u32>::new());
        assert_eq!(primes_loop(2), Vec::<u32>::new());
        assert_eq!(primes(2), Vec::<u32>::new());
        assert_eq!(primes_loop(3), vec![2]);
        assert_eq!(primes(3), vec![2]);
        assert_eq!(primes_loop(10), vec![2, 3, 5, 7]);
        assert_eq!(primes(10), vec![2, 3, 5, 7]);
        assert_eq!(primes_loop(30), vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
        assert_eq!(primes(30), vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]);
    }

    #[test]
    fn primes_large_count() {
        let p100 = primes(100);
        assert_eq!(p100.len(), 25);
        assert_eq!(p100.last(), Some(&97));
        assert_eq!(p100, primes_loop(100));
    }

    #[test]
    fn wrap_call_fn_ptr() {
        fn times2(x: u32) -> u32 {
            x * 2
        }
        let val = wrap_call(times2, |a, b| a + b);
        assert_eq!(val, 6); // 2*1 + 2*2 = 2 + 4 = 6
    }

    #[test]
    fn rle_basic_and_edges() {
        assert_eq!(run_length_encode_loop(&[]), Vec::<(u32, usize)>::new());
        assert_eq!(run_length_encode(&[]), Vec::<(u32, usize)>::new());
        assert_eq!(run_length_encode_loop(&[7]), vec![(7, 1)]);
        assert_eq!(run_length_encode(&[7]), vec![(7, 1)]);
        let data = [1, 1, 2, 2, 2, 1];
        let expect = vec![(1, 2), (2, 3), (1, 1)];
        assert_eq!(run_length_encode_loop(&data), expect);
        assert_eq!(run_length_encode(&data), expect);
    }

    #[test]
    fn rle_varied_runs() {
        let data = [3, 3, 3, 3, 2, 2, 9, 9, 9, 1, 1, 1, 1, 1];
        let expect = vec![(3, 4), (2, 2), (9, 3), (1, 5)];
        assert_eq!(run_length_encode_loop(&data), expect);
        assert_eq!(run_length_encode(&data), expect);
    }

    #[test]
    fn compose_all_identity_and_order() {
        fn add1(x: i32) -> i32 {
            x + 1
        }
        fn times2(x: i32) -> i32 {
            x * 2
        }
        fn square(x: i32) -> i32 {
            x * x
        }

        let id_iter = compose_all(&[]);
        let id_loop = compose_all_loop(&[]);
        assert_eq!(id_iter(42), 42);
        assert_eq!(id_loop(42), 42);

        // Zastosowanie w kolejności: add1, times2, square
        let f_iter = compose_all(&[add1, times2, square]);
        let f_loop = compose_all_loop(&[add1, times2, square]);
        // (((3 + 1) * 2) ^2) = (4*2)^2 = 8^2 = 64
        assert_eq!(f_iter(3), 64);
        assert_eq!(f_loop(3), 64);

        // Odwrócenie kolejności daje inny wynik
        let g_iter = compose_all(&[square, times2, add1]);
        assert_eq!(g_iter(3), ((3 * 3) * 2) + 1);
    }

    #[test]
    fn compose_all_matches_loop() {
        fn f1(x: i32) -> i32 {
            x - 5
        }
        fn f2(x: i32) -> i32 {
            x * 3
        }
        fn f3(x: i32) -> i32 {
            x + 10
        }
        let funcs = [f1, f2, f3];
        let c1 = compose_all(&funcs);
        let c2 = compose_all_loop(&funcs);
        for x in [-10, -1, 0, 1, 7, 20] {
            assert_eq!(c1(x), c2(x));
        }
    }
}
