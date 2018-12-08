mod dependency;

use crate::dependency::*;
use std::collections::HashMap;

fn get_input() -> Vec<&'static str> {
    include_str!("../input.txt")
        .split("\n")
        .filter(|s| s.len() > 0)
        .collect()
}

fn main() {
    let input = get_input();

    println!("Puzzle 1: {}", puzzle_1(&input));
    println!("Puzzle 2: {}", puzzle_2(&input, 5, |c| char_to_num(c) + 60));
}

fn char_to_num(c: char) -> i32 {
    "_ABCDEFGHIJKLMNOPQRSTUVWXYZ".find(c).unwrap() as i32
}

fn get_unique_steps_sorted(dependencies: &Vec<Dependency>) -> Vec<char> {
    let mut steps = Vec::with_capacity(dependencies.len());

    for dependency in dependencies {
        steps.push(dependency.step);
        steps.push(dependency.depends_on);
    }

    steps.sort();
    steps.dedup();

    steps
}

fn get_free_steps(steps: &Vec<char>, dependencies: &Vec<Dependency>) -> Vec<char> {
    steps.iter()
        .filter(|step| {
            dependencies.iter()
                .filter(|dependency| dependency.step == **step)
                .count() == 0
        })
        .map(|&c| c)
        .collect()
}

fn remove_from(list: &mut Vec<char>, to_remove: &Vec<char>) {
    list.retain(|step| !to_remove.contains(step));
}

fn remove_from_deps(list: &mut Vec<Dependency>, to_remove: &Vec<char>) {
    list.retain(|dependency| !to_remove.contains(&dependency.depends_on));
}

fn do_the_work<Work: Fn(char) -> i32>(mut dependencies: Vec<Dependency>, workers: i32, calculate_work: Work) -> (String, i32) {
    let mut steps_todo = get_unique_steps_sorted(&dependencies);
    let mut steps_working_on = Vec::<char>::new();
    let mut steps_done = Vec::<char>::new();

    let mut elapsed_seconds = 0;

    let mut steps_work_left = HashMap::<char, i32>::new();
    for step in &steps_todo {
        steps_work_left.insert(*step, calculate_work(*step));
    }

    loop {
        // if workers are idle, take on some new work
        let workers_idle = (workers as usize) - steps_working_on.len();
        if workers_idle > 0 {
            let mut steps_free = get_free_steps(&steps_todo, &dependencies);

            steps_free.truncate(workers_idle);
            for step in &steps_free {
                steps_working_on.push(*step);
            }

            remove_from(&mut steps_todo, &steps_working_on);
        }

        // println!("* loop: {}", elapsed_seconds);
        // println!("todo: {:?}", steps_todo);
        // println!("work: {:?}", steps_working_on);
        // println!("done: {:?}", steps_done);
        // println!("time: {:?}", steps_work_left);

        // do the work
        for step in &steps_working_on {
            let work_left = steps_work_left.get_mut(&step)
                .expect("Found no work time for this step");

            *work_left -= 1;

            if *work_left == 0 {
                steps_done.push(*step);
            }
        }

        // bookkeeping
        remove_from(&mut steps_working_on, &steps_done);
        remove_from_deps(&mut dependencies, &steps_done);

        elapsed_seconds += 1;

        if steps_todo.is_empty() && steps_working_on.is_empty() {
            break;
        }
    }
    (steps_done.into_iter().collect(), elapsed_seconds)
}

fn puzzle_1(input: &Vec<&str>) -> String {
    let dependencies = Dependency::parse_vec(&input);

    do_the_work(dependencies, 1, |_| { 1 })
        .0
}

fn puzzle_2<Work: Fn(char) -> i32>(input: &Vec<&str>, workers: i32, calculate_work: Work) -> i32 {
    let dependencies = Dependency::parse_vec(&input);

    do_the_work(dependencies, workers, calculate_work)
        .1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_char_to_num() {
        assert_eq!(char_to_num('A'), 1);
        assert_eq!(char_to_num('B'), 2);
        assert_eq!(char_to_num('C'), 3);
        assert_eq!(char_to_num('Z'), 26);
    }

    #[test]
    fn test_puzzle_1() {
        let mut example = Vec::new();
        example.push("Step C must be finished before step A can begin.");
        example.push("Step C must be finished before step F can begin.");
        example.push("Step A must be finished before step B can begin.");
        example.push("Step A must be finished before step D can begin.");
        example.push("Step B must be finished before step E can begin.");
        example.push("Step D must be finished before step E can begin.");
        example.push("Step F must be finished before step E can begin.");

        assert_eq!(puzzle_1(&example), "CABDFE".to_string());
    }

    #[test]
    fn test_puzzle_2() {
        let mut example = Vec::new();
        example.push("Step C must be finished before step A can begin.");
        example.push("Step C must be finished before step F can begin.");
        example.push("Step A must be finished before step B can begin.");
        example.push("Step A must be finished before step D can begin.");
        example.push("Step B must be finished before step E can begin.");
        example.push("Step D must be finished before step E can begin.");
        example.push("Step F must be finished before step E can begin.");

        assert_eq!(puzzle_2(&example, 2, |c| char_to_num(c)), 15);
    }
}
