use std::cmp::Reverse;
use std::collections::HashMap;
use std::error::Error;
use std::io::{stdin, stdout, BufRead, Write};

fn sln(input: impl BufRead, output: &mut impl Write) -> Result<(), Box<dyn Error>> {
    let mut is_first = true;
    for line in input.lines() {
        if is_first {
            is_first = false;
        } else {
            writeln!(output)?;
        }
        let mut stats = HashMap::<char, u32>::new();
        for char in line?.trim().chars() {
            *stats.entry(char).or_insert(0) += 1;
        }
        let mut stats: Vec<_> = stats.into_iter().collect();
        stats.sort_by_key(|(char, freq)| (*freq, Reverse(*char)));
        for (char, freq) in stats {
            writeln!(output, "{} {}", char as u32, freq)?;
        }
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
