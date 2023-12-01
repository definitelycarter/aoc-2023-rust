fn main() -> anyhow::Result<()> {
    let input = include_str!("input.txt");

    let mut sum = 0;
    let lines = input.lines();

    for line in lines {
        sum += sum_calibration_line(line)?;
    }

    println!("Sum: {}", sum);

    Ok(())
}

fn sum_calibration_line(input: &str) -> anyhow::Result<i32> {
    let chars = input
        .chars()
        .filter_map(|c| if c.is_digit(10) { Some(c) } else { None })
        .collect::<Vec<_>>();

    let first = chars.first().unwrap();
    let last = chars.last().unwrap();

    let num = format!("{}{}", first, last);

    let value = num.parse::<i32>()?;

    Ok(value)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_sum_correctly() {
        let input = r#"1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"#;

        let mut sum = 0;
        let lines = input.lines();

        for line in lines {
            sum += sum_calibration_line(line).unwrap();
        }

        assert_eq!(sum, 142)
    }
}
