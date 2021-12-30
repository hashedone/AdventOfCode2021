use std::collections::HashMap;

struct Field
{
       field : Vec<Vec<char>>,
       ampho : Vec<(char,i32,i32)>,
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
                 field : vec![vec!['.';dx];dy],
                 ampho : vec![],                    
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
                    res.ampho.push((c,x as i32,y_pos as i32));
                }
            }                
        }

        res.ampho.sort();
        println!("{:?}",res.ampho);

        res.xpos.insert('A', 3);
        res.xpos.insert('B', 5);
        res.xpos.insert('C', 7);
        res.xpos.insert('D', 9);
    
        res
    }
 
    fn code(&self,x:usize,y:usize)->usize
    {
        match self.field[y][x] 
        {
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

    fn clean_road(&self,x0:usize,y0:usize,x1:usize,y1:usize)->bool {
        for x in usize::min(x0,x1) as usize+1..=usize::max(x0,x1) as usize-1 {
            if self.field[1][x]!='.' {return false;}
        }
        for y in usize::min(y0,y1) as usize+1..=usize::max(y0,y1) as usize-1 {
            if y1==1 {
                if self.field[y][x0 as usize]!='.' { return false; }
            }
            else {
                if self.field[y][x1 as usize]!='.' { return false; }
            }
        }

        if y0==3 && self.field[y0-1][x0 as usize]!='.' { return false; }
        if y0==2 && self.field[y0-1][x0 as usize]!='.' { return false; }
        true
    }
    
    fn good_move(&self,c:char,x0:i32,y0:i32,x1:i32,y1:i32)->bool
    {
        if y1<1 || y1>3 { return false; }
        if y0<1 || y0>3 { return false; }

        if x0==x1 { return false; }
        if y0==y1 { return false; }

        if self.field[y0 as usize][x0 as usize]=='.' 
        { 
            //self.print(999);
            //println!("{},{}->{},{}",x0,y0,x1,y1); 
            panic!("xy0");
            //return false;             
        }

        if y0==3 {
            if x0==3 && c=='A' { return false; }
            if x0==5 && c=='B' { return false; }
            if x0==7 && c=='C' { return false; }
            if x0==9 && c=='D' { return false; }
        }

        if y1>1 
        {
            if x1==3 && c!='A' { return false; }
            if x1==5 && c!='B' { return false; }
            if x1==7 && c!='C' { return false; }
            if x1==9 && c!='D' { return false; }
        }
        else 
        {
            if y0==3 && self.field[(y0-1) as usize][x0 as usize]!='.' { return false; }
            if y0==3 && self.field[(y0-2) as usize][x0 as usize]!='.' { return false; }
            if y0==2 && self.field[(y0-1) as usize][x0 as usize]!='.' { return false; }
        }

        if y1==2 && self.field[3 as usize][x1 as usize]!='.'
        {
            if self.field[3 as usize][x1 as usize]!=c { return false;}
        }

        if y0==1 && y1==2 {
            if self.field[3 as usize][x1 as usize]=='.' { return false; }
        }

        if self.field[y1 as usize][x1 as usize]!='.' { return false; }

        if x0==x1 
        {
            let matched = *self.xpos.get(&c).unwrap()==x1 as usize;

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

        self.field[y1 as usize][x1 as usize]=='.'
    }

    fn print(&self,n:usize)
    {
        for y in self.field.iter() {
            let yy = y.iter().collect::<String>();
            println!("{:?}",yy);
        }
        println!("{}",n);
    }
}

fn get_possible_moves(y:i32)->Vec<(i32,i32)>
{
    if y==3 || y==2 {
        return vec![( 1,1),
                    ( 2,1),
                    ( 4,1),
                    ( 6,1),
                    ( 8,1),
                    (10,1),
                    (11,1),
                    ];
    }
    if y==1 {
        return vec![(3,3),
                    (5,3),
                    (7,3),
                    (9,3),
                    (3,2),
                    (5,2),
                    (7,2),
                    (9,2),
                    ];
    }
    panic!("wat?");
    vec![]
}

fn go(depth     : usize,
      max_depth : usize,
      best      : &mut usize,
      f         : &mut Field,
      amph      : &mut Vec<(char,i32,i32)>,
      moves     : &mut HashMap<usize,Vec<(char,usize,i32,i32,i32,i32)>>,
      cost      : usize)
{
    if cost >=*best      { return; }
    if depth>  max_depth { return; }

//  if depth==2 {f.print(); return *best;}
//  if depth>  2 { return *best; }

    let state = f.get_code();
    //let mut res = *best;

    let current_cost = *f.cost.get(&state).unwrap_or(&best);
    if cost>=current_cost { return; }

    f.cost.insert(state,cost);

    if state==f.final_code
    {
        println!("found:{}",cost);
        //f.print();
        *best = cost;
        return;
    }

    if depth<3 {
        println!("depth:{} max:{}  moves:{}",depth,max_depth, f.cost.len());
    }

    let mo = moves.get(&state);

   // if mo==None
    {
        let mut good_moves = vec![];

        for id in 0..amph.len()
        {
            let c = amph[id].0; //*ch;
            let x0 = amph[id].1; //*x;
            let y0 = amph[id].2; //;*y;

            for mo in get_possible_moves(y0)
            {
                let x1 = mo.0;
                let y1 = mo.1;

                if f.good_move(c,x0,y0,x1,y1) && f.clean_road(x0 as usize,y0 as usize,x1 as usize,y1 as usize)
                {
                    if f.field[y1 as usize][x1 as usize]!='.' { panic!("1"); }
                    if f.field[y0 as usize][x0 as usize]=='.' { panic!("2"); }
                    good_moves.push((c,id,x0,y0,x1,y1));
                }
            }
        }
        //if moves.get(&state)!=None {
          //  panic!("none");
        //}
        moves.insert(state, good_moves);
    }

    if moves.get(&state).unwrap().len()==0 
    {
        return;
    }

    let good_moves2 = moves.get(&state).unwrap().clone();

    for (num,(c,ids,x0,y0,x1,y1)) in good_moves2.into_iter().enumerate()
    {
        ///f.print(num);

        let ox0 = x0;
        let ox1 = x1;
        let oy0 = y0;
        let oy1 = y1;

        Field::swap(&mut f.field,ox0,oy0,ox1,oy1);
        //f.print(num);

        let new_cost = cost + Field::get_cost(c,x0,y0,x1,y1);

        let ol1 = amph[ids].1;
        let ol2 = amph[ids].2;

        if x0!=ol1 || y0!=ol2 
        {
            println!("error id:{} letter:{}",ids,c);
            f.print(55);
            println!("{:?}",  amph);

            println!("{} {}",x0 ,y0 );
            println!("{} {}",ol1,ol2);
            panic!("e");
        }        
        

        if f.field[amph[ids].2 as usize][amph[ids].1 as usize]!='.'
        {
            panic!("eeeeeee");
        }

        amph[ids].1 = x1;
        amph[ids].2 = y1;
        go(depth+1,max_depth,best,f,amph,moves,new_cost);        
        amph[ids].1 = ol1;
        amph[ids].2 = ol2;

        //res = res.min(nc);

        //Field::swap(&mut f.field,x0,y0,x1,y1);
        Field::swap(&mut f.field,ox0,oy0,ox1,oy1);
    }
    
    
}

pub fn part1(data:&[String])->usize
{
    let mut f = Field::new(data);
    let mut am = f.ampho.clone();
    let mut best = 15540;//usize::MAX;

//    let mut max_depth=14; nothing found

//depth:2 max:16  moves:4922724
//part1:14019
//Elapsed: 585.809 secs

//let mut max_depth=16;
    let mut max_depth=16;
//       15540 too high
//       15378 too high
//       15358
// found:15022
// found:15020
// found:14041
// found:14021
// found:14019 too low

    //let mut tab =vec![];
  //  loop 
//    {
    //let ccc = go(0,7,&mut best,&mut f,&mut am,0);

    let mut moves: HashMap<usize,Vec<(char,usize,i32,i32,i32,i32)>> = HashMap::new();
    let ccc2 = go(0,max_depth,&mut best,&mut f,&mut am,&mut moves,0);
    //    max_depth+=1;

  //      if max_depth>100 {break;}
//    };
    
   
    //println!("{:?}",ccc);
   // go_bsf(0,max_depth,&mut best,&mut f,&mut am,0);
    best
}

pub fn part2(_data:&[String])->usize
{
    let mut f = Field::new(data);
    let mut am = f.ampho.clone();
    let mut best = 15540;//usize::MAX;

    let mut max_depth=32;
    let mut moves: HashMap<usize,Vec<(char,usize,i32,i32,i32,i32)>> = HashMap::new();
    go(0,max_depth,&mut best,&mut f,&mut am,&mut moves,0);
    best
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


#[test]
fn testb4()
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

#[test]
fn test_part2_1()
{
    let v = [
    "#############".to_string(),
    "#...........#".to_string(),
    "###B#C#B#D###".to_string(),
    "  #D#C#B#A#".to_string(),
    "  #D#B#A#C#".to_string(),
    "  #A#D#C#A#".to_string(),
    "  #########".to_string()
    ];
    assert_eq!(part2(&v),44169);
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

17538
 */