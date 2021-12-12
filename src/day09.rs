fn get_point(field:&[Vec<i32>],x:i32,y:i32)->i32
{
    if x<0 || y<0 { return 9; }
    if x>=field[0].len() as i32 || y>=field.len() as i32 { return 9; }
    
    field[y as usize][x as usize]
}

fn get_field(data:&[String])->(Vec<Vec<i32>>,i32,i32)
{
    let dx = data[0].len();
    let dy = data.len();
    
    let mut field = vec![vec![0;dx];dy];

    for y in 0..dy {
    for x in 0..dx {
        field[y][x] = data[y].chars().nth(x).unwrap() as i32 - '0' as i32;
    }
    }
    (field,dx as i32,dy as i32)
}

fn calc(data:&[String])->Vec<(i32,i32)>
{
    let (field,dx,dy) = get_field(data);
    let mut res = vec![];

    for y in 0..dy  {
    for x in 0..dx  {
        let v = get_point(&field,x,y);

        if  get_point(&field,x-1,y  )>v &&
            get_point(&field,x+1,y  )>v &&
            get_point(&field,x  ,y-1)>v &&
            get_point(&field,x  ,y+1)>v { res.push((x as i32,y as i32)); }
    }
    }    

    res
}


pub fn part1(data:&[String])->i32
{
    let (field,_,_) = get_field(data);
    calc(data).iter()
              .map(|(x,y)| get_point(&field,*x,*y)+1)
              .sum()
}

fn fill_field(data:&mut Vec<Vec<i32>>,x:i32,y:i32)->i32
{
    let v = get_point(data, x, y);
    let mut res = 0;

    if v<9
    {
        res+=1;       
        data[y as usize][x as usize] = 9;

        let p1 = get_point(data, x, y+1);
        let p2 = get_point(data, x, y-1);
        let p3 = get_point(data, x-1, y);
        let p4 = get_point(data, x+1, y);

        if p1>v && p1<9 { res+=fill_field(data,x  ,y+1); }
        if p2>v && p2<9 { res+=fill_field(data,x  ,y-1); }
        if p3>v && p3<9 { res+=fill_field(data,x-1,y  ); }
        if p4>v && p4<9 { res+=fill_field(data,x+1,y  ); }
    }

    res
}


pub fn part2(data:&[String])->i32
{
    let (mut field,_,_) = get_field(data);
    let mut big = calc(data).iter()
                             .map(|(x,y)| fill_field(&mut field,*x,*y))
                             .collect::<Vec<i32>>();
    big.sort_unstable();
    big[big.len()-1]*big[big.len()-2]*big[big.len()-3]
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day9");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "2199943210".to_string(),
        "3987894921".to_string(),
        "9856789892".to_string(),
        "8767896789".to_string(),
        "9899965678".to_string(),
    ];
    assert_eq!(part1(&v),15);
}

#[test]
fn test2()
{
    let v = vec![
        "2199943210".to_string(),
        "3987894921".to_string(),
        "9856789892".to_string(),
        "8767896789".to_string(),
        "9899965678".to_string(),
    ];
    assert_eq!(part2(&v),1134);
}