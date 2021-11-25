use std::error::Error;
use std::io::{stdin, stdout, BufRead, Write};

fn sln(mut input: impl BufRead, output: &mut impl Write) -> Result<(), Box<dyn Error>> {
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
