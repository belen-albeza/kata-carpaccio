use std::io;

pub fn read_line<R>(mut reader: R) -> String
where
    R: io::BufRead,
{
    let mut buffer = String::new();
    reader.read_line(&mut buffer).expect("Could not read line");
    buffer.trim_end().to_string()
}

pub fn run<R>(reader: R) -> Result<String, String>
where
    R: io::BufRead,
{
    let input = read_line(reader);

    Ok(input)
}
