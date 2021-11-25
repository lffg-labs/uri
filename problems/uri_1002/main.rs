use std::error::Error;
use std::io::{stdin, stdout, BufRead, Write};

const PI: f64 = 3.14159;

fn sln(mut input: impl BufRead, output: &mut impl Write) -> Result<(), Box<dyn Error>> {
    let mut radius = String::new();
    input.read_line(&mut radius)?;

    let radius: f64 = radius.trim().parse()?;
    let area = radius.powi(2) * PI;

    writeln!(output, "A={:.4}", area)?;
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
