use std::error::Error;
use std::io::{stdin, stdout, BufRead, Write};

fn sln(mut input: impl BufRead, output: &mut impl Write) -> Result<(), Box<dyn Error>> {
    let mut a = String::new();
    let mut b = String::new();
    input.read_line(&mut a)?;
    input.read_line(&mut b)?;
    let a: i32 = a.trim().parse()?;
    let b: i32 = b.trim().parse()?;
    writeln!(output, "X = {}", a + b)?;
    Ok(())
}

fn main() {
    sln(stdin().lock(), &mut stdout()).unwrap();
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let input = include_bytes!("test-input.txt") as &[u8];
        let mut output = Vec::new();
        super::sln(input, &mut output).unwrap();

        assert_eq!(
            String::from_utf8(output).unwrap(),
            include_str!("test-output.txt")
        );
    }
}
