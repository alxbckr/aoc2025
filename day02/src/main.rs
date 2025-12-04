use std::fs;

#[derive(PartialEq, Debug)]
struct Interval {
    start: i64,
    end: i64,
}

fn read_file(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");
    return contents.split("\n").map(|s| s.to_string()).collect();
}

fn parse(filename: &str) -> Vec<Interval> {
    let mut res = Vec::new();
    let lines = read_file(filename);
    let parts: Vec<&str> = lines[0].split(',').collect();
    for part in parts{
        let nums: Vec<&str> = part.split('-').collect();
        let start = nums[0].parse::<i64>().unwrap();
        let end = nums[1].parse::<i64>().unwrap();
        res.push( Interval { start: start, end: end } );
    }
    res
}

fn check_occurences(s: &str, sub: &str, ignore_count : bool) -> bool {
    // this checks if the entire string consists of recurring occurrences of sub
    if s.len() % sub.len() != 0 {
        return false;   
    }
    let mut pos = 0;
    let mut rep_cnt = 0;
    while pos < s.len() {
        if &s[pos..pos+sub.len()] != sub {
            return false;   
        }
        pos += sub.len();
        rep_cnt += 1;
        if rep_cnt > 2 && !ignore_count{
            return false;   
        }
    }
    return true;          
}

fn check_interval(interval: &Interval, ignore_count : bool) -> i64 {
    let mut sum : i64 = 0;
    for i in interval.start..=interval.end {
        let i_str = i.to_string();
        for j in 1..=i_str.len()/2 {
            if check_occurences(&i_str, &i_str[0..j], ignore_count) {
                sum += i;
                break;
            }
        }
    }
    return sum;
}

fn part_1(filename: &str) -> i64 {   
    let _dummy = parse(filename);
    let mut ans : i64 = 0;
    for i in _dummy.iter() {
        ans += check_interval(i,false);
    }
    return ans;
}

fn part_2(filename: &str) -> i64 {
    let _dummy = parse(filename);
    let mut ans : i64 = 0;
    for i in _dummy.iter() {
        ans += check_interval(i,true);
    }
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
    fn test_parse_demo_file() {
        let test_file = "dummy_parse_demo.txt";
        create_test_file(test_file, "11-22,95-115");
        
        let result = parse(test_file);
        
        assert_eq!(result.len(), 2);
        assert_eq!(result, vec![
            Interval { start: 11, end: 22 },
            Interval { start: 95, end: 115 }
        ]);
        
        cleanup_test_file(test_file);
    }

    #[test]
    fn test_check_occurence() {
        let test_str = "1188511885";
        let result = check_occurences(test_str, "11885",false);
        assert_eq!(result, true);
    }

    #[test]
    fn test_check_interval() {
        let test_interval = Interval { start: 11, end: 22 };
        let result = check_interval(&test_interval,false);
        assert_eq!(result, 33);
    }    

    #[test]
    fn test_part_1() {
        let test_file = "test-part1.txt";
        let result = part_1(test_file);
        assert_eq!(result,1227775554);
    }

    #[test]
    fn test_part_2() {
        let test_file = "test-part1.txt";
        let result = part_2(test_file);
        assert_eq!(result,4174379265);
    }
}