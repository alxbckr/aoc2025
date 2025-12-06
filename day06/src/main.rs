use std::{fs};

fn read_file(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");
    return contents.split("\n").map(|s| s.to_string()).collect();
}

fn parse_part1(filename: &str) -> Vec<(Vec<u64>,char)> {      
    
    let mut result: Vec<(Vec<u64>,char)> = Vec::new();
    let lines = read_file(filename);

    for line in lines {
        let first_char = line.chars().nth(0).unwrap();
        if first_char == '*' || first_char == '+' { 
            let chars : Vec<char> = line 
                .split_whitespace()
                .map(|s| s.chars().nth(0).unwrap())
                .collect();
            for (i,ch) in chars.iter().enumerate() {
                result[i].1 = *ch;
            }
        } else {
            let numbers : Vec<u64> = line 
                .split_whitespace()
                .map(|s| s.parse::<u64>().unwrap())
                .collect();
            
            for (i,num) in numbers.iter().enumerate() {
                if i >= result.len() {
                    result.push((Vec::new(), ' '));
                }
                result[i].0.push(*num);
            }
        }
    }
    result
}

fn parse_part2(filename: &str) -> Vec<(Vec<u64>,char)> {      
    
    let mut result: Vec<(Vec<u64>,char)> = Vec::new();
    let mut lines = read_file(filename);

    for ch in lines[lines.len()-1]
        .split_whitespace()
        .map(|s| s.chars().nth(0).unwrap())
    {
        result.push((Vec::new(), ch));
    }
    lines.remove(lines.len()-1);

    let max_len = 
        lines.iter()
            .map(|s| { s.len() })
            .max().unwrap();

    let chars : Vec<Vec<char>> = 
        lines.iter().map(|s| {
            let mut clone = s.clone();
            if clone.len() < max_len {
                clone.push_str(&" ".repeat(max_len - clone.len()));
            }
            clone.chars().collect()
        } ).collect();

    let mut problem = 0;
    for x in 0..max_len {
        let mut num_str = String::new();
        for y in 0..chars.len() {
            if chars[y][x] != ' ' {
                num_str.push(chars[y][x]);
            }
        }
        if num_str.is_empty() {
            problem += 1;
            continue;
        }
        result[problem].0.push(num_str.parse::<u64>().unwrap());
    }

    result
}

fn part_1(filename: &str) -> u64 {   
    let problems = parse_part1(filename);
    let mut ans : u64 = 0;
    for problem in problems{
        ans += problem.0.iter().fold(
            if problem.1 == '*' { 1 } else { 0 },
            |acc, &x| {
                if problem.1 == '*' {
                    acc * x
                } else {
                    acc + x
                }
        });
    }
    ans
}

fn part_2(filename: &str) -> u64 {
    let problems = parse_part2(filename);
    let mut ans : u64 = 0;
    for problem in problems{
        ans += problem.0.iter().fold(
            if problem.1 == '*' { 1 } else { 0 },
            |acc, &x| {
                if problem.1 == '*' {
                    acc * x
                } else {
                    acc + x
                }
        });
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
    fn test_parse_part1_file() {
        let test_file = "test-part1.txt";
        let result = parse_part1(test_file);
        
        assert_eq!(result.len(), 4);
        assert_eq!(result[0], (vec![123, 45, 6], '*'));
        assert_eq!(result[1], (vec![328, 64, 98], '+'));
        assert_eq!(result[2], (vec![51, 387, 215], '*'));
        assert_eq!(result[3], (vec![64, 23, 314], '+'));
    }

    #[test]
    fn test_parse_part2_file() {
        let test_file = "test-part1.txt";
        let result = parse_part2(test_file);
        
        assert_eq!(result.len(), 4);
        assert_eq!(result[0], (vec![1, 24, 356], '*'));
        assert_eq!(result[1], (vec![369, 248, 8], '+'));        
        assert_eq!(result[2], (vec![32, 581, 175], '*')); 
        assert_eq!(result[3], (vec![623, 431, 4], '+'));   
    }

    #[test]
    fn test_part_1() {
        let test_file = "test-part1.txt";
        let result = part_1(test_file);
        assert_eq!(result,4277556);
    }

    #[test]
    fn test_part_2() {
        let test_file = "test-part1.txt";
        let result = part_2(test_file);
        assert_eq!(result,3263827);
    }
}