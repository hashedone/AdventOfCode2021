use std::collections::{HashMap};
use std::collections::BTreeMap;

fn expand(e:String,codes:&HashMap<String,char>)->String
{
    let mut res : Vec<String> = vec![];
    res.push(e.chars().nth(0).unwrap().to_string());
    
    for i in 0..e.len()-1 
    {
        let cc:String = [e.chars().nth(i).unwrap().to_string(),e.chars().nth(i+1).unwrap().to_string()].join("") as String;
        
        if codes.contains_key(&cc) {
            res.push(codes[&cc].to_string());
        }
        
        
        res.push(e.chars().nth(i+1).unwrap().to_string());
        
       // println!("{}-{:?}",cc,res);
    }
    res.join("") 
}

fn letter_frequency(input: &str) -> BTreeMap<char, i32> {
    input.chars()
        .fold(BTreeMap::new(), |mut acc, chr| {
            *acc.entry(chr).or_insert(0) += 1;
            acc
        })
}

pub fn part1(data:&[String])->i32
{
    let equation = data[0].to_string();
    let mut code  = HashMap::new();
    
    for line in data.iter()
    {
        if line.contains(" -> ")
        {
            let tt : Vec<&str> = line.split(" -> ").collect();
            code.insert(tt[0].to_string(),tt[1].chars().nth(0).unwrap());
        }        
    }

    let mut res = equation;

    println!("{:?}",code);

    for _ in 0..10 {
        res = expand(res,&code);
        //println!("{}",res)
    }

    let tree = letter_frequency(&res[..]);
    let maxv = tree.values().max().unwrap();
    let minv = tree.values().min().unwrap();

    println!("{:?}",tree);
    
    maxv-minv
}

pub fn part2(data:&[String])->i32
{
    let mut pos = 0;
    let mut depth = 0;
    let mut aim = 0;
    
    for line in data {
        let cmd : Vec<&str> = line.split(' ').collect();
        let x   : i32       = cmd[1].parse().unwrap();

        match cmd[0] {
            "forward" => {   pos+=x; 
                           depth+=x*aim; },
            "up"      =>     aim-=x,
            "down"    =>     aim+=x,
            _         => {},
        }
    }

    pos*depth
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day2");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "NNCB".to_string(),
        "".to_string(),
        "CH -> B".to_string(),
        "HH -> N".to_string(),
        "CB -> H".to_string(),
        "NH -> C".to_string(),
        "HB -> C".to_string(),
        "HC -> B".to_string(),
        "HN -> C".to_string(),
        "NN -> C".to_string(),
        "BH -> H".to_string(),
        "NC -> B".to_string(),
        "NB -> B".to_string(),
        "BN -> B".to_string(),
        "BB -> N".to_string(),
        "BC -> B".to_string(),
        "CC -> N".to_string(),
        "CN -> C".to_string(),
    ];
    assert_eq!(part1(&v),1588);
}

#[test]
fn test2()
{
    let v = vec![
    ];
    assert_eq!(part2(&v),900);
}