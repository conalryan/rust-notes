use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};

fn main() -> io::Result<()> {
    let f = File::open("foo.txt")?;
    let mut reader = BufReader::new(f);
    let mut buffer = String::new();

    // read a line into buffer
    reader.read_line(&mut buffer)?;

    println!("{}", buffer);
    // Ok(())

    // writer
    let f = File::create("bar.txt")?;
    {
        let mut writer = BufWriter::new(f);

        // write a byte to the buffer
        writer.write(&[42])?;
    } // the buffer is flushed once writer goes out of scope

    Ok(())
}
