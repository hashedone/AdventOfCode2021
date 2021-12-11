use std::collections::{HashMap,HashSet};

pub fn calc(data:&Vec<String>,steps:i32,part_two:bool)->i32
{
    let dx = data[0].len();
    let dy = data.len();

    let mut fields : HashMap<(i32,i32),u32> = HashMap::new();

    for y in 0..dy {
    for x in 0..dx {
        fields.insert((x as i32,y as i32),
                      data[y].chars().nth(x).unwrap().to_digit(10).unwrap() );
    }
    }

    let keys = fields.keys()
                     .map(|(x,y)| (*x,*y))
                     .collect::<Vec<(i32,i32)>>();
    
    let mut res = 0;

    for step in 1..=steps 
    {
        for (_,val) in fields.iter_mut() { *val+=1; }

        let mut flashed = HashSet::new();
        let mut flash   = true;
        
        while flash 
        {
            flash = false;

            for p in &keys
            {
                if *fields.get(&p).unwrap()>9 && !flashed.contains(&p)
                {
                    flashed.insert(p);
                    res+=1;
                    flash = true;

                    for yy in -1..=1 {
                    for xx in -1..=1 {
                    if xx!=0 || yy!=0
                    {
                        let pn = (p.0 + xx,p.1 + yy);
                        fields.insert(pn,fields.get(&pn).unwrap_or(&0) + 1);
                    }
                    }
                    }
                }
            }
        }

        if part_two && flashed.len()==dx*dy { return step; }

        for p in &keys
        {
            *fields.get_mut(p).unwrap() = if flashed.contains(p) { 0 } else { fields.get(&p).unwrap()%10 };
        }
    }

    res
}

pub fn part1(data:&Vec<String>,steps:i32)->i32
{  
    calc(data,steps,false)
}

pub fn part2(data:&Vec<String>)->i32
{
    calc(data,i32::MAX,true)
}

#[allow(unused)]
pub fn solve(data:&Vec<String>)
{    
    println!("Day11");
    println!("part1:{}",part1(data,100));
    println!("part2:{}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "5483143223".to_string(),
        "2745854711".to_string(),
        "5264556173".to_string(),
        "6141336146".to_string(),
        "6357385478".to_string(),
        "4167524645".to_string(),
        "2176841721".to_string(),
        "6882881134".to_string(),
        "4846848554".to_string(),
        "5283751526".to_string(),
    ];
    assert_eq!(part1(&v,10),204);
}

#[test]
fn test3()
{
    let v = vec![
        "5483143223".to_string(),
        "2745854711".to_string(),
        "5264556173".to_string(),
        "6141336146".to_string(),
        "6357385478".to_string(),
        "4167524645".to_string(),
        "2176841721".to_string(),
        "6882881134".to_string(),
        "4846848554".to_string(),
        "5283751526".to_string(),
    ];
    assert_eq!(part1(&v,100),1656);
}

#[test]
fn test2()
{
    let v = vec![
        "11111".to_string(),
        "19991".to_string(),
        "19191".to_string(),
        "19991".to_string(),
        "11111".to_string(),
    ];
    assert_eq!(part1(&v,3),9);
}