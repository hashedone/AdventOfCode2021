use std::collections::{HashMap};

fn get_point(s:&String)->(i32,i32)
{
    let p : Vec<_>= s.split(',').collect();
    (p[0].parse::<i32>().unwrap() , p[1].parse::<i32>().unwrap())
}

fn draw(data:&Vec<String>,diagonal:bool)->i32
{
    let mut field: HashMap<(i32,i32),i32> = HashMap::new();

    for l in data {
        let  v: Vec<_>  = l.split(" -> ").collect();
        let (p1_x,p1_y) = get_point(&v[0].to_string());
        let (p2_x,p2_y) = get_point(&v[1].to_string());

        let (dx,dy) = (i32::signum(p2_x-p1_x),i32::signum(p2_y-p1_y));
        let n = if !diagonal && dx!=0 && dy!=0 { 0 } 
                                          else { 1 + i32::max(i32::abs(p2_x-p1_x),i32::abs(p2_y-p1_y)) };

        let (mut x,mut y) = (p1_x,p1_y);
        for _ in 0..n 
        {
            let point = (x,y);
            field.insert(point, field.get(&point).unwrap_or(&0)+1 );            
            x+=dx;
            y+=dy;
        }
    }
    
    field.values()
         .filter(|&&v|v>1)
         .count() as i32
}

pub fn part1(data:&Vec<String>)->i32
{
    draw(data,false)
}

pub fn part2(data:&Vec<String>)->i32
{
    draw(data,true)
}

#[allow(unused)]
pub fn solve(data:&Vec<String>)
{    
    println!("Day5");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "0,9 -> 5,9".to_string(),
        "8,0 -> 0,8".to_string(),
        "9,4 -> 3,4".to_string(),
        "2,2 -> 2,1".to_string(),
        "7,0 -> 7,4".to_string(),
        "6,4 -> 2,0".to_string(),
        "0,9 -> 2,9".to_string(),
        "3,4 -> 1,4".to_string(),
        "0,0 -> 8,8".to_string(),
        "5,5 -> 8,2".to_string(),
    ];
    assert_eq!(part1(&v),5);
}

#[test]
fn test2()
{
    let v = vec![
        "0,9 -> 5,9".to_string(),
        "8,0 -> 0,8".to_string(),
        "9,4 -> 3,4".to_string(),
        "2,2 -> 2,1".to_string(),
        "7,0 -> 7,4".to_string(),
        "6,4 -> 2,0".to_string(),
        "0,9 -> 2,9".to_string(),
        "3,4 -> 1,4".to_string(),
        "0,0 -> 8,8".to_string(),
        "5,5 -> 8,2".to_string(),   
    ];
    assert_eq!(part2(&v),12);
}