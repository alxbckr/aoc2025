use std::fs;

fn read_file(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");
    return contents.split("\n").map(|s| s.to_string()).collect();
}

fn parse(filename: &str) -> Vec<Vec<char>> {
    read_file(filename)
        .iter()
        .map(|s| s.chars().collect())
        .collect()
}

fn part_1(filename: &str) -> u64 {   
    let mut manifold = parse(filename);
    let startX = manifold[0].iter().position(|c| *c == 'S').unwrap();

    manifold[0][startX] = '|';
    let mut split_count = 0;
    for y in 1..manifold.len() {
        for x in 0..manifold[y].len() {
            if manifold[y-1][x] != '|' {
                continue;
            }
            if manifold[y][x] == '^' {
                split_count += 1;
                manifold[y][x-1] = '|';
                manifold[y][x+1] = '|';
            } else {
                manifold[y][x] = '|';
            }
        }
    }

    // for l in manifold {
    //     println!("{:?}", l);
    // }

    split_count
}

fn part_2(filename: &str) -> u64 {
    let mut manifold = parse(filename);
    let mut multiverse_count : Vec<Vec<u64>> = vec![vec![0;manifold[0].len()];manifold.len()];
    let startX = manifold[0].iter().position(|c| *c == 'S').unwrap();

    manifold[0][startX] = '|';
    multiverse_count[0][startX] = 1;
    for y in 1..manifold.len() {
        for x in 0..manifold[y].len() {
            if manifold[y-1][x] != '|' {
                continue;
            }
            if manifold[y][x] == '^' {
                manifold[y][x-1] = '|';
                multiverse_count[y][x-1] += multiverse_count[y-1][x];
                manifold[y][x+1] = '|';
                multiverse_count[y][x+1] += multiverse_count[y-1][x];
            } else {
                manifold[y][x] = '|';
                multiverse_count[y][x] += multiverse_count[y-1][x];
            }
        }
    }

    // for m in &multiverse_count{
    //     println!("{:?}",m);
    // }

    multiverse_count[multiverse_count.len()-1].iter().sum()
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
        let test_file = "dummy_parse.txt";
        create_test_file(test_file, ".S.\n.^.");

        let result = parse(test_file);

        assert_eq!(result.len(), 2);
        assert_eq!(result, vec![['.','S','.'],['.','^','.']]);
        
        cleanup_test_file(test_file);
    }

    #[test]
    fn test_part_1() {
        let test_file = "test-part1.txt";
        let result = part_1(test_file);
        assert_eq!(result,21);
    }

    #[test]
    fn test_part_1_real() {
        let test_file = "input.txt";
        let result = part_1(test_file);
        assert_eq!(result,1587);
    }

    #[test]
    fn test_part_2() {
        let test_file = "test-part1.txt";
        let result = part_2(test_file);
        assert_eq!(result,40);
    }
}