use std::collections::BTreeSet;
use std::error::Error;
use std::io::{stdin, stdout, BufRead, Write};

fn sln(input: impl BufRead, output: &mut impl Write) -> Result<(), Box<dyn Error>> {
    let mut dict = BTreeSet::<String>::new();
    for line in input.lines() {
        let mut control: Option<String> = None;
        for char in line?.chars().map(|char| char.to_ascii_lowercase()) {
            if char.is_ascii_alphabetic() {
                match control.as_mut() {
                    Some(string) => string.push(char),
                    None => control = Some(char.to_string()),
                }
            } else {
                match control {
                    Some(string) => {
                        dict.insert(string);
                        control = None;
                    }
                    None => continue,
                }
            }
        }
        if let Some(string) = control {
            dict.insert(string);
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
