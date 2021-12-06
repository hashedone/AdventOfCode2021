fn count(data:&Vec<String>,iterations:i32)->i64
{
    let mut tab : Vec<(i32,usize)> = data[0].split(',')
                                            .map(|d| (d.parse().unwrap(),1) )
                                            .collect();

    for _ in 0..iterations
    {
        let count_new = tab.iter()
                           .fold(0, |sum,(x,count)| if x==&0 {sum + count} else {sum});

        tab = tab.into_iter()
                 .map(|(value,count)|
                      if value==0 { (      6,count) }
                             else { (value-1,count) } )
                 .collect();

        tab.push((8,count_new));
    }

    tab.into_iter()
       .fold(0i64, |sum,(_,c)| sum + c as i64)
}

#[allow(unused)]
pub fn solve(data:&Vec<String>)
{    
    println!("Day6");
    println!("part1:{}",count(data, 80));
    println!("part2:{}",count(data,256));
}

#[test]
fn test0()
{
    let v = vec![
        "3,4,3,1,2".to_string()
    ];
    assert_eq!(count(&v,18),26);
}
#[test]
fn test1()
{
    let v = vec![
        "3,4,3,1,2".to_string()
    ];
    assert_eq!(count(&v,80),5934);
}

#[test]
fn test2()
{
    let v = vec![
        "3,4,3,1,2".to_string()
    ];
    assert_eq!(count(&v,256),26984457539);
}