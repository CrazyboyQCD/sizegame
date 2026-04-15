use std::io::{self, Write};

fn main() -> io::Result<()> {
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "Hello, World!")?;
    Ok(())
}
