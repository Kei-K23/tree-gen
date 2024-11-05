use std::fs::OpenOptions;
use std::io::{self, Write};

pub fn write_output(output_file: &str, content: &str) -> io::Result<()> {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(output_file)?;

    // Write Markdown header and code block syntax
    writeln!(file, "{}", content)?;

    Ok(())
}
