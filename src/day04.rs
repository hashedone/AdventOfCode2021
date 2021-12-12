use std::collections::{HashSet};

const N : usize = 5;

struct Board
{
    used : HashSet<i32>,
    tab  : Vec<Vec<i32>>,
}

impl Board
{
    fn new()->Self
    {
        Self {
            used : HashSet::new(),
            tab  : vec![vec![0;N];N],
        }        
    }

    fn from_vec(s:Vec<String>)->Self
    {
        let mut board = Board::new();
        board.fill(s);
        board
    }

    fn fill(&mut self,s:Vec<String>)
    {
        for (y,l) in s.iter().enumerate() 
        {
            let num = get_numbers(l);
            for (id,&v) in num.iter().enumerate() 
            {
                self.tab[y][id] = v;
            }
        }
    }

    fn bingo(&mut self,n:i32)->bool 
    {
        self.used.insert(n);

        for i in 0..N 
        {
            if (0..N).into_iter()
                     .filter(|&x| self.used.contains(&self.tab[i][x]))
                     .count()==N { return true } 

            if (0..N).into_iter()
                     .filter(|&y| self.used.contains(&self.tab[y][i]))
                     .count()==N { return true } 
        }
      
        false
    }

    fn sum(&self)->i32 
    {
        self.tab
            .iter()
            .flatten()
            .filter(|x|!self.used.contains(x))
            .sum::<i32>()
    }
}

fn get_numbers(s:&str)->Vec<i32>
{
    s.split_whitespace()
     .map(|s| s.parse::<i32>().unwrap())
     .collect::<Vec<i32>>()
}

fn prepare_data(data:&[String])->(Vec<i32>,Vec<Board>)
{
    let        num = get_numbers(&data[0].replace(','," "));
    let mut boards = vec![];

    for i in (2..data.len()).step_by(N+1)
    {
        boards.push(Board::from_vec(data[i..i+N].to_vec()));
    }
    (num,boards)
}

pub fn part1(data:&[String])->i32
{
    let (num,mut boards) = prepare_data(data);

    for v in num 
    {
        for b in boards.iter_mut() 
        {
            if b.bingo(v) { return v*b.sum(); }
        }
    }
    0
}

pub fn part2(data:&[String])->i32
{
    let (num,mut boards) = prepare_data(data);

    let count   = boards.len();
    let mut won = HashSet::new();

    for v in num 
    {
        for (id,b) in boards.iter_mut().enumerate()
        {
            if b.bingo(v) 
            {
                won.insert(id);
                if won.len()==count { return v*b.sum(); }
            }
        }
    }
    0
}

#[allow(unused)]
pub fn solve(data:&[String])
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