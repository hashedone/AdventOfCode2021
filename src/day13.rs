pub fn part1(_data:&[String])->i32
{
    /*
    let mut pos = 0;
    let mut depth = 0;
    
    for line in data {
        let cmd : Vec<&str> = line.split(' ').collect();
        let x   : i32       = cmd[1].parse().unwrap();
    }
 */
    -1
}

pub fn part2(_data:&[String])->i32
{
    -1
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day13");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn test1_1()
{
    let v = vec![
    ];
    assert_eq!(part1(&v),-1);
}

#[test]
fn test2_1()
{
    let v = vec![
    ];
    assert_eq!(part2(&v),-1);
}