use std::cmp::Ordering;
use std::fs::OpenOptions;
use std::io::{self, Read, Seek, SeekFrom};
use std::os::unix::io::AsRawFd;
use std::process;
const BUFFER_SIZE: u64 = 1_000_000;

fn main() -> io::Result<()> {
    let filename = std::env::args().nth(1).expect("Please supply a filename");

    let mut f = OpenOptions::new()
        .read(true)
        // .custom_flags(libc::O_DIRECT)
        .open(filename)
        .expect("Failed to open file");

    unsafe {
        libc::posix_fadvise(f.as_raw_fd(), 0, 0, libc::POSIX_FADV_DONTNEED);
    }

    let mut buffer: [Vec<u8>; 10] = Default::default();
    for i in 0..buffer.len() {
        buffer[i] = vec![0; BUFFER_SIZE.try_into().unwrap()];
    }

    let mut location = 0;
    loop {
        f.seek(SeekFrom::Start(location))?;
        for i in &mut buffer {
            let num = f.read(i)?;
            if num <= 0 {
                println!("Reached end of file");
                process::exit(0);
            }
            f.seek(SeekFrom::Start(location))?;
        }

        for i in &buffer[1..] {
            if let Ordering::Less | Ordering::Greater = &buffer[0].cmp(i) {
                println!("Byte mismatch starting at: {}MB", location / BUFFER_SIZE);
                process::exit(1);
            }
        }

        location += BUFFER_SIZE;

        println!("{} MB", location / BUFFER_SIZE);
    }
}
