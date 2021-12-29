//use crate::tools;
use std::collections::HashMap;

struct Field
{
        dx : usize,
        dy : usize,
     field : Vec<Vec<char>>,
      amph : Vec<(char,i32,i32)>,
      xpos : HashMap<char,usize>,
final_code : usize,
      cost : HashMap<usize,usize>,
}

impl Field
{
    fn new(data:&[String])->Self
    {
        let dx = data[0].len();
        let dy = data.len();

        let mut res = Self {
                    dx,
                    dy,
                 field : vec![vec!['.';dx];dy],
                  amph : vec![],     
                  xpos : HashMap::new(),   
            final_code : Field::final_state(),
                  cost : HashMap::new(),
        };

        for (y_pos,y) in data.iter().enumerate()
        {
            for x in 0..dx
            {
                let c = y.chars().nth(x).unwrap_or(' ');
                res.field[y_pos][x] = c;
    
                if c.is_alphabetic() 
                {
                    res.amph.push((c,x as i32,y_pos as i32));
                }
            }                
        }

        res.xpos.insert('A', 3);
        res.xpos.insert('B', 5);
        res.xpos.insert('C', 7);
        res.xpos.insert('D', 9);
    
        res
    }
    //0.
    //1A
    //2B
    //3C
    //4D

    //  0123456789012
    // 0#############
    // 1#...........#
    // 2###C#A#B#D###
    // 3  #D#C#A#B#
    // 4  #########    
    fn code(&self,x:usize,y:usize)->usize
    {
        match self.field[y][x] {
            '.' => 0,
            'A' => 1,
            'B' => 2,
            'C' => 3,
            'D' => 4,
             _  => panic!("unknown code")
        }
    }

    fn get_cost(c:char,x0:i32,y0:i32,x1:i32,y1:i32)->usize
    {
        let moves = (x0-x1).abs() + (y0-y1).abs();
        
        let letter_cost : i32 = match c 
        {
            'A' => 1,
            'B' => 10,
            'C' => 100,
            'D' => 1000,
             _  =>  panic!("wrong letter")
        };
        (moves*letter_cost) as usize
    }

    fn get_code(&self)->usize
    {
        let mut res = 0usize;
        res = (res<<3) | self.code( 1,1);
        res = (res<<3) | self.code( 2,1);
        res = (res<<3) | self.code( 3,1);
        res = (res<<3) | self.code( 4,1);
        res = (res<<3) | self.code( 5,1);
        res = (res<<3) | self.code( 6,1);
        res = (res<<3) | self.code( 7,1);
        res = (res<<3) | self.code( 8,1);
        res = (res<<3) | self.code( 9,1);
        res = (res<<3) | self.code(10,1);
        res = (res<<3) | self.code(11,1);
        

        res = (res<<3) | self.code( 3,2);
        res = (res<<3) | self.code( 3,3);

        res = (res<<3) | self.code( 5,2);
        res = (res<<3) | self.code( 5,3);

        res = (res<<3) | self.code( 7,2);
        res = (res<<3) | self.code( 7,3);

        res = (res<<3) | self.code( 9,2);
        res = (res<<3) | self.code( 9,3);

        res
    }

    fn final_state()->usize
    {
        let mut res = 0usize;
        res = (res<<3) | 1;//A
        res = (res<<3) | 1;

        res = (res<<3) | 2;//B
        res = (res<<3) | 2;

        res = (res<<3) | 3;//C
        res = (res<<3) | 3;

        res = (res<<3) | 4;//D
        res = (res<<3) | 4;
        res
    }

    fn swap(field:&mut Vec<Vec<char>>,x0:i32,y0:i32,x1:i32,y1:i32)
    {
                let t = field[y0 as usize][x0 as usize];
        field[y0 as usize][x0 as usize] = field[y1 as usize][x1 as usize];
        field[y1 as usize][x1 as usize] = t;
        //std::mem::swap(x, y)
    }

    fn good_move(&self,c:char,x0:i32,y0:i32,x1:i32,y1:i32)->bool
    {
        if y1<1 || y1>3 { return false; }

        if x0==x1 
        {
            let matched = *self.xpos.get(&c).unwrap_or(&0)==x1 as usize;

            if y0==1
            {
                if !matched { return false; }
                
                if y1==3 
                {
                    return self.field[(y1-1) as usize][x1 as usize]=='.' && self.field[y1 as usize][x1 as usize]=='.';
                }
                if y1==2
                {
                    return self.field[y1 as usize][x1 as usize]=='.';
                } 
            }
            if y0==3
            {
                if matched { return false; }
                if y1>y0 { return false;}
                return self.field[(y1+1) as usize][x1 as usize]=='.' && self.field[y1 as usize][x1 as usize]=='.';
            }
        }
/*
        match y1 
        {
            1=> {
                if y0==1 
                {
                    return self.field[y1][x1]=='.';
                }
                else
                if y0==3 {
                    return self.field[2][x1]=='.' && self.field[y1][x1]=='.';
                }
                else {
                    return self.field[y1][x1]=='.';
                }
                
            },
            2=> {},
            3=> {},
            _=> panic!("error"),
        }
        */
        self.field[y1 as usize][x1 as usize]=='.'
    }

    fn print(&self)
    {
        for y in self.field.iter() {
            let yy = y.iter().collect::<String>();
            println!("{:?}",yy);
        }
        println!();
    }

/*
    fn move_ok(&self,x0:i32,y0:i32,x1:i32,y1:i32)->bool
    {
        if y0<y1
        {
            if x0>x1
            {
                for x in (x1..=x0-1).rev()
                {
                    if self.field[y0 as usize][x as usize]!='.' { return false; }
                }
                for y in (x1..=x0-1).rev()
                {
                    if self.field[y0 as usize][x1 as usize]!='.' { return false; }
                }
                
            }
            else {
                for x in x0+1..=x1 
                {
                    if self.field[y0 as usize][x as usize]!='.' { return false; }
                }
            }
            //2,3 des
            return false;
        }
        else {
            return false;
        }
        true
    }
   */  
}

fn go(depth:usize,max_depth:usize,best:&mut usize,f:&mut Field,amph:&mut Vec<(char,i32,i32)>,cost:usize)->usize
{
    //f.print();
    
    if  cost>=*best { return *best; }
    if depth> max_depth   { return *best; }


    let state = f.get_code();
    let mut res = *best;

    let current_cost = *f.cost.get(&state).unwrap_or(&best);
    if cost>current_cost { return current_cost; }

    f.cost.insert(state,cost);

    if state==f.final_code
    {
        println!("found:{}",cost);
        *best = cost;
        return cost;
    }
//    let moves = [(0,2),(0,1),(1,0),(-1,0),(0,-2),(0,-1)];
    let moves = [(0,1),(-1,0),(0,-1),(1,0)];

    for (id,(ch,x,y)) in amph.clone().iter().enumerate()
    {
        let x0 = *x;
        let y0 = *y;
        let c = *ch;

        if depth<2 {
            println!("depth:{} max:{} id:{}",depth,max_depth, id);
        }

        for mo in moves 
        {
            let x1 = x0 + mo.0;
            let y1 = y0 + mo.1;

            if f.good_move(c,x0,y0,x1,y1)
            {
                Field::swap(&mut f.field,x0, y0, x1, y1);
                let new_cost = cost + Field::get_cost(c,x0,y0,x1,y1);

                amph[id].1 = x1;
                amph[id].2 = y1;
                let nc = go(depth+1,max_depth,best,f,amph,new_cost);
                amph[id].1 = x0;
                amph[id].2 = y0;

                res = res.min(nc);

                Field::swap(&mut f.field,x0, y0, x1, y1);
            }
        }
    }
    
    res
}
/*
fn go_bfs(depth:usize,max_depth:usize,best:&mut usize,f:&mut Field,amph:&mut Vec<(char,i32,i32)>,cost:usize)->usize
{
    if  cost>=*best     { return usize::MAX; }
    if depth> max_depth { return usize::MAX; }

    //f.print();

    let state = f.get_code();
    let mut res = usize::MAX;

    let current_cost = *f.cost.get(&state).unwrap_or(&usize::MAX);
    if cost>current_cost { return current_cost; }

    f.cost.insert(state,cost);

    if state==f.final_code
    {
        println!("found:{}",cost);
        *best = cost;
        return cost;
    }

//    let moves = [(0,2),(0,1),(1,0),(-1,0),(0,-2),(0,-1)];
    let moves = [(0,1),(-1,0),(0,-1),(1,0)];

    for (id,(ch,x,y)) in amph.clone().iter().rev().enumerate()
    {
        let x0 = *x;
        let y0 = *y;
        let c = *ch;

        if depth<2 {
            println!("depth:{} max:{} id:{}",depth,max_depth, id);
        }

        for mo in moves 
        {
            let x1 = x0 + mo.0;
            let y1 = y0 + mo.1;

            if f.good_move(c,x0,y0,x1,y1)
            {
                Field::swap(&mut f.field,x0, y0, x1, y1);
                let new_cost = cost + Field::get_cost(c,x0,y0,x1,y1);
                //let mut new_amph = amph.clone();
                amph[id].1 = x1;
                amph[id].2 = y1;
                let nc = go(depth+1,max_depth,best,f,amph,new_cost);
                amph[id].1 = x0;
                amph[id].2 = y0;
                res = res.min(nc);

                Field::swap(&mut f.field,x0, y0, x1, y1);
            }
        }
    }
    
    res
}
 */

pub fn part1(data:&[String])->usize
{
    let mut f = Field::new(data);
    let mut am = f.amph.clone();
    let mut best = 15540;//usize::MAX;

    let mut max_depth=15;

    //let mut tab =vec![];
  //  loop 
//    {
    let ccc = go(0,7,&mut best,&mut f,&mut am,0);
    let ccc2 = go(0,max_depth,&mut best,&mut f,&mut am,0);
    //    max_depth+=1;

  //      if max_depth>100 {break;}
//    };
    
   
    //println!("{:?}",ccc);
   // go_bsf(0,max_depth,&mut best,&mut f,&mut am,0);
    best
}

pub fn part2(_data:&[String])->usize
{
0
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day23");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
"#############".to_string(),
"#A..........#".to_string(),
"###.#B#C#D###".to_string(),
"  #A#B#C#D#".to_string(),
"  #########".to_string(),
    ];
    assert_eq!(part1(&v),3);
}


#[test]
fn test2()
{
    let v = vec![
"#############".to_string(),
"#A..B.......#".to_string(),
"###.#.#C#D###".to_string(),
"  #A#B#C#D#".to_string(),
"  #########".to_string(),
    ];
    assert_eq!(part1(&v),23);
}


#[test]
fn test3()
{
    let v = vec![
"#############".to_string(),
"#...........#".to_string(),
"###D#B#C#A###".to_string(),
"  #A#B#C#D#".to_string(),
"  #########".to_string(),
    ];
    assert_eq!(part1(&v),8+2 +8000);
}
/*

#[test]
fn test3()
{
    let v = vec![
"#############".to_string(),
"#...........#".to_string(),
"###B#C#B#D###".to_string(),
"  #A#D#C#A#".to_string(),
"  #########".to_string(),
    ];
    assert_eq!(part1(&v),12521);
}
 */







/*
#############
#...........#
###C#A#B#D###
  #D#C#A#B#
  #########

#############
#.........D.#
###C#A#B#.###
  #D#C#A#B#
  #########  

  2d

#############
#.A.......D.#
###C#.#B#.###
  #D#C#A#B#
  #########  

  4a

#############
#.A.C.....D.#
###C#.#B#.###
  #D#.#A#B#
  #########  
 3c

#############
#.A.C.....D.#
###C#B#.#.###
  #D#B#A#.#
  #########  
 
5b 7b

#############
#.A.C.......#
###C#B#.#.###
  #D#B#A#D#
  #########  
3d

#############
#.A.C...A...#
###C#B#.#.###
  #D#B#.#D#
  #########  
3a

#############
#.A.....A...#
###.#B#C#.###
  #D#B#C#D#
  #########  
5c 6c

#############
#AA.........#
###.#B#C#.###
  #D#B#C#D#
  #########  
1a 6a 

#############
#AA.........#
###.#B#C#D###
  #.#B#C#D#
  #########  
9d

#############
#...........#
###A#B#C#D###
  #A#B#C#D#
  #########  
3a 3a




















20a
12b
14c
14d

14000
 1400
  120
   20

15540 too high
 */