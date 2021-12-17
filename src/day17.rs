use std::collections::HashSet;
use crate::tools;

fn parse(data:&str)->(i32,i32,i32,i32)
{
    let xx = tools::get_between(data,"x=",", ");
    let yy = tools::get_between(data,"y=","");
    
    let x0 = tools::i32_get_between(&xx[..],  "","..");    
    let x1 = tools::i32_get_between(&xx[..],"..",  "");
    let y0 = tools::i32_get_between(&yy[..],  "","..");    
    let y1 = tools::i32_get_between(&yy[..],"..",  "");  
    
    ( x0,x1,y0,y1 )
}

pub fn compute(data:&str)->(i32,usize)
{
    let (x0,x1,y0,y1) = parse(data);
    let mut hash : HashSet<(i32,i32)> = HashSet::new();
    let mut res = i32::MIN;
    
    for vy in -1000..1000
    {
        for vx in -1000..1000
        {            
            let  mut max_y = i32::MIN;
            let (mut vel_x,mut vel_y) = (vx,vy);
            let (mut pos_x,mut pos_y) = ( 0, 0);

            loop
            {
                max_y = max_y.max(pos_y);
                pos_x+=vel_x;
                pos_y+=vel_y;

                if vel_x>0 { vel_x-=1; }
                if vel_x<0 { vel_x+=1; }
                             vel_y-=1; 
                
                if pos_x>=x0 && pos_x<=x1 &&
                   pos_y>=y0 && pos_y<=y1
                {
                    hash.insert((vx,vy));
                    res = res.max(max_y);
                    break;
                }

                if pos_y<y0 && vel_y<0 { break; }
                if pos_x>x1 && vel_x>0 { break; }
                if pos_x<x0 && vel_x<0 { break; }
            }
        }
    }

    (res,hash.len())
}

pub fn part1(data:&[String])->i32
{
    compute(&data[0]).0
}

pub fn part2(data:&[String])->usize
{
    compute(&data[0]).1
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day17");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "target area: x=20..30, y=-10..-5".to_string()
    ];
    assert_eq!(part1(&v),45);
}

#[test]
fn test2()
{
    let v = vec![
        "target area: x=20..30, y=-10..-5".to_string()
    ];
    assert_eq!(part2(&v),112);
}

#[test]
fn test3()
{
    let v = vec![
        "target area: x=282..314, y=-80..-45".to_string()
    ];
    assert_eq!(part1(&v),3160);
}

#[test]
fn test4()
{
    let v = vec![
        "target area: x=282..314, y=-80..-45".to_string()
    ];
    assert_eq!(part2(&v),1928);
}