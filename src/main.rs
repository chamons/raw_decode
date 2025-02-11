use std::io::stdin;

fn main() -> anyhow::Result<()> {
    let mut input = String::new();
    stdin().read_line(&mut input)?;
    println!("{}", decode(&input));
    Ok(())
}

fn decode(input: &str) -> String {
    let input = input.replace(&['[', ']'], "");
    input
        .split(',')
        .map(|c| match c.trim().parse::<u8>() {
            Ok(c) => c as char,
            Err(_) => ' ',
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use crate::decode;

    #[test]
    fn test_decode() {
        const INPUT: &str = "[123, 34, 114, 101, 97, 115, 111, 110, 34, 58, 34, 73, 110, 118, 97, 108, 105, 100, 80, 114, 111, 118, 105, 100, 101, 114, 84, 111, 107, 101, 110, 34, 125]";
        println!("{}", decode(INPUT));
    }
}
