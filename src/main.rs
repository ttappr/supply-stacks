#![allow(dead_code)]

use std::{error::Error, fs::File, io::{BufReader, BufRead}};
use std::collections::HashMap;
use regex::Regex;


fn main() -> Result<(), Box<dyn Error>> {
    println!("part_1: {:?}", part_1()?);
    println!("part_2: {}", part_2()?);
    Ok(())
}

fn move_crates_9000(num    : usize, 
                    from   : usize, 
                    to     : usize, 
                    crates : &mut Vec<Vec<String>>) 
{
    let     pos  = crates[from].len() - num;
    let mut temp = crates[from].split_off(pos);
    temp.reverse();
    crates[to].append(&mut temp);
}

fn move_crates_9001(num    : usize, 
                    from   : usize, 
                    to     : usize, 
                    crates : &mut Vec<Vec<String>>) 
{
    let     pos  = crates[from].len() - num;
    let mut temp = crates[from].split_off(pos);
    crates[to].append(&mut temp);
}

fn execute_crate_plan<F>(mover: F) -> Result<String, Box<dyn Error>> 
where
    F: Fn(usize, usize, usize, &mut Vec<Vec<String>>),
{
    let file     = File::open("data/data.txt")?;
    let reader   = BufReader::new(file);
    let re_crate = Regex::new(r"\[\w\]")?;
    let re_moves = Regex::new(r"move (\d+) from (\d+) to (\d+)")?;

    let mut lines     = reader.lines();
    let mut crate_map = HashMap::new();
    let mut crate_vec;

    // Create the initial stacks of crates.
    for line in &mut lines {
        let line = line?;

        if re_crate.is_match(&line) {

            for cr in re_crate.find_iter(&line) {
                let num  = cr.start();
                let name = line.get(cr.start() + 1..cr.end() - 1)
                               .unwrap()
                               .to_string();

                crate_map.entry(num).or_insert(Vec::new()).push(name);
            }
        } else { break; }
    }
    let mut stack_nums = crate_map.keys().copied().collect::<Vec<_>>();
    stack_nums.sort();

    crate_vec = vec![vec![]; stack_nums.len() + 1];

    // Put the crate stacks in the correct order in a vector.
    for (i, k) in (1..).zip(stack_nums) {
        let mut stack = crate_map.remove(&k).unwrap();
        stack.reverse();
        crate_vec[i] = stack;
    }

    // Move the crates.
    for line in &mut lines {
        let line = line?;

        if let Some(caps) = re_moves.captures(&line) {            
            let p = (1..=3).map(|i| caps[i].parse::<usize>())
                           .collect::<Result<Vec<_>,_>>()?;

            let (num, from, to) = (p[0], p[1], p[2]);
            let     pos  = crate_vec[from].len() - num;
            let mut temp = crate_vec[from].split_off(pos);

            temp.reverse();

            crate_vec[to].append(&mut temp);
        }
    }

    // Get the crate name at the top of each stack.
    let mut top_crates = String::new();
    for stack in crate_vec {
        if let Some(name) = stack.last() {
            top_crates.push_str(name);
        }
    }

    Ok(top_crates)
}

fn part_1() -> Result<String, Box<dyn Error>> {
    execute_crate_plan(move_crates_9000)
}

fn part_2() -> Result<u64, Box<dyn Error>> {
    execute_crate_plan(move_crates_9001)
}