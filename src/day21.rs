use std::collections::HashMap;
use crate::tools;

fn compute1(data:&[String])->i64
{
    let p1: i64 = tools::i32_get_between(&data[0],"starting position: ","") as i64;  
    let p2: i64 = tools::i32_get_between(&data[1],"starting position: ","") as i64;  

    let mut pos    = 0;    
    let mut player = 1;  
    let mut score  = vec![0,0];
    let mut space  = vec![(p1+9)%10,(p2+9)%10];
    let mut count  = 0;
    
    while score[player]<1000 
    {
        count+=1;
        player=1-player;

        let mut scores =0;

        pos = (pos + 1)%100;        scores+=pos;
        pos = (pos + 1)%100;        scores+=pos;
        pos = (pos + 1)%100;        scores+=pos;

        space[player]+=scores;
        score[player]+=space[player]%10 + 1;
    }
    3*count*score[1-player]
}

fn compute2(h:&mut HashMap<(bool,u8,i64,i64,i64,i64),(i64,i64)>,player_one:bool,sum_dices:u8,p1:i64,p2:i64,s1:i64,s2:i64)->(i64,i64)
{
    let hash_key = &(player_one,sum_dices,p1,p2,s1,s2);
    if h.contains_key(hash_key)
    {
        return *h.get(hash_key).unwrap();
    }

    let mut s1n = s1;
    let mut s2n = s2;
    let mut p1n = p1;
    let mut p2n = p2;
    
    if player_one
    { 
        p1n = p1 + sum_dices as i64;
        while p1n>10 { p1n-=10; }
    }
      else 
    { 
        p2n = p2 + sum_dices as i64;
        while p2n>10 { p2n-=10; }
    }

    if sum_dices>0
    {
        if player_one { s1n += p1n; }
                 else { s2n += p2n; }

        if s1n>=21 { return (1,0); }
        if s2n>=21 { return (0,1); }            
    }

    let mut res = (0,0);

    for d1 in 1..=3 {
        for d2 in 1..=3 {
            for d3 in 1..=3 {
                let r = compute2(h,!player_one,d1+d2+d3,p1n,p2n,s1n,s2n); 
                res.0+=r.0;
                res.1+=r.1;
            }
        } 
    }
    
    h.insert(*hash_key, res);
    res
}

pub fn part1(data:&[String])->i64
{
    compute1(data)
}

pub fn part2(data:&[String])->i64
{
    let p1: i64 = tools::i32_get_between(&data[0],"starting position: ","") as i64;
    let p2: i64 = tools::i32_get_between(&data[1],"starting position: ","") as i64;

    let mut h = HashMap::new();
    let res = compute2(&mut h,false,0,p1,p2,0,0);    
    res.0.max(res.1)
}


#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day21");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "Player 1 starting position: 4".to_string(),
        "Player 2 starting position: 8".to_string(),
    ];
    assert_eq!(part1(&v),739785);
}

#[test]
fn test2()
{
    let v = vec![
        "Player 1 starting position: 4".to_string(),
        "Player 2 starting position: 8".to_string(),
    ];
    assert_eq!(part2(&v),444356092776315);
}

#[test]
fn test3()
{
    let v = vec![
        "Player 1 starting position: 6".to_string(),
        "Player 2 starting position: 3".to_string(),
    ];
    assert_eq!(part2(&v),309196008717909);
}