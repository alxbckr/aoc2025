use std::fs;

fn read_file(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");
    return contents.split("\n").map(|s| s.to_string()).collect();
}

fn parse(filename: &str) -> Vec<String> {
    read_file(filename)
}

fn find_joltage(line: &str, battery_len: usize) -> u64 {
    let chars: Vec<i32> = line
        .chars()
        .filter_map(|c| c.to_digit(10).map(|d| d as i32))
        .collect();

    let mut joltage : u64 = 0;
    let mut max_pos_prev = 0;
    for j in 1..=battery_len {
        let (max_pos, first_max) = chars[max_pos_prev..chars.len() - battery_len + j]
            .iter()
            .enumerate()
            .fold((0, 0), |(best_i, best_val), (i, &val)| {
                if val > best_val {
                    (i + max_pos_prev, val)
                } else {
                    (best_i, best_val)
                }
            });
        joltage += first_max as u64 * 10_u64.pow((battery_len - j) as u32);
        max_pos_prev = max_pos + 1;
    }
    joltage
}

fn part_1(filename: &str) -> u64 {   
    let lines = parse(filename);
    let mut ans = 0;
    for line in lines {
        ans += find_joltage(&line,2);
    }
    ans
}

fn part_2(filename: &str) -> u64 {
    let lines = parse(filename);
    let mut ans = 0;
    for line in lines {
        ans += find_joltage(&line,12);
    }
    ans
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
    fn test_parse_demo_file() {
        let test_file = "dummy_parse_demo_file.txt";
        create_test_file(test_file, "123\n456\n789");
        
        let result = parse(test_file);
        
        assert_eq!(result.len(), 3);
        assert_eq!(result, vec!["123", "456", "789"]);
        
        cleanup_test_file(test_file);
    }

    #[test]
    fn test_find_joltage_part1() {
        let mut result = find_joltage("987654321111111",2);
        assert_eq!(result,98);
        result = find_joltage("811111111111119",2);
        assert_eq!(result,89);
        result = find_joltage("234234234234278",2);
        assert_eq!(result,78);      
        result = find_joltage("818181911112111",2);
        assert_eq!(result,92);                  
    }

    #[test]
    fn test_find_joltage_part2() {
        let mut result = find_joltage("987654321111111",12);
        assert_eq!(result,987654321111);
        result = find_joltage("811111111111119",12);
        assert_eq!(result,811111111119);
        result = find_joltage("234234234234278",12);
        assert_eq!(result,434234234278);      
        result = find_joltage("818181911112111",12);
        assert_eq!(result,888911112111);                  
    }

    #[test]
    fn test_part_1() {
        let test_file = "test-part1.txt";
        let result = part_1(test_file);
        assert_eq!(result,357);
    }

    #[test]
    fn test_part_1_input() {
        let test_file = "input.txt";
        let result = part_1(test_file);
        assert_eq!(result,17095);
    }

    #[test]
    fn test_part_2() {
        let test_file = "test-part1.txt";
        let result = part_2(test_file);
        assert_eq!(result,3121910778619);
    }
}