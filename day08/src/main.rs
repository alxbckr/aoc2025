use std::{fs};

#[derive(PartialEq, Debug, Clone, Hash, Copy)]
struct Coord {
    x : u64,
    y : u64,
    z : u64
}

#[derive(Clone, Copy)]
struct CoordPair {
    c1 : Coord,
    c2 : Coord
}

impl CoordPair {
    fn dist(&self) -> f64 {
        let dx = self.c1.x.abs_diff(self.c2.x) as f64;
        let dy = self.c1.y.abs_diff(self.c2.y) as f64;
        let dz = self.c1.z.abs_diff(self.c2.z) as f64;
        ((dx * dx + dy * dy + dz * dz) as f64).sqrt()
    }
}

impl PartialEq for CoordPair {
    fn eq(&self, other: &Self) -> bool {
        (self.c1 == other.c1 && self.c2 == other.c2) ||
        (self.c1 == other.c2 && self.c2 == other.c1)
    }
}

impl Eq for CoordPair {}

fn read_file(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename)
    .expect("Something went wrong reading the file");
    return contents.split("\n").map(|s| s.to_string()).collect();
}

fn parse(filename: &str) -> Vec<Coord> {
    let lines = read_file(filename);
    lines.iter()
        .map(|s| {
            let mut parts = s.split(',').map(|p| p.parse::<u64>().unwrap());            
            Coord { x: parts.next().unwrap(), y: parts.next().unwrap(), z: parts.next().unwrap() }
        })
        .collect()
}

fn part_1(filename: &str, connection_count : usize) -> i32 {   

    let boxes = parse(filename);
    let mut distances = Vec::new();
    for i in 0..boxes.len()-1 {
        for j in (i + 1)..boxes.len() {
            distances.push(CoordPair { c1: boxes[i], c2: boxes[j] });
        }
    }

    distances.sort_by(|a, b| a.dist().partial_cmp(&b.dist()).unwrap());
    
    let mut circuits : Vec<Vec<Coord>> = Vec::new();
    for b in boxes {
        circuits.push(Vec::new());
        let last_idx = circuits.len() - 1;
        circuits[last_idx].push(b);        
    }    

    let mut conn = 0;
    let mut dist_idx = 0;
    while conn < connection_count {
        if dist_idx >= distances.len() {
            break;
        }
        let pair = &distances[dist_idx];
        dist_idx += 1;

        let mut idx_c1 = 0;
        let mut idx_c2 = 0;
        // print!("Connecting {:?} to {:?}", pair.c1, pair.c2);
        for (i, circuit) in circuits.iter().enumerate() {
            if circuit.contains(&pair.c1) { idx_c1 = i; }
            if circuit.contains(&pair.c2) { idx_c2 = i; }
        }
        
        if idx_c1 == idx_c2 {
            // println!(" - same circuit");
        } else {
            let (target, remove_idx) = if idx_c1 < idx_c2 { (idx_c1, idx_c2) } else { (idx_c2, idx_c1) };
            let mut to_append = circuits.remove(remove_idx);
            circuits[target].append(&mut to_append);
            // println!(" monving {} to {}", remove_idx, target);
        }
        conn += 1;
    }

    circuits.sort_by(|a, b| b.len().cmp(&a.len()));
    // for c in &circuits {
    //     println!("{}", c.len());
    // }  
    ( circuits[0].len() * circuits[1].len() * circuits[2].len() ) as i32
}

fn part_2(filename: &str) -> i64 {

    let boxes = parse(filename);
    let mut distances = Vec::new();
    for i in 0..boxes.len()-1 {
        for j in (i + 1)..boxes.len() {
            distances.push(CoordPair { c1: boxes[i], c2: boxes[j] });
        }
    }

    distances.sort_by(|a, b| a.dist().partial_cmp(&b.dist()).unwrap());
    
    let mut circuits : Vec<Vec<Coord>> = Vec::new();
    for b in boxes {
        circuits.push(Vec::new());
        let last_idx = circuits.len() - 1;
        circuits[last_idx].push(b);        
    }    

    let mut dist_idx = 0;
    let mut lastPair : CoordPair = distances[0];

    while dist_idx < distances.len() {
        let pair = &distances[dist_idx];
        dist_idx += 1;

        if circuits.len() == 1 {
            break;
        }

        let mut idx_c1 = 0;
        let mut idx_c2 = 0;
        // print!("Connecting {:?} to {:?}", pair.c1, pair.c2);
        for (i, circuit) in circuits.iter().enumerate() {
            if circuit.contains(&pair.c1) { idx_c1 = i; }
            if circuit.contains(&pair.c2) { idx_c2 = i; }
        }
        
        if idx_c1 == idx_c2 {
            // println!(" - same circuit");
        } else {
            let (target, remove_idx) = if idx_c1 < idx_c2 { (idx_c1, idx_c2) } else { (idx_c2, idx_c1) };
            let mut to_append = circuits.remove(remove_idx);
            circuits[target].append(&mut to_append);
            // println!(" monving {} to {}", remove_idx, target);
        }        
        lastPair = *pair;
    }

    ( lastPair.c1.x * lastPair.c2.x ) as i64
}

fn main() {
    let input_path = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("input.txt");
    let input_path_str = input_path.to_str().unwrap();
    println!("Answer for part 1: {}", part_1(input_path_str,1000));
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
        create_test_file(test_file, "1,2,3\n3,2,1");
        
        let result = parse(test_file);
        
        assert_eq!(result.len(), 2);
        assert_eq!(result,vec![Coord{x:1,y:2,z:3},Coord{x:3,y:2,z:1}]);
        
        cleanup_test_file(test_file);
    }

    #[test]
    fn test_part_1() {
        let test_file = "test-part1.txt";
        let result = part_1(test_file,10);
        assert_eq!(result,40);
    }

    #[test]
    fn test_part_2() {
        let test_file = "test-part1.txt";
        let result = part_2(test_file);
        assert_eq!(result,25272);
    }
}