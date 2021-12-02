pub fn part1(data:&Vec<String>)->i32
{
    let mut pos = 0;
    let mut depth = 0;
    
    for line in data {
        let cmd : Vec<&str> = line.split(' ').collect();
        let x   : i32       = cmd[1].parse().unwrap();

        match cmd[0] {
            "forward" => pos  +=x,
            "up"      => depth-=x,
            "down"    => depth+=x,
            _         => {},
        }
    }

    pos*depth
}

pub fn part2(data:&Vec<String>)->i32
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
pub fn solve(data:&Vec<String>)
{    
    println!("Day1");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "forward 5".to_string(),
        "down 5".to_string(),
        "forward 8".to_string(),
        "up 3".to_string(),
        "down 8".to_string(),
        "forward 2".to_string(),
    ];
    assert_eq!(part1(&v),150);
}

#[test]
fn test2()
{
    let v = vec![
        "forward 5".to_string(),
        "down 5".to_string(),
        "forward 8".to_string(),
        "up 3".to_string(),
        "down 8".to_string(),
        "forward 2".to_string(),
    ];
    assert_eq!(part2(&v),900);
}