use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::io::{stdin, stdout, BufRead, Write};

#[inline]
fn add_entry(map: &mut HashMap<u32, HashSet<u32>>, from: u32, guest: u32) {
    if from != guest {
        map.entry(from).or_default().insert(guest);
    }
}

fn parse_line_relations(line: &str) -> HashMap<u32, HashSet<u32>> {
    let relations: Vec<(u32, u32)> = line
        .split_ascii_whitespace()
        .map(|str| str[1..str.len() - 1].split(',').map(|n| n.parse().unwrap()))
        .map(|mut iter| (iter.next().unwrap(), iter.next().unwrap()))
        .collect();

    let mut map = HashMap::new();
    for (a, b) in relations {
        add_entry(&mut map, a, b);
        add_entry(&mut map, b, a);
    }
    map
}

fn sln(input: impl BufRead, output: &mut impl Write) -> Result<(), Box<dyn Error>> {
    let mut lines = input.lines();
    loop {
        match lines.next() {
            Some(Ok(string)) if string == "0" => return Ok(()),
            Some(Ok(_)) => (),

            Some(Err(err)) => return Err(Box::new(err)),
            None => unreachable!(),
        };

        let line = lines.next().unwrap()?;
        let mut map = parse_line_relations(&line);

        let mut seen = HashSet::<u32>::new();
        seen.insert(1);

        let mut stack = vec![1];

        while let Some(curr) = stack.pop() {
            while let Some(guests) = map.remove(&curr) {
                for guest in guests {
                    seen.insert(guest);
                    stack.push(guest);
                }
            }
        }

        writeln!(output, "{}", seen.len())?;
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
