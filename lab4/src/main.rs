use std::{collections::BTreeSet, fs, hint::{self}, io::{self, ErrorKind, Read, Write}, net::{TcpListener, TcpStream}, num::NonZero, os::unix::ffi::OsStrExt, path::PathBuf, str::FromStr, time::{Duration, Instant}};

const NEWLINE: [u8;2] = [0xd, 0xa];
#[allow(dead_code)]
const SPACE: [u8;1] = [0x20];

fn divisors(n: NonZero<u32>) -> BTreeSet<NonZero<u32>> {
    let mut set = BTreeSet::<NonZero<u32>>::new();

    let mut i: u32 = 1;
    loop {
        if n.get().is_multiple_of(i) {
            set.insert(NonZero::<u32>::new(i).unwrap());
            set.insert(NonZero::<u32>::new(n.get() / i).unwrap());
        }   
        i += 1;
        if i * i == n.get() {
            set.insert(NonZero::<u32>::new(i).unwrap());
            break;
        } else if i * i > n.get() {
            break;
        }
    }

    set
}

fn assert_sorted(vec: &[u32]) {
    for x in vec.windows(2) {
        if x[0] > x[1] {
            panic!("vec is not sorted!");
        }
    }
}

fn benchmark_divisors(n: u32) -> f64 {
    let mut times = Vec::<Duration>::new();
    for i in 1..n {
        let start = Instant::now();
        hint::black_box(divisors(NonZero::<u32>::new(i).unwrap()));
        let dur = start.elapsed();
        times.push(dur);
    }

    let nanos: u128 = times.iter()
                           .map(|d| d.as_nanos())
                           .sum();
    (nanos as f64) / (1e6 * (n as f64))
}

fn bulk_write(stream: &mut TcpStream, buf: &[u8]) -> io::Result<()> {
    let mut to_write = buf.len();
    let mut buf = buf;
    while to_write > 0 {
        match stream.write(buf) {
            Ok(amount) => {
                buf = &buf[amount..];
                to_write -= amount;
            },
            Err(e) if e.kind() == ErrorKind::Interrupted => {},
            Err(e) => return Err(e),
        }
    }
    Ok(())
}

fn bulk_read(stream: &mut TcpStream, size: usize) -> io::Result<Vec<u8>> {
    let mut size = size;
    let mut bytes: Vec<u8> = vec![0; size];
    let mut slice: &mut [u8] = &mut bytes;
    while size > 0 {
        match stream.read(slice) {
            Ok(amount) => {
                slice = &mut slice[amount..];
                size -= amount;
            },
            Err(e) if e.kind() == ErrorKind::Interrupted => {},
            Err(e) => return Err(e),
        }
    }
    
    Ok(bytes)
}

/* Nigdy nie sądziłem że będzie mi potrzebny algorytm tekstowy z asdów 2. Co ja robię */
fn calc_p(pattern: &[u8]) -> Vec<usize>
{
    let m = pattern.len() + 1;
    let mut p: Vec<usize> = vec![0; m];
    let mut len = 0;

    for j in 2..m {
        /* Można łatwo rozszerzyć prefisko sufiks */
        if pattern[len] == pattern[j - 1] {
            p[j] = len + 1;
            continue;
        }
        /* Nie ma żadnego prefikso-sufiksu */
        else if len == 0 {
            p[j] = 0;
            continue;
        }
        /* Szukamy w policzonym już pod-prefikso-sufiksie  */
        loop {
            len = p[len];
            if !(len > 0 && pattern[len] != pattern[j - 1]) {
                break;
            }
        }
        if pattern[len] == pattern[j - 1] {
            p[j] = len + 1;
        }
    }

    p
}

// end to u mnie \cr\lf czyli 0xad
fn read_until(stream: &mut TcpStream, end: &[u8]) -> io::Result<Vec<u8>> {
    if end.is_empty() {
        return Err(io::Error::other("end must be non zero length!"));
    }
    let mut bytes: Vec<u8> = Vec::new();
    let p = calc_p(end); // Optymalizacja żeby funkcja była liniowa względem długości terminatora xd
    let end_size = end.len();
    let mut i = 0;

    loop {
        match bulk_read(stream, 1) {
            Ok(read) => bytes.extend(read),
            Err(e) => return Err(e),
        };

        if *bytes.last().unwrap() == end[i] {
            i += 1;
        } else {
            i = p[i];
        }
        if i == end_size {
            bytes.truncate(bytes.len() - end_size);
            return Ok(bytes);
        }
    }
}

#[allow(dead_code)]
fn test_client(mut stream: TcpStream) -> io::Result<()> {
    loop {
        let bytes: Vec<u8> = read_until(&mut stream, &NEWLINE)?;
        bytes.iter().for_each(|b| print!("{:#x} ", b));
        println!();
    }
}

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    
    let path_bytes = read_until(&mut stream, &NEWLINE)?;

    let path_str = match String::from_utf8(path_bytes) {
        Ok(s) => s,
        Err(_) => {
            stream.write_all("Bad path\n".as_bytes())?;
            return Ok(())
        },
    };
    let path = match PathBuf::from_str(&path_str) {
        Ok(p) => p,
        Err(_) => {
            stream.write_all("Bad path\n".as_bytes())?;
            return Ok(())
        },
    };

    let dir_iter = match fs::read_dir(path) {
        Ok(iter) => iter,
        Err(_) => {
            stream.write_all("Bad dir\n".as_bytes())?;
            return Ok(())
        },
    };

    for ent in dir_iter {
        match ent {
            Ok(e) => {
                let mut file_name_bytes: Vec<u8> = e.file_name().as_bytes().to_vec();
                file_name_bytes.extend(NEWLINE);
                bulk_write(&mut stream, &file_name_bytes)?;
            },
            Err(e) => {
                println!("[ERROR]{}", e);
                bulk_write(&mut stream, "Bad dir\n".as_bytes())?;
                return Ok(())
            },
        }
    }

    Ok(())
}

fn tcp_server(port: u32) -> io::Result<()> {
    if let 0..=1024 = port {
        return Err(io::Error::new(io::ErrorKind::AddrNotAvailable, format!("Cannot use system port: {}", port)))
    } else if port > 65535 {
        return Err(io::Error::new(io::ErrorKind::AddrNotAvailable, "Cannot use port outside range: 1025-65535"))
    }
    let listener = TcpListener::bind(format!("127.0.0.1:{}", port))?;

    for client in listener.incoming() {
        match client {
            Ok(stream) => handle_client(stream)?,
            Err(e) => return Err(e),
        }
    }

    Ok(())
}

fn main() {

    println!("Average diviros() time: {} ms", benchmark_divisors(10000));
    let mut v: Vec<u32> = vec![2, 5, 1, 624, 2367, 1235, 2137, 11, 751];
    v.sort();
    assert_sorted(&v);

    match tcp_server(5000) {
        Ok(_) => println!("Server closing."),
        Err(e) => println!("[ERROR]: {}", e),
    }

}
