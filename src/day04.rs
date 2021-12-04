use std::collections::{HashSet};

struct Board
{
    hash : HashSet<i32>,
    tab  : Vec<Vec<i32>>,
}

impl Board
{
    fn new()->Self
    {
        Board {
            hash : HashSet::new(),
            tab  : vec![vec![0;5];5],
        }        
    }

    fn fill(&mut self,s:Vec<String>)
    {
        for (y,l) in s.iter().enumerate() {
            let num = get_num(&l,' ');
            for (id,&v) in num.iter().enumerate() {
                self.tab[y][id] = v;
            }
        }
    }

    fn bingo(&mut self, n:i32)->bool 
    {
        self.hash.insert(n);

        for y in 0..5 
        {
            let mut cnt=0;
            for x in 0..5 
            {
                if self.hash.contains(&self.tab[y][x]) {cnt+=1}                
            }    
            if cnt==5 {return true;}
        }

        for y in 0..5 
        {
            let mut cnt=0;
            for x in 0..5 
            {
                if self.hash.contains(&self.tab[x][y]) {cnt+=1}                
            }    
            if cnt==5 {return true;}
        }
        
        false
    }

    fn sum(&self)->i32 {
        self.tab
            .iter()
            .flatten()
            .filter(|x|!self.hash.contains(x))
            .sum::<i32>()
    }
}

fn get_num(s:&String,c:char)->Vec<i32>
{
    s.split_whitespace()
     .map(|s| s.parse::<i32>().unwrap())
     .collect::<Vec<i32>>()
}

pub fn part1(data:&Vec<String>)->i32
{
    let num = get_num(&data[0].replace(','," "),',');
    println!("{:?}",num);
    let mut boards = vec![];

    for i in 2..data.len() {
        if (i-2)%6==0 {
            let mut b = Board::new();
            b.fill([data[i].clone(),data[i+1].clone(),data[i+2].clone(),data[i+3].clone(),data[i+4].clone()].to_vec());
            boards.push(b);
            
        }
    }

    for v in num {
        for (id,b) in boards.iter_mut().enumerate(){
            if b.bingo(v) {
                println!("{} {} {}",id,v,b.sum());
                return v*b.sum();
            }
        }
    }
    0
}

pub fn part2(data:&Vec<String>)->i32
{
    let num = get_num(&data[0].replace(','," "),',');
    let mut boards = vec![];

    for i in 2..data.len() {
        if (i-2)%6==0 {
            let mut b = Board::new();
            b.fill([data[i].clone(),data[i+1].clone(),data[i+2].clone(),data[i+3].clone(),data[i+4].clone()].to_vec());
            boards.push(b);
            
        }
    }
    let count= boards.len();
    let mut won = HashSet::new();

    for v in num {
        for (id,b) in boards.iter_mut().enumerate(){
            if b.bingo(v) {
                won.insert(id);
                if won.len()==count {
                    return v*b.sum();
                }
            }
        }
        
    }
0
}

#[allow(unused)]
pub fn solve(data:&Vec<String>)
{    
    println!("Day4");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1".to_string(),
        "".to_string(),
        "22 13 17 11  0".to_string(),
        " 8  2 23  4 24".to_string(),
        "21  9 14 16  7".to_string(),
        " 6 10  3 18  5".to_string(),
        " 1 12 20 15 19".to_string(),
        "".to_string(),
        " 3 15  0  2 22".to_string(),
        " 9 18 13 17  5".to_string(),
        "19  8  7 25 23".to_string(),
        "20 11 10 24  4".to_string(),
        "14 21 16 12  6".to_string(),
        "".to_string(),
        "14 21 17 24  4".to_string(),
        "10 16 15  9 19".to_string(),
        "18  8 23 26 20".to_string(),
        "22 11 13  6  5".to_string(),
        " 2  0 12  3  7".to_string(),
    ];
    assert_eq!(part1(&v),4512);
}

#[test]
fn test2()
{
    let v = vec![
        "7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1".to_string(),
        "".to_string(),
        "22 13 17 11  0".to_string(),
        " 8  2 23  4 24".to_string(),
        "21  9 14 16  7".to_string(),
        " 6 10  3 18  5".to_string(),
        " 1 12 20 15 19".to_string(),
        "".to_string(),
        " 3 15  0  2 22".to_string(),
        " 9 18 13 17  5".to_string(),
        "19  8  7 25 23".to_string(),
        "20 11 10 24  4".to_string(),
        "14 21 16 12  6".to_string(),
        "".to_string(),
        "14 21 17 24  4".to_string(),
        "10 16 15  9 19".to_string(),
        "18  8 23 26 20".to_string(),
        "22 11 13  6  5".to_string(),
        " 2  0 12  3  7".to_string(),
    ];
    assert_eq!(part2(&v),1924);
}