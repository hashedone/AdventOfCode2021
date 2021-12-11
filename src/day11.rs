use std::collections::{HashMap,HashSet};

pub fn calc(data:&Vec<String>,steps:i32,part_two:bool)->i32
{
    let dx = data[0].len();
    let dy = data.len();

    let mut f : HashMap<(i32,i32),u32> = HashMap::new();

    for y in 0..dy {
        for x in 0..dx {
            f.insert((x as i32,y as i32),
                     data[y as usize].chars().nth(x as usize).unwrap().to_digit(10).unwrap() );
        }
    }
    
    let mut res = 0;

    for step in 1..=steps {
        let mut flash = true;
        let mut flushed = HashSet::new();

        for y in 0..dy {
            for x in 0..dx {
                let p = (x as i32,y as i32);
                let v = *f.get(&p).unwrap_or(&0);
                f.insert(p, v+1);
            }
        }

        while flash 
        {
            flash = false;

            for y in 0..dy {
                for x in 0..dx {
                    let p = (x as i32,y as i32);
                    let v = *f.get(&p).unwrap_or(&0);

                    if v>9
                    {
                        if !flushed.contains(&p)
                        {
                            flushed.insert(p);
                            res+=1;
                            flash = true;

                            for yy in -1..=1 {
                            for xx in -1..=1 {    
                            if xx!=0 || yy!=0                    
                            {
                                let pn = (x as i32 + xx,y as i32 + yy);
                                let nv = *f.get(&pn).unwrap_or(&0) + 1;
                                f.insert(pn, nv );                
                            }
                            }
                            }
                        }
                    }
                }   
            }
        }

        if part_two && flushed.len()==dx*dy { return step;}

        for y in 0..dy {
        for x in 0..dx {
            let p = (x as i32,y as i32);
            let v = *f.get(&p).unwrap();
            f.insert(p,if flushed.contains(&p) {0} else {v%10} );
        }}
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