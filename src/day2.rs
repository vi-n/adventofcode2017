use util;

fn parse_input(input: &str) -> Vec< Vec<u32> > {
    let mut matrix: Vec< Vec<u32> > = vec![];
    for line in input.lines() {
        matrix.push(
            line.split_whitespace()
                .map(|v| v.parse::<u32>().unwrap())
                .collect()
            );
    }

    matrix
}

fn calculate_first_checksum(matrix: &[Vec<u32>]) -> u32 {
    let mut checksum = 0;
    for line in matrix {
        let mut min = u32::max_value();
        let mut max = 0;

        for value in line {
            min = min.min(*value); 
            max = max.max(*value);
        }

        checksum += max - min;
    }

    checksum
}

fn calculate_second_checksum(matrix: &[Vec<u32>]) -> u32 {
    let mut checksum = 0;

    for line in matrix {
        'outer: for i in 0..line.len()-1 {
            for j in i+1..line.len() {
                let a = line[i].max(line[j]);
                let b = line[i].min(line[j]);
                if a % b == 0 {
                    checksum += a / b;
                    break 'outer;
                }
            }
        }
    }

    checksum
}

fn solve_part1(input: &str) -> u32 {
    let matrix = parse_input(input);
    calculate_first_checksum(&matrix)
}

fn solve_part2(input: &str) -> u32 {
    let matrix = parse_input(input);
    calculate_second_checksum(&matrix)
}

pub fn solve_part1_file(path: &str) -> u32 {
    solve_part1(&util::read_file(path).ok().unwrap())
}

pub fn solve_part2_file(path: &str) -> u32 {
    solve_part2(&util::read_file(path).ok().unwrap())
}

#[test]
fn test_examples_part1() {
    let input = "5 1 9 5\n 7 5 3\n 2 4 6 8";

    assert_eq!(solve_part1(input), 18);
}

#[test]
fn test_examples_part2() {
    let input = "5 9 2 8\n9 4 7 3\n3 8 6 5";
    
    assert_eq!(solve_part2(input), 9);
}

#[test]
fn test_given_input() {
    let input = "inputs/day2.txt";
    assert_eq!(solve_part1_file(input), 34581);
    assert_eq!(solve_part2_file(input), 214);
}
