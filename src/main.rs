use std::{fs::File, io};
use io::Write;

mod cb;
mod offset;
mod deflate;
mod ni;

fn main() -> io::Result<()> {
    const FILE: &'static [u8] = include_bytes!("../examples/TESTINSTRUMENTBEST.compressed");

    // match ni::read(FILE) {
    //     Ok(f) => println!("done\n{:?}", f.1),
    //     Err(e) => println!("error: {:?}", e)
    // }

    match deflate::deflate(FILE, 0) {
        Ok((_, content)) => {
            let mut buffer = File::create("TESTINSTRUMENTBEST.deflate")?;
            buffer.write_all(&content)?;
            println!("done!");
        },
        Err(e) => println!("error: {:?}", e)
    }

    Ok(())
}