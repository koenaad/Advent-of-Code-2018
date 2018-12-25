mod vec4;

use crate::vec4::Vec4;

type Point = Vec4;
type Constellation<'a> = Vec<&'a Point>;

fn parse_input(input: &str) -> Result<Vec<Point>, &'static str> {
    let mut points = Vec::new();

    for line in input.lines() {
        if let Ok(point) = Point::parse(line) {
            points.push(point);
        } else {
            return Err("could not parse point");
        }
    }

    Ok(points)
}

fn main() {
    let input = include_str!("../input.txt");
    let points = parse_input(&input)
        .expect("Could not parse input");

    println!("Puzzle 1: {}", puzzle_1(&points));
}

fn is_part_of_constellation(new_point: &Point, constellation: &Constellation) -> bool {
    for point in constellation {
        if Point::distance(&point, &new_point) <= 3 {
            return true;
        }
    }
    false
}

fn to_constellations<'a>(points: &'a Vec<Vec4>) -> Vec<Constellation> {
    let mut constellations = Vec::new();

    for point in points {
        let mut constellations_part_of: Vec<&mut Constellation> = constellations.iter_mut()
            .filter(|constellation| {
                is_part_of_constellation(&point, &constellation)
            })
            .collect();

        match constellations_part_of.len() {
            0 =>  {
                let new_constellation: Vec<&Vec4> = vec![&point];
                constellations.push(new_constellation);
            },
            1 => {
                constellations_part_of[0].push(&point);
            },
            _ => {
                // merge all the other constellations in the_constellation -  I don't like this code
                let the_constellation = constellations_part_of.remove(0);

                for constellation in constellations_part_of {
                    the_constellation.append(constellation);
                    constellation.clear();  // it's not possible to remove `constellation` while `constellations_part_of` is used
                }
                the_constellation.push(&point);
            },
        }
        constellations.retain(|constellation| constellation.len() > 0);
    }
    
    constellations
}

fn puzzle_1(points: &Vec<Vec4>) -> usize {
    let constellations = to_constellations(&points);

    constellations.len()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_puzzle_1_example_1() {
        let example = include_str!("../example_1.txt");
        let points = parse_input(&example).unwrap();

        assert_eq!(puzzle_1(&points), 2);
    }
        
    #[test]
    fn test_puzzle_1_example_2() {
        let example = include_str!("../example_2.txt");
        let points = parse_input(&example).unwrap();

        assert_eq!(puzzle_1(&points), 4);
    }

    #[test]
    fn test_puzzle_1_example_3() {
        let example = include_str!("../example_3.txt");
        let points = parse_input(&example).unwrap();

        assert_eq!(puzzle_1(&points), 3);
    }

    #[test]
    fn test_puzzle_1_example_4() {
        let example = include_str!("../example_4.txt");
        let points = parse_input(&example).unwrap();

        assert_eq!(puzzle_1(&points), 8);
    }
}
