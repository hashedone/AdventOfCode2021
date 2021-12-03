fn how_many(data:&Vec<String>, x:usize)->(i32,i32)
{
    data.iter()
        .fold((0,0), 
              |(zero,one),d| 
              if d.chars().nth(x).unwrap()=='0' { (zero+1,one  ) }
                                           else { (zero  ,one+1) } )
}

pub fn part1(data:&Vec<String>)->i32
{
    let mut gamma   = 0;
    let mut epsilon = 0;

    for x in 0..data[0].len()
    {
          gamma*=2;
        epsilon*=2;
        let cnt = how_many(data,x);
        if cnt.0>cnt.1 {   gamma|=1; }
                  else { epsilon|=1; }
    }

    gamma*epsilon
}

fn get_more(data:&Vec<String>,pos_x:usize,oxygen:bool)->i32
{
    if data.len()==1 
    {
        return i32::from_str_radix(&data[0].to_owned(), 2).unwrap();
    }

    let cnt = how_many(data,pos_x);
    let c   = if oxygen {  if cnt.0>cnt.1 {'0'} else {'1'}  }
                   else {  if cnt.0>cnt.1 {'1'} else {'0'}  };
   
    get_more(&data.iter()
                  .filter(|s| s.chars().nth(pos_x).unwrap()==c)
                  .map(|s| s.clone())
                  .collect::<Vec<String>>(),
                   pos_x+1,
                   oxygen)
}

pub fn part2(data:&Vec<String>)->i32
{
    let oxygen = get_more(data,0,false);
    let co2    = get_more(data,0,true );
    oxygen*co2
}

#[allow(unused)]
pub fn solve(data:&Vec<String>)
{    
    println!("Day3");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "00100".to_string(),
        "11110".to_string(),
        "10110".to_string(),
        "10111".to_string(),
        "10101".to_string(),
        "01111".to_string(),
        "00111".to_string(),
        "11100".to_string(),
        "10000".to_string(),
        "11001".to_string(),
        "00010".to_string(),
        "01010".to_string(),
    ];
    assert_eq!(part1(&v),198);
}

#[test]
fn test2()
{
    let v = vec![
        "00100".to_string(),
        "11110".to_string(),
        "10110".to_string(),
        "10111".to_string(),
        "10101".to_string(),
        "01111".to_string(),
        "00111".to_string(),
        "11100".to_string(),
        "10000".to_string(),
        "11001".to_string(),
        "00010".to_string(),
        "01010".to_string(),
    ];
    assert_eq!(part2(&v),230);
}