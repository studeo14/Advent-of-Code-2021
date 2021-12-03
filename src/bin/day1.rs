//! Advent of Code 2021
//! Day 1.

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
    let nums: Vec<u32> = io::BufReader::new(nums_file)
        .lines()
        .map(|line| line.unwrap().trim().parse::<u32>())
        .filter_map(|res| res.ok())
        .collect();
    println!("Your answer is: {}, {}", part1(&nums[..]), part2(&nums[..]));
}

fn part1(nums: &[u32]) -> u32
{
    nums.windows(2).fold(0, |acc, item| {
        if item[1] > item[0]
        {
            acc + 1
        }
        else
        {
            acc
        }
    })
}

fn part2(nums: &[u32]) -> u32
{
    nums.windows(3)
        .map(|item| item.iter().sum())
        .collect::<Vec<u32>>()
        .windows(2)
        .fold(0, |acc, item| {
            if item[1] > item[0]
            {
                acc + 1
            }
            else
            {
                acc
            }
        })
}
