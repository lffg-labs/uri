use std::collections::BTreeSet;
use std::error::Error;
use std::io::{stdin, stdout, BufRead, Write};

fn sln(input: impl BufRead, output: &mut impl Write) -> Result<(), Box<dyn Error>> {
    let mut dict = BTreeSet::<String>::new();
    for mut line in input.lines().map(Result::unwrap) {
        line += " ";
        let mut word = String::new();
        for char in line.chars() {
            if char.is_ascii_alphabetic() {
                word.push(char.to_ascii_lowercase());
            } else if word != "" {
                dict.insert(word);
                word = String::new();
            }
        }
    }
    for word in dict {
        writeln!(output, "{}", word)?;
    }
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
