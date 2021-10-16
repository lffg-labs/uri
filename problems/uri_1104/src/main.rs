use std::cmp::min;
use std::collections::HashSet;
use std::error::Error;
use std::io::{stdin, stdout, BufRead, Write};
use std::num::ParseIntError;

fn parse_line(line: String) -> Result<HashSet<u32>, ParseIntError> {
    line.split_ascii_whitespace()
        .map(|num_str| num_str.parse())
        .collect()
}

fn sln(input: impl BufRead, output: &mut impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines();
    loop {
        match lines.next() {
            Some(string) if string.as_ref().unwrap() == "0 0" => return Ok(()),
            Some(_) => (),
            None => unreachable!(),
        };

        let set_a = parse_line(lines.next().unwrap()?)?;
        let set_b = parse_line(lines.next().unwrap()?)?;

        // `set_a` can give to `b` the cards which `set_b` does not have.
        // `set_b` can give to `a` the cards which `set_a` does not have.
        let a_give_count = (&set_a - &set_b).len();
        let b_give_count = (&set_b - &set_a).len();

        writeln!(output, "{}", min(a_give_count, b_give_count))?;
    }
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
