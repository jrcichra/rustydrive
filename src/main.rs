use maligned::align_first;
use std::cmp::Ordering;
use std::fs::OpenOptions;
use std::io::{self, Read, Seek, SeekFrom};
use std::os::unix::fs::OpenOptionsExt;
use std::process;

const READ_ATTEMPTS: usize = 10;
const BUFFER_SIZE: u64 = 4096;

fn main() -> io::Result<()> {
    let filename = std::env::args().nth(1).expect("Please supply a filename");

    let mut f = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_DIRECT)
        .open(filename)
        .expect("Failed to open file");

    let mut buffer: [Vec<u8>; READ_ATTEMPTS] = Default::default();
    for i in 0..buffer.len() {
        buffer[i] = align_first::<u8, maligned::A4096>(BUFFER_SIZE.try_into().unwrap());
        for _ in 0..BUFFER_SIZE {
            buffer[i].push(0);
        }
    }

    let mut location = 0;
    loop {
        f.seek(SeekFrom::Start(location))?;
        for i in &mut buffer {
            let num = f.read(i)?;
            if num <= 0 {
                println!("f.read returned {}", num);
                process::exit(0);
            }
            f.seek(SeekFrom::Start(location))?;
        }

        for i in &buffer[1..] {
            if let Ordering::Less | Ordering::Greater = &buffer[0].cmp(i) {
                println!("Byte mismatch starting at: {}", location);
                process::exit(1);
            }
        }

        location += BUFFER_SIZE;

        println!("{} MB", location);
    }
}
