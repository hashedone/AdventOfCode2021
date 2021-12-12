use std::collections::{HashMap,HashSet};

fn count<'a>(    limit : i32,
           connections : &'a  HashMap<String,Vec<String>>,
                  cave : &str,
                  path : &mut Vec<String>,                  
               visited : &mut HashMap<&'a str,i32>,
             all_paths : &mut HashSet<String>)
{  
    if cave=="end" 
    {
        all_paths.insert(path.join(""));
        return;
    }

    path.push(cave.to_string());

    for point in connections.get(cave).unwrap().iter()
    {
        let is_lower       = point.chars().next().unwrap().is_lowercase();
        let &visited_count = visited.get(&point[..]).unwrap_or(&0);
         
        if !(is_lower && visited_count>=limit)
        {
            visited.insert(&point[..],visited_count+1);

            let new_limit = if is_lower && visited_count+1==2 {     1 } 
                                                         else { limit };

            count(new_limit, connections, point, path, visited, all_paths);

            visited.insert(&point[..],visited_count);
        }    
    }

    path.pop();
}

fn solution(data:&[String],limit:i32)->usize
{
    let mut   all_paths : HashSet<String>             = HashSet::new();
    let mut     visited : HashMap<&str  ,i32>         = HashMap::new();
    let mut connections : HashMap<String,Vec<String>> = HashMap::new();
    let mut        path : Vec<String>                 =     Vec::new();
    
    for line in data 
    {
        let con = line.split('-')
                      .map(|s| s.to_string())
                      .collect::<Vec<String>>();

        connections.entry(con[0].clone()).or_insert_with(Vec::new).push(con[1].clone());
        connections.entry(con[1].clone()).or_insert_with(Vec::new).push(con[0].clone());
    }

    let cave = "start".to_string();
    visited.insert(&cave,2);
    
    count(limit, &connections, &cave, &mut path, &mut visited, &mut all_paths);
    all_paths.len()
}

pub fn part1(data:&[String])->usize
{
    solution(data,1)
}

pub fn part2(data:&[String])->usize
{
    solution(data,2)
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day12");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn test1_0()
{
    let v = vec![
        "start-A".to_string(),
        "start-b".to_string(),
        "A-c".to_string(),
        "A-b".to_string(),
        "b-d".to_string(),
        "A-end".to_string(),
        "b-end".to_string(),
        ];
        assert_eq!(part1(&v),10);
}
    
#[test]
fn test1_1()
{
    let v = vec![
        "dc-end".to_string(),
        "HN-start".to_string(),
        "start-kj".to_string(),
        "dc-start".to_string(),
        "dc-HN".to_string(),
        "LN-dc".to_string(),
        "HN-end".to_string(),
        "kj-sa".to_string(),
        "kj-HN".to_string(),
        "kj-dc".to_string(),
    ];
    assert_eq!(part1(&v),19);
}

#[test]
fn test1_2()
{
    let v = vec![
        "fs-end".to_string(),
        "he-DX".to_string(),
        "fs-he".to_string(),
        "start-DX".to_string(),
        "pj-DX".to_string(),
        "end-zg".to_string(),
        "zg-sl".to_string(),
        "zg-pj".to_string(),
        "pj-he".to_string(),
        "RW-he".to_string(),
        "fs-DX".to_string(),
        "pj-RW".to_string(),
        "zg-RW".to_string(),
        "start-pj".to_string(),
        "he-WI".to_string(),
        "zg-he".to_string(),
        "pj-fs".to_string(),
        "start-RW".to_string(),
        ];
    assert_eq!(part1(&v),226);
}

#[test]
fn test2_0()
{
    let v = vec![
        "start-A".to_string(),
        "start-b".to_string(),
        "A-b".to_string(),
        "A-c".to_string(),
        "b-d".to_string(),
        "A-end".to_string(),
        "b-end".to_string(),
        ];
        assert_eq!(part2(&v),36);
}
    
#[test]
fn test2_1()
{
    let v = vec![
        "dc-end".to_string(),
        "HN-start".to_string(),
        "start-kj".to_string(),
        "dc-start".to_string(),
        "dc-HN".to_string(),
        "LN-dc".to_string(),
        "HN-end".to_string(),
        "kj-sa".to_string(),
        "kj-HN".to_string(),
        "kj-dc".to_string(),
    ];
    assert_eq!(part2(&v),103);
}

#[test]
fn test2_2()
{
    let v = vec![
        "fs-end".to_string(),
        "he-DX".to_string(),
        "fs-he".to_string(),
        "start-DX".to_string(),
        "pj-DX".to_string(),
        "end-zg".to_string(),
        "zg-sl".to_string(),
        "zg-pj".to_string(),
        "pj-he".to_string(),
        "RW-he".to_string(),
        "fs-DX".to_string(),
        "pj-RW".to_string(),
        "zg-RW".to_string(),
        "start-pj".to_string(),
        "he-WI".to_string(),
        "zg-he".to_string(),
        "pj-fs".to_string(),
        "start-RW".to_string(),
        ];
    assert_eq!(part2(&v),3509);
}

//Day12
//part1:4691
//part2:140718
//Elapsed: 0.33400002 secs
//Elapsed: 0.28500003 secs
