use std::{collections::BTreeSet, fs, hint::{self, black_box}, io::{self, ErrorKind, Read, Write}, net::{TcpListener, TcpStream}, num::NonZero, os::unix::ffi::OsStrExt, path::PathBuf, str::FromStr, time::{Duration, Instant}};

fn divisors(n: NonZero<u32>) -> BTreeSet<NonZero<u32>> {
    let mut set = BTreeSet::<NonZero<u32>>::new();

    let mut i: u32 = 1;
    loop {
        if n.get() % i == 0 {
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

fn assert_sorted(vec: &Vec<u32>) {
    for x in vec.windows(2) {
        if x[1] < x[0] {
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
    let mut bytes: Vec<u8> = Vec::new();
    let mut slice: &mut [u8] = &mut bytes;
    while size > 0 {
        match stream.read(slice) {
            Ok(amount) => {
                slice = &mut slice[amount..];
                size -= amount;
            },
            Err(e) if e.kind() == ErrorKind::Interrupted => todo!(),
            Err(e) => return Err(e),
        }
    }
    
    Ok(bytes)
}

fn handle_client(mut stream: TcpStream) -> io::Result<()> {
    /* Klient ma przesłać:
     * maks do 200 bajtów ścieżki, zakończonej \n
     */
    let max_n = 200;
    let mut read_bytes: usize = 0;
    let path_bytes = String::new();

    while read_bytes < ma_n {
        // match 
    }
    // Przeczytać bajt po bajcie aż będzie new line character. Chyba że przekroczono 200 znaków wtedy error wysłać klientowi
    // let mut path_bytes = vec![0; n];
    // stream.read_exact(&mut path_bytes)?;

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
                stream.write_all(e.file_name().as_bytes())?;
            },
            Err(e) => {
                stream.write_all(format!("{}\n", e.to_string()).as_bytes())?;
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

    match tcp_server(5000) {
        Ok(_) => println!("Server closing."),
        Err(e) => println!("[ERROR]: {}", e.to_string()),
    }

}
