//! Advent of Code 2021
//! Day 2.

use std::{
    env,
    fs::File,
    io::{self, BufRead},
};

fn main()
{
    let args: Vec<String> = env::args().collect();
    let filename = &args[1];
    let nums_file = File::open(filename).expect("Unable to read file");
    let commands: Vec<String> = io::BufReader::new(nums_file)
        .lines()
        .map(|line| line.unwrap())
        .collect();
    println!("Your answer is: {}, {}", part1(&commands[..]), part2(&commands[..]));
}

fn part1(commands: &[String]) -> u32
{
    let dims: (u32, u32) = commands.into_iter().fold((0, 0), |acc, item| {
        let params: Vec<&str> = item.split(" ").collect();
        if params.len() != 2
        {
            println!("{:?}", params);
            acc
        }
        else
        {
            let command = params[0];
            let num = params[1].parse::<u32>().unwrap();
            match (command, num)
            {
                ("forward", b) => (acc.0 + b, acc.1),
                ("down", b) => (acc.0, acc.1 + b),
                ("up", b) => (acc.0, acc.1 - b),
                _ => acc,
            }
        }
    });
    dims.0 * dims.1
}

fn part2(commands: &[String]) -> u32
{
    let dims: (u32, u32, u32) = commands.into_iter().fold((0, 0, 0), |acc, item| {
        let params: Vec<&str> = item.split(" ").collect();
        let res = if params.len() != 2
        {
            //println!("{:?}", params);
            acc
        }
        else
        {
            let command = params[0];
            let num = params[1].parse::<u32>().unwrap();
            match (command, num)
            {
                ("forward", b) => (acc.0 + b, acc.1 + (acc.2 * b), acc.2),
                ("down", b) => (acc.0, acc.1, acc.2 + b),
                ("up", b) => (acc.0, acc.1, acc.2 - b),
                _ => acc,
            }
        };
        //println!("res: {:?}", res);
        res
    });
    dims.0 * dims.1
}
