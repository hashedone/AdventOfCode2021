use std::collections::{HashMap,HashSet};

//part1:4691
//part2:140718
//Elapsed: 1.6700001 
//Elapsed: 1.59

fn count(lim:i32,path:String,c:&HashMap<String,Vec<String>>,p:String,vis:&mut HashMap<String,i32>,all_paths:&mut HashSet<String>)->i32
{
    let pathn = format!("{},{}",path,p);
    
    if p=="end" 
    {
        all_paths.insert(path);
        return 1;
    }

    let points = c.get(&p).unwrap();

    for p in points.iter()
    {
        let lower = p.chars().nth(0).unwrap().is_lowercase();
        let visited = *vis.get(p).unwrap_or(&0);
         
        if !(lower && visited>=lim)
        {
            vis.insert(p.clone(),visited+1);
            count(if lower && visited+1==2 { 1 } else { lim },pathn.clone(), &c, p.clone(), vis, all_paths);
            vis.insert(p.clone(),visited);
        }    
    }
    0
}

fn solution(data:&Vec<String>,limit:i32)->i32
{
    let mut  all_paths : HashSet<String> = HashSet::new();
    let mut  vis : HashMap<String,i32> = HashMap::new();
    let mut conn : HashMap<String,Vec<String>> = HashMap::new();
    
    for line in data {
        let con = line.split('-')
                                 .map(|s| s.to_string())
                                 .collect::<Vec<String>>();

        conn.entry(con[0].clone()).or_insert(Vec::new()).push(con[1].clone());
        conn.entry(con[1].clone()).or_insert(Vec::new()).push(con[0].clone());
    }

    let p = "start".to_string();
    vis.insert("start".to_string(),2);
    
    count(limit,"".to_string(), &conn, p.to_string(), &mut vis,&mut all_paths);
    all_paths.len() as i32
}

pub fn part1(data:&Vec<String>)->i32
{
    solution(data,1)
}

pub fn part2(data:&Vec<String>)->i32
{
    solution(data,2)
}

#[allow(unused)]
pub fn solve(data:&Vec<String>)
{    
    println!("Day12");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn test0()
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
fn test1()
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
fn test2()
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