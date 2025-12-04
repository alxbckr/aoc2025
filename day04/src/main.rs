use std::fs;

fn read_file(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");
    return contents.split("\n").map(|s| s.to_string()).collect();
}

fn parse(filename: &str) -> Vec<Vec<char>> {
    read_file(filename).iter().map(|line| line.chars().collect()).collect()
}

fn check_surrounding_rolls(lines :&Vec<Vec<char>>, x : usize, y: usize) -> bool {
    let mut cnt = 0;
    for dx in -1..=1 {
        for dy in -1..=1 {
            let nx = x as isize + dx;
            let ny = y as isize + dy;
            if nx < 0 || ny < 0 || ny as usize >= lines.len() || nx as usize >= lines[0].len()
                || (dx == 0 && dy == 0) {
                continue;
            }
            if lines[ny as usize][nx as usize] == '@' {
                cnt += 1;
            }
        }
    }
    if cnt < 4 {
        return true;
    }
    false
}

fn part_1(filename: &str) -> i32 {   
    let lines = parse(filename);
    let mut ans = 0;

    for y in 0..lines.len() {
        for x in 0..lines[0].len() {
            if lines[y][x] == '@' && check_surrounding_rolls(&lines,x,y) {
                ans += 1;
            }
        }
    }
    ans
}

fn part_2(filename: &str) -> i32 {   
    let mut lines = parse(filename);
    let mut rolls_collected: Vec<(usize,usize)> = Vec::new();
    let mut ans = 0;
    let mut rolls_removed = true;

    while rolls_removed {
        rolls_removed = false;
        rolls_collected.clear();
        for y in 0..lines.len() {
            for x in 0..lines[0].len() {
                if lines[y][x] == '@' && check_surrounding_rolls(&lines,x,y) {
                    rolls_collected.push((x,y));
                    ans += 1;
                    rolls_removed = true;
                }
            }
        }
        for (x,y) in &rolls_collected {
            lines[*y][*x] = '.';
        }
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
        let test_file = "dummy_parse_demo.txt";
        create_test_file(test_file, "..@\n@@@\n..@");
        
        let result = parse(test_file);
        
        assert_eq!(result.len(), 3);
        assert_eq!(result, vec![vec!['.', '.', '@'], vec!['@', '@', '@'], vec!['.', '.', '@']]);
        
        cleanup_test_file(test_file);
    }

    #[test]
    fn test_part_1() {
        let test_file = "test-part1.txt";
        let result = part_1(test_file);
        assert_eq!(result,13);
    }

    #[test]
    fn test_part_1_real() {
        let test_file = "input.txt";
        let result = part_1(test_file);
        assert_eq!(result,1428);
    }


    #[test]
    fn test_part_2() {
        let test_file = "test-part1.txt";
        let result = part_2(test_file);
        assert_eq!(result,43);
    }

     #[test]
    fn test_part_2_real() {
        let test_file = "input.txt";
        let result = part_2(test_file);
        assert_eq!(result,8936);
    }   
}