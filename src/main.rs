use io_uring::{opcode, types, IoUring};
use std::fs::{OpenOptions, File};
use std::os::unix::io::AsRawFd;
use std::io;

fn main() -> io::Result<()> {
    let mut ring = IoUring::new(8)?;

    let file_read = File::open("input.txt")?;
    
    let file_write = OpenOptions::new()
        .write(true)
        .create(true)
        .open("output.txt")?;

    let mut buf = vec![0; 1024];

    // Reading operation from "input.txt"
    let read_e = opcode::Read::new(
        types::Fd(file_read.as_raw_fd()),
        buf.as_mut_ptr(),
        buf.len() as _,
    )
    .build()
    .user_data(0x42);

    // Writing operation to "output.txt"
    let write_e = opcode::Write::new(
        types::Fd(file_write.as_raw_fd()),
        buf.as_ptr(),
        buf.len() as _,
    )
    .build()
    .user_data(0x43);

    unsafe {
        ring.submission().push(&read_e).expect("submission queue is full");
        ring.submission().push(&write_e).expect("submission queue is full");
    }

    ring.submit_and_wait(2)?;

    Ok(())
}
