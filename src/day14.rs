use std::collections::{HashMap};

pub fn count(data:&[String],cnt:usize)->i64
{
    let start = data[0].to_string();
    let mut code = HashMap::new();
    
    for line in data.iter()
    {
        if line.contains(" -> ")
        {
            let table : Vec<&str> = line.split(" -> ").collect();
            code.insert((table[0].chars().next().unwrap(),
                         table[0].chars().nth(1).unwrap()),
                         table[1].chars().next().unwrap());
        }        
    }

    let mut freq  = HashMap::new();
    let mut pairs = HashMap::new();

    for i in 0..start.len() 
    {
        let c = start.chars().nth(i).unwrap();
        if i<start.len()-1 
        {
            let p = (c,start.chars().nth(i+1).unwrap());
            pairs.insert(p, pairs.get(&p).unwrap_or(&0)+1);
        }
        
        freq.insert(c,freq.get(&c).unwrap_or(&0)+1);
    }
     
    (0..cnt).for_each(|_|  
       pairs.clone()
            .iter()
            .for_each(|(pair,count)|
            if code.get(pair)!=None
            {
                let ch = *code.get(pair).unwrap();
                freq.insert(ch,freq.get(&ch).unwrap_or(&0)+count);

                *pairs.entry((pair.0,     ch)).or_insert(0)+=count;
                *pairs.entry((    ch, pair.1)).or_insert(0)+=count;
                *pairs.entry(           *pair).or_insert(0)-=count;
            }
        )
    );
    
    freq.values().max().unwrap() - freq.values().min().unwrap()
}

pub fn part1(data:&[String])->i64
{
    count(data,10)
}

pub fn part2(data:&[String])->i64
{
    count(data,40)
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day14");
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
    assert_eq!(part2(&v),2188189693529);
}