use std::fs;

fn read_file(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");
    return contents.split("\n").map(|s| s.to_string()).collect();
}

fn parse(filename: &str) -> (Vec<(u64,u64)>, Vec<u64>) {
    let mut intervals: Vec<(u64,u64)> = Vec::new();
    let mut ingredients: Vec<u64> = Vec::new();
    let lines = read_file(filename);
    let mut parse_ingredients = false;
    for line in lines {
        if line.is_empty() {
            parse_ingredients = true;
            continue;
        }
        if !parse_ingredients {
            let arr : Vec<u64> = line.split("-")
                .map(|s| s.parse::<u64>().unwrap())
                .collect();
            intervals.push((arr[0], arr[1]));
        } else {
            ingredients.push(line.parse::<u64>().unwrap());
        }
    }
    (intervals, ingredients)
}

fn part_1(filename: &str) -> u64 {   
    let (intervals,ingredients) = parse(filename);
    let mut ans = 0;
    for ingredient in ingredients {
        if intervals.iter().any(|(low,high)| ingredient >= *low && ingredient <= *high) {
            ans += 1;
        }
    }
    ans
}

fn part_2(filename: &str) -> u64 {
    let (mut intervals,_) = parse(filename);

    let mut merged_intervals: Vec<(u64,u64)> = Vec::new();
    let mut merged = true;

    while merged {
        merged = false;
        merged_intervals.clear();
        for interval in &intervals {
            let mut new_low = interval.0;
            let mut new_high = interval.1;
            for interval_cross in &intervals {
                if interval == interval_cross {
                    continue;
                }
                if interval.0 <= interval_cross.1 && interval.0 >= interval_cross.0 {
                    new_low = interval_cross.0;
                    merged = true;
                }
                if interval.1 >= interval_cross.0 && interval.1 <= interval_cross.1 {
                    new_high = interval_cross.1;
                    merged = true;
                }
            }
            merged_intervals.push((new_low,new_high));
        }
        merged_intervals.sort();
        merged_intervals.dedup();
        intervals = merged_intervals.clone();
    }

    // merged_intervals.iter().for_each(|(low,high)| {
    //     println!("{} - {}", low, high);
    // });
    
    merged_intervals
        .iter()
        .map(|(low,high)| high - low + 1)
        .sum()
}

fn main() {
    let input_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let input_path_str = input_path.to_str().unwrap();
    println!("Answer for part 1: {}", part_1(input_path_str));
    println!("Answer for part 2: {}", part_2(input_path_str));
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::io::Write;

    fn create_test_file(filename: &str, content: &str) {
        let mut file = fs::File::create(filename).unwrap();
        file.write_all(content.as_bytes()).unwrap();
    }

    fn cleanup_test_file(filename: &str) {
        let _ = fs::remove_file(filename);
    }

    #[test]
    fn test_read_file() {
        let test_file = "dummy_read.txt";
        create_test_file(test_file, "line1\nline2\nline3");
        
        let result = read_file(test_file);
        
        assert_eq!(result.len(), 3);
        assert_eq!(result[0], "line1");
        assert_eq!(result[1], "line2");
        assert_eq!(result[2], "line3");
        
        cleanup_test_file(test_file);
    }

    #[test]
    fn test_parse_file() {
        let test_file = "dummy_parse.txt";
        create_test_file(test_file, "11-12\n21-22\n\n7\n8\n9");
        
        let (intervals, ingredients) = parse(test_file);
        
        assert_eq!(intervals.len(), 2);
        assert_eq!(intervals, vec![(11, 12), (21, 22)]);
        assert_eq!(ingredients.len(), 3);
        assert_eq!(ingredients, vec![7, 8, 9]);
        cleanup_test_file(test_file);
    }

    #[test]
    fn test_part_1() {
        let test_file = "test-part1.txt";
        let result = part_1(test_file);
        assert_eq!(result,3);
    }

    #[test]
    fn test_part_1_real() {
        let test_file = "input.txt";
        let result = part_1(test_file);
        assert_eq!(result,690);
    }

    #[test]
    fn test_part_2() {
        let test_file = "test-part1.txt";
        let result = part_2(test_file);
        assert_eq!(result,14);
    }
    #[test]
    fn test_part_2_real() {
        let test_file = "input.txt";
        let result = part_2(test_file);
        assert_eq!(result,344323629240733);
    }

}