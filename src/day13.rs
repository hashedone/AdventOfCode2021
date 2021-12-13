use std::collections::{HashSet};

fn fold(horiz:bool,val:i32,f:&mut HashSet<(i32,i32)>)
{
    if !horiz {
        for (x,y) in f.clone().iter() {
            if *y >val { f.insert( (*x,val - (y-val))); }
            if *y>=val { f.remove(&(*x,           *y)); }
        }
    }
      else 
    {
        for (x,y) in f.clone().iter() {
            if *x> val { f.insert( (val - (x-val),*y)); }
            if *x>=val { f.remove(&(           *x,*y)); }
        }
    }
}

fn parse(data:&[String],field:&mut HashSet<(i32,i32)>,folds:&mut Vec<(bool,i32)>)
{
    for line in data 
    {
        if line.contains("fold along ")
        {
            let pos : Vec<&str> = line.split("fold along ").collect();    
            let fol : Vec<&str> = pos[1].split('=').collect();
            
            folds.push((fol[0]=="x",fol[1].parse().unwrap()));
        }
        else if !line.is_empty()
        {
            let pos = line.split(',').collect::<Vec<&str>>();    
            let xx  = pos[0].parse().unwrap();
            let yy  = pos[1].parse().unwrap();

            field.insert((xx,yy));
        }
    }
}

pub fn part1(data:&[String])->usize
{
    let mut f : HashSet<(i32,i32)> = HashSet::new();
    let mut v : Vec<(bool,i32)>    = Vec::new();

    parse(data,&mut f,&mut v);
    fold(v[0].0,v[0].1,&mut f);
 
    f.len()
}

pub fn part2(data:&[String])
{
    let mut f : HashSet<(i32,i32)> = HashSet::new();
    let mut v : Vec<(bool,i32)>    = Vec::new();

    parse(data,&mut f,&mut v);

    for (horizontal,point) in v {
        fold(horizontal,point,&mut f);
    }

    let (xm,ym) = (f.iter().map(|(x,_)| *x ).max().unwrap(),
                   f.iter().map(|(_,y)| *y ).max().unwrap());

    for y in 0..=ym {
    for x in 0..=xm {
            print!("{}", if f.contains(&(x,y)) {'█'} else {' '} );
        }
        println!();
    }
}


#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day13");
    println!("part1:{}",part1(data));
    println!("part2:");
    part2(data);
}

#[test]
fn test1_1()
{
    let v = vec![
        "6,10".to_string(),
        "0,14".to_string(),
        "9,10".to_string(),
        "0,3".to_string(),
        "10,4".to_string(),
        "4,11".to_string(),
        "6,0".to_string(),
        "6,12".to_string(),
        "4,1".to_string(),
        "0,13".to_string(),
        "10,12".to_string(),
        "3,4".to_string(),
        "3,0".to_string(),
        "8,4".to_string(),
        "1,10".to_string(),
        "2,14".to_string(),
        "8,10".to_string(),
        "9,0".to_string(),
        "".to_string(),
        "fold along y=7".to_string(),
        "fold along x=5".to_string(),
    ];
    assert_eq!(part1(&v),17);
}
