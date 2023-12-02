const MAX_RED: usize = 12;
const MAX_GREEN: usize = 13;
const MAX_BLUE: usize = 14;

fn main() -> anyhow::Result<()> {
    let input = include_str!("input.txt");

    let mut sum = 0;
    for line in input.lines() {
        if let Ok(game_number) = is_game_possible(line) {
            sum += game_number;
        }
    }

    println!("Sum: {}", sum);

    Ok(())
}

fn is_game_possible(input: &str) -> anyhow::Result<i32> {
    let parts = input.split(":").collect::<Vec<&str>>();

    let game_number = if let Some(game) = parts.get(0) {
        game.strip_prefix("Game ").unwrap().parse::<i32>().unwrap()
    } else {
        return Err(anyhow::anyhow!("Invalid input"));
    };

    if let Some(sets) = parts.get(1) {
        for set in sets.split(";").into_iter() {
            for color in set.split(",").into_iter() {
                let color = color.trim().split(" ").collect::<Vec<&str>>();
                let Some(count) = color.get(0) else {
                    return Err(anyhow::anyhow!("Invalid input"));
                };

                let Some(color) = color.get(1) else {
                    return Err(anyhow::anyhow!("Invalid input"));
                };

                let Ok(count) = count.parse::<usize>() else {
                    return Err(anyhow::anyhow!("Invalid input"));
                };

                match *color {
                    "red" => {
                        if count > MAX_RED {
                            return Err(anyhow::anyhow!("Invalid input"));
                        }
                    }
                    "green" => {
                        if count > MAX_GREEN {
                            return Err(anyhow::anyhow!("Invalid input"));
                        }
                    }
                    "blue" => {
                        if count > MAX_BLUE {
                            return Err(anyhow::anyhow!("Invalid input"));
                        }
                    }
                    _ => {
                        return Err(anyhow::anyhow!("Invalid input"));
                    }
                }
            }
        }
    } else {
        return Err(anyhow::anyhow!("Invalid input"));
    };

    Ok(game_number)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_should_be_possible_game() {
        let input = r#"
            Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green
        "#;

        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let line = line.trim();
            let is_possible = is_game_possible(line);
            assert!(is_possible.is_ok())
        }
    }

    #[test]
    fn it_should_be_impossible_game() {
        let input = r#"
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
        "#;

        for line in input.lines() {
            let line = line.trim();
            if line.is_empty() {
                continue;
            }
            let is_possible = is_game_possible(line);
            assert!(is_possible.is_err())
        }
    }
}
