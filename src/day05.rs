use std::collections::{HashMap};

fn get_point(s:&String)->(i32,i32)
{
    let p :Vec<_>= s.split(',').collect();
    (p[0].parse::<i32>().unwrap(),p[1].parse::<i32>().unwrap())
}

pub fn part1(data:&Vec<String>)->i32
{
    let mut field: HashMap<(i32,i32),i32> = HashMap::new();

    for l in data {
        let v:Vec<&str> = l.split(" -> ").collect();
        let p1 = get_point(&v[0].to_string());
        let p2 = get_point(&v[1].to_string());

        println!("p1 {}x{}",p1.0,p1.1);
        println!("p2 {}x{}",p2.0,p2.1);


        if p1.0==p2.0
        {
            for i in i32::min(p1.1,p2.1)..=i32::max(p1.1,p2.1) {
                let p = (p1.0,i);
                field.insert(p , field.get(&p).unwrap_or(&0)+1  );
            }
        }
        else if p1.1==p2.1
        {
            for i in i32::min(p1.0,p2.0)..=i32::max(p1.0,p2.0) {
                let p = (i,p1.1);
                field.insert(p , field.get(&p).unwrap_or(&0)+1  );
                
            }
        }

        
    }
    field.values().filter(|&&v|v>1).count() as i32
}

pub fn part2(data:&Vec<String>)->i32
{
    let mut field: HashMap<(i32,i32),i32> = HashMap::new();

    for l in data {
        let v:Vec<&str> = l.split(" -> ").collect();
        let p1 = get_point(&v[0].to_string());
        let p2 = get_point(&v[1].to_string());

        println!("p1 {}x{}",p1.0,p1.1);
        println!("p2 {}x{}",p2.0,p2.1);


        if p1.0==p2.0
        {
            for i in i32::min(p1.1,p2.1)..=i32::max(p1.1,p2.1) {
                let p = (p1.0,i);
                field.insert(p , field.get(&p).unwrap_or(&0)+1  );
            }
        }
        else if p1.1==p2.1
        {
            for i in i32::min(p1.0,p2.0)..=i32::max(p1.0,p2.0) {
                let p = (i,p1.1);
                field.insert(p , field.get(&p).unwrap_or(&0)+1  );
                
            }
        }
        else
        {
            if p1.0>p2.0
            {
                let t = p1;
                let p1 = p2;
                let p2 = t;
                let mut y = p1.1;
                for i in p1.0..=p2.0
                {
                    let p = (i,y);
                    field.insert(p , field.get(&p).unwrap_or(&0)+1  );
                    if p2.1>p1.1 { y+=1; }
                            else { y-=1; }
                }
            }
            else {
                let mut y = p1.1;
                for i in p1.0..=p2.0
                {
                    let p = (i,y);
                    field.insert(p , field.get(&p).unwrap_or(&0)+1  );
                    if p2.1>p1.1 { y+=1; }
                            else { y-=1; }
                }
                
            }
            //}
            //else {
              //  
            //}
        }
        
    }
    field.values().filter(|&&v|v>1).count() as i32
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
        "5,5 -> 8,2".to_string(),    ];
    assert_eq!(part2(&v),12);
}