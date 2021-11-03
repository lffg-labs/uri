use std::collections::BTreeMap;
use std::io::{stdin, stdout, BufRead, Write};

fn sln(input: impl BufRead, output: &mut impl Write) {
    let map: BTreeMap<u32, u32> = input
        .lines()
        .skip(1)
        .map(|line| line.unwrap().parse().unwrap())
        .fold(BTreeMap::new(), |mut map, curr| {
            *map.entry(curr).or_insert(0) += 1;
            map
        });

    for (num, freq) in map {
        writeln!(output, "{} aparece {} vez(es)", num, freq).unwrap();
    }
}

fn main() {
    sln(stdin().lock(), &mut stdout());
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let input = include_bytes!("test-input.txt") as &[u8];
        let mut output = Vec::new();
        super::sln(input, &mut output);

        assert_eq!(
            String::from_utf8(output).unwrap(),
            include_str!("test-output.txt")
        );
    }
}
