use std::{fs};

fn read_file(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");
    return contents.split("\n").map(|s| s.to_string()).collect();
}

fn parse(filename: &str) -> Vec<(usize,usize)> {
    read_file(filename)
        .iter()
        .map(|line| {
            let nums: Vec<usize> = line.split(',').map(|s| s.parse::<usize>().unwrap()).collect();
            (nums[0], nums[1])
        })
        .collect()
}

fn part_1(filename: &str) -> u64 {   
    let tiles = parse(filename);
    let mut max_area = 0u64;
    for i in 0..tiles.len()-1 {
        for j in i+1..tiles.len() {
            let area = ( ( tiles[i].0 as i32 - tiles[j].0 as i32 ).abs() + 1 ) as u64 * 
                       ( ( tiles[i].1 as i32 - tiles[j].1 as i32 ).abs() + 1 ) as u64;
            max_area = max_area.max(area);
        }
    }
    max_area
}

fn part_2(filename: &str) -> i32 {
    let _dummy = parse(filename);
    let ans = 0;
    return ans;
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
    fn test_parse_dummy_file() {
        let test_file = "dummy_parse.txt";
        create_test_file(test_file, "7,1\n11,2");
        
        let result = parse(test_file);
        
        assert_eq!(result.len(), 2);
        assert_eq!(result,vec![(7,1),(11,2)]);
        
        cleanup_test_file(test_file);
    }

    #[test]
    fn test_part_1() {
        let test_file = "test-part1.txt";
        let result = part_1(test_file);
        assert_eq!(result,50);
    }

    #[test]
    fn test_part_2() {
        let test_file = "test-part1.txt";
        let result = part_2(test_file);
        assert_eq!(result,0);
    }
}