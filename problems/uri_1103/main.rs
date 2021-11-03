use std::error::Error;
use std::io::{stdin, stdout, BufRead, Write};

const DAY_MINUTES: u32 = 24 * 60;

fn sln(input: impl BufRead, output: &mut impl Write) -> Result<(), Box<dyn Error>> {
    for line in input.lines() {
        if let [hh_before, mm_before, hh_after, mm_after] = line?
            .split_ascii_whitespace()
            .map(|str| str.parse::<u32>().unwrap())
            .collect::<Vec<_>>()
            .as_slice()
        {
            let tmm_before = hh_before * 60 + mm_before;
            let tmm_after = hh_after * 60 + mm_after;

            let total = if tmm_after >= tmm_before {
                tmm_after - tmm_before
            } else {
                DAY_MINUTES - tmm_before + tmm_after
            };

            if total == 0 {
                return Ok(()); // Must finish in this case.
            }

            writeln!(output, "{}", total)?;
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
