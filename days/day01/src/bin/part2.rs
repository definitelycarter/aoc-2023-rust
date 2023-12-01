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
    let words = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];

    let len = input.len();
    let mut start = 0;

    let mut nums: Vec<i32> = Vec::new();

    while start < len {
        let ch = &input[start..start + 1];
        let rest = &input[start..];

        if let Ok(num) = ch.parse::<i32>() {
            nums.push(num);
            start += 1;
            continue;
        }

        let mut found_word: Option<(usize, &str)> = None;
        for (idx, word) in words.iter().enumerate() {
            if rest.starts_with(word) {
                found_word = Some((idx, word));
                break;
            }
        }

        if let Some((idx, word)) = found_word {
            nums.push(idx as i32);
            start += word.len();
        } else {
            start += 1;
        }
    }

    let first = nums.first().unwrap();
    let last = nums.last().unwrap();

    let number = format!("{}{}", first, last).parse()?;
    Ok(number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_sum_correctly() {
        let input = "two1nine\neightwothree\nabcone2threexyz\nxtwone3four\n4nineeightseven2\nzoneight234\n7pqrstsixteen";

        let mut sum = 0;
        let lines = input.lines();

        for line in lines {
            sum += sum_calibration_line(line).unwrap();
        }

        assert_eq!(sum, 281)
    }
}
