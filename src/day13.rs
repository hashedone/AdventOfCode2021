use std::collections::{HashSet};

fn fold(horiz:bool,val:i32,f:&mut HashSet<(i32,i32)>)
{
    if !horiz { //y=
        let yy = val;

        for (x,y) in f.clone().iter() {
            if *y>yy {
                let np = (*x,yy - (y-yy));
                f.insert(np);
            }
        }

        for (x,y) in f.clone().iter() {
            if *y>=yy { f.remove(&(*x,*y)); }
        }
    }
    else {
        let xx =val;

        for (x,y) in f.clone().iter() {
            if *x>xx {
                let np = (xx - (x-xx),*y);
                f.insert(np);
            }
        }

        for (x,y) in f.clone().iter() {
            if *x>=xx {
                f.remove(&(*x,*y));
            }
        }
        
    }
   
}

pub fn part1(data:&[String])->i32
{
    let mut f : HashSet<(i32,i32)> = HashSet::new();
    let mut v : Vec<(bool,i32)>    = Vec::new();

    for line in data 
    {
        if line.contains("fold along ")
        {
            let pos : Vec<&str> = line.split("fold along ").collect();    
            let fol : Vec<&str> = pos[1].split("=").collect();
            
            v.push((fol[0]=="x",fol[1].parse().unwrap()));
        }
        else if line.len()>0 
        {
            let pos : Vec<&str> = line.split(',').collect();    
            let xx = pos[0].parse().unwrap();
            let yy = pos[1].parse().unwrap();
            f.insert((xx,yy));
        }
    }

    fold(v[0].0,v[0].1,&mut f);

    println!("{:?}",v);
    println!("{:?}",f);

 
    f.len() as i32
}

pub fn part2(data:&[String])
{
    let mut f : HashSet<(i32,i32)> = HashSet::new();
    let mut v : Vec<(bool,i32)>    = Vec::new();

    for line in data 
    {
        if line.contains("fold along ")
        {
            let pos : Vec<&str> = line.split("fold along ").collect();    
            let fol : Vec<&str> = pos[1].split("=").collect();
            
            //println!("fol=[{:?}]",line);
            //println!("fol=[{:?}]",fol);/

            if fol[0]=="x"
            {
                v.push((true ,fol[1].parse().unwrap()));
            }
            else if fol[0]=="y"
            {
                v.push((false,fol[1].parse().unwrap()));
            }
        }
        else if line.len()>0 
        {
            let pos : Vec<&str> = line.split(',').collect();    
            let xx = pos[0].parse().unwrap();
            let yy = pos[1].parse().unwrap();
            f.insert((xx,yy));
        }
    }

        for cmd in v {
            fold(cmd.0,cmd.1,&mut f);
        }

        for y in 0..10 {
        for x in 0..40 {
                print!("{}", if f.contains(&(x,y)) {'█'} else {' '} );
            }
            println!("");
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
