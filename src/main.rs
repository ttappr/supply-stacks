use std::{error::Error, fs::File, io::{BufReader, BufRead}};
use std::collections::BTreeMap;
use regex::Regex;


fn main() -> Result<(), Box<dyn Error>> {
    println!("part_1: {}", part_1()?);
    println!("part_2: {}", part_2()?);
    Ok(())
}

/// Executes the crate moving plan using the CrateMover 9000.
/// 
fn part_1() -> Result<String, Box<dyn Error>> {
    execute_crate_plan(crate_mover_9000)
}

/// Executes the crate moving plan using the CrateMover 9001.
/// 
fn part_2() -> Result<String, Box<dyn Error>> {
    execute_crate_plan(crate_mover_9001)
}


/// Moves the specified crates as if they were moved one at a time with the 
/// CrateMover 9000.
/// 
fn crate_mover_9000(num    : usize, 
                    from   : usize, 
                    to     : usize, 
                    crates : &mut [Vec<String>]) 
{
    let     idx = crates[from].len() - num;
    let mut crs = crates[from].split_off(idx);
    crs.reverse();
    crates[to].append(&mut crs);
}

/// Moves the the specified crates all at once with the CrateMover 9001.
/// 
fn crate_mover_9001(num    : usize, 
                    from   : usize, 
                    to     : usize, 
                    crates : &mut [Vec<String>]) 
{
    let     idx = crates[from].len() - num;
    let mut crs = crates[from].split_off(idx);
    crates[to].append(&mut crs);
}

/// Takes the crate moving plan and executes it using the given crate mover.
/// Processes the file line by line rather than reading it in all at once as
/// one big string, this approach is more memory efficient.
/// 
fn execute_crate_plan<F>(crate_mover_900n: F) -> Result<String, Box<dyn Error>> 
where
    F: Fn(usize, usize, usize, &mut [Vec<String>]),
{
    let file     = File::open("data/data.txt")?;
    let reader   = BufReader::new(file);
    let re_crate = Regex::new(r"\[\w\]")?;
    let re_moves = Regex::new(r"move (\d+) from (\d+) to (\d+)")?;

    let mut lines     = reader.lines();
    let mut crate_map = BTreeMap::new();
    let mut crate_vec = vec![vec![]];

    // Read top lines of file and create the initial stacks of crates.
    for line in &mut lines {
        let line = line?;
        if re_crate.is_match(&line) {
            for cr in re_crate.find_iter(&line) {
                // Use the match offset as the initial stack number.
                let num  = cr.start();
                let name = cr.as_str()[1..2].to_string();
                crate_map.entry(num).or_insert_with(|| vec![]).push(name);
            }
        } else { break; }
    }
    // Put the crate stacks in the correct order in the stack vector.
    for mut stack in crate_map.into_values() {
        stack.reverse();
        crate_vec.push(stack);
    }
    // Read remaining lines of file and move the crates per the instructions.
    for line in &mut lines {
        let line = line?;
        if let Some(caps) = re_moves.captures(&line) {            
            let mov = (1..=3).map(|i| caps[i].parse::<usize>())
                             .collect::<Result<Vec<_>,_>>()?;
                             
            crate_mover_900n(mov[0], mov[1], mov[2], &mut crate_vec);
        }
    }
    // Return a string with the crate name at the top of each stack.
    Ok(crate_vec.into_iter()
                .map(|mut v| v.pop().unwrap_or_default())
                .collect())
}

