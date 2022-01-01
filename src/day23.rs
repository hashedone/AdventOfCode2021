use std::collections::HashMap;

struct Field
{
       part1 : bool,
       field : Vec<Vec<char>>,
       ampho : Vec<(char,i32,i32)>,
        xpos : HashMap<char,usize>,
  final_code : u128,
        cost : HashMap<u128,usize>,       
}

impl Field
{
    fn new(data:&[String])->Self
    {
        let dx = data[0].len();
        let dy = data.len();
        let part1    = dy<7;

        let mut res = Self {
                 part1,
                 field : vec![vec!['.';dx];dy],
                 ampho : vec![],                    
                  xpos : HashMap::new(),   
            final_code : Field::final_state(part1),
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

        res.ampho.sort_unstable();

        res.xpos.insert('A', 3);
        res.xpos.insert('B', 5);
        res.xpos.insert('C', 7);
        res.xpos.insert('D', 9);
    
        res
    }
 
    fn code(&self,x:usize,y:usize)->u128
    {
        match self.field[y][x] 
        {
            '.' => 0,
            'A' => 1,
            'B' => 2,
            'C' => 3,
            'D' => 4,
             _  => panic!("unknown code {}",self.field[y][x])
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

    fn get_code(&self)->u128
    {
        let mut res = 0u128;
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

        if self.part1 
        {   
            res = (res<<3) | self.code( 5,2);
            res = (res<<3) | self.code( 5,3);
    
            res = (res<<3) | self.code( 7,2);
            res = (res<<3) | self.code( 7,3);
    
            res = (res<<3) | self.code( 9,2);
            res = (res<<3) | self.code( 9,3);
        }
          else
        {
            res = (res<<3) | self.code( 3,4);
            res = (res<<3) | self.code( 3,5);

            res = (res<<3) | self.code( 5,2);
            res = (res<<3) | self.code( 5,3);
            res = (res<<3) | self.code( 5,4);
            res = (res<<3) | self.code( 5,5);

            res = (res<<3) | self.code( 7,2);
            res = (res<<3) | self.code( 7,3);
            res = (res<<3) | self.code( 7,4);
            res = (res<<3) | self.code( 7,5);

            res = (res<<3) | self.code( 9,2);
            res = (res<<3) | self.code( 9,3);
            res = (res<<3) | self.code( 9,4);
            res = (res<<3) | self.code( 9,5);
        }

        res
    }

    fn final_state(part1:bool)->u128
    {
        let mut res = 0u128;
        res = (res<<3) | 1; //A
        res = (res<<3) | 1;

        if part1 
        {
            res = (res<<3) | 2; //B
            res = (res<<3) | 2;
    
            res = (res<<3) | 3; //C
            res = (res<<3) | 3;
   
        }
          else 
        {
            res = (res<<3) | 1;
            res = (res<<3) | 1;
    
            res = (res<<3) | 2; //B
            res = (res<<3) | 2;
            res = (res<<3) | 2;
            res = (res<<3) | 2;
    
            res = (res<<3) | 3; //C
            res = (res<<3) | 3;
            res = (res<<3) | 3;
            res = (res<<3) | 3;
    
            res = (res<<3) | 4; //D
            res = (res<<3) | 4;
        }

        res = (res<<3) | 4;
        res = (res<<3) | 4;
        res
    }

    fn swap(field:&mut Vec<Vec<char>>,x0:i32,y0:i32,x1:i32,y1:i32)
    {
                             let t = field[y0 as usize][x0 as usize];
        field[y0 as usize][x0 as usize] = field[y1 as usize][x1 as usize];
        field[y1 as usize][x1 as usize] = t;
    }

    fn clean_road(&self,x0:usize,x1:usize)->bool 
    {
        for x in usize::min(x0,x1) as usize+1..=usize::max(x0,x1) as usize-1 
        {
            if self.field[1][x]!='.' { return false; }
        }
        true
    }
    
    fn good_move(&self,c:char,x0:i32,y0:i32,x1:i32,y1:i32)->bool
    {
        if self.field[y1 as usize][x1 as usize]!='.' 
        {
            return false;
        }

        let lim5 = if self.part1 {3} else {5};
        //do not move if matches
        if y0>1 
        {
            let mut ok = true;
            
            for y in y0..=lim5 
            {
                let cc = self.field[y as usize][x0 as usize];
                if x0==3 && cc!='A' { ok = false; break; }
                if x0==5 && cc!='B' { ok = false; break; }
                if x0==7 && cc!='C' { ok = false; break; }
                if x0==9 && cc!='D' { ok = false; break; }
            }
            if ok { return false;}

            for y in 1..y0 
            {
                if self.field[y as usize][x0 as usize]!='.' { return false; }
            } 
        }

        if y0>1 && y1>1 { return false; }

        if y1>1 
        {
            if x1==3 && c!='A' { return false; }
            if x1==5 && c!='B' { return false; }
            if x1==7 && c!='C' { return false; }
            if x1==9 && c!='D' { return false; }
            
            for y in y1+1..=lim5
            {
                let cc = self.field[y as usize][x1 as usize];
                if x1==3 && cc!='A' { return false; }
                if x1==5 && cc!='B' { return false; }
                if x1==7 && cc!='C' { return false; }
                if x1==9 && cc!='D' { return false; }
            }

            for y in 1..=y1
            {
                if self.field[y as usize][x1 as usize]!='.' { return false; }
            }
        }
          else 
        {
            for y in 1..y0
            {
                let cc = self.field[y as usize][x0 as usize];
                if cc!='.' { return false; }
            }
        }

        self.field[y1 as usize][x1 as usize]=='.'
    }

    #[allow(unused)]
    fn print(&self,n:usize)
    {
        for y in self.field.iter() {
            let yy = y.iter().collect::<String>();
            println!("{:?}",yy);
        }
        println!("{}",n);
    }
}

fn get_possible_moves2(part1:bool,y:i32)->Vec<(i32,i32)>
{
    if y==1 {
        if part1 {
            vec![
                (3,3),
                (5,3),
                (7,3),
                (9,3),
                (3,2),
                (5,2),
                (7,2),
                (9,2),
                ]
        }
        else {
            vec![
                (3,5),(5,5),(7,5),(9,5),
                (3,4),(5,4),(7,4),(9,4),    
                (3,3),(5,3),(7,3),(9,3),
                (3,2),(5,2),(7,2),(9,2),
                ]    
        }
    }
        else 
    {
        vec![
             ( 1,1),
             ( 2,1),
             ( 4,1),
             ( 6,1),
             ( 8,1),
             (10,1),
             (11,1),
            ]        
    }
}    
 


fn go2(depth     : usize,
       max_depth : usize,
       best      : &mut usize,
       f         : &mut Field,
       amph      : &mut Vec<(char,i32,i32)>,      
       cost      : usize)
{
    if cost >*best      { return; }
    if depth> max_depth { return; }

    let state = f.get_code();
    
    if state==f.final_code
    {
        //f.print();
        if cost<*best
        {
            //println!("found:{}",cost);
            *best = cost;
        }
            
        return;
    }

    let current_cost = *f.cost.get(&state).unwrap_or(best);
    if cost>current_cost { return; }

    f.cost.insert(state,cost);


/*

        let good_moves:Vec<(char,usize,i32,i32,i32,i32)> = amph.into_iter()
                         .enumerate()
                         .map( |(id,(c,x0,y0))|
                            get_possible_moves2(*y0)
                            .into_iter()
                            .filter_map(|(x1,y1)|
                            {
                                if f.good_move(*c,*x0,*y0,x1,y1) && 
                                   f.clean_road(*x0 as usize,*y0 as usize,x1 as usize,y1 as usize)
                                {                                    
                                    Some((*c,id,*x0,*y0,x1,y1))
                                }
                                  else 
                                {
                                    None
                                }
                            }).collect::<Vec<(char,usize,i32,i32,i32,i32)>>()
                        )
                        .flatten()
                        .collect::<Vec<(char,usize,i32,i32,i32,i32)>>();
*/

            let mut good_moves = vec![];

            for (id,(c,x0,y0)) in amph.iter().enumerate()
            {
                for (x1,y1) in get_possible_moves2(f.part1,*y0)
                {
                    if f.good_move(*c,*x0,*y0,x1,y1) && f.clean_road(*x0 as usize,x1 as usize)
                    {
                        good_moves.push((*c,id,*x0,*y0,x1,y1));
                    }
                }
            }


    if good_moves.is_empty()
    {
        return;
    }

    let good_moves2 = good_moves.clone();

    for (c,ids,x0,y0,x1,y1) in good_moves2.into_iter()
    {
        Field::swap(&mut f.field,x0,y0,x1,y1);
        let new_cost = cost + Field::get_cost(c,x0,y0,x1,y1);
        
        amph[ids].1 = x1;
        amph[ids].2 = y1;
        go2(depth+1,max_depth,best,f,amph,new_cost);
        amph[ids].1 = x0;
        amph[ids].2 = y0;

        Field::swap(&mut f.field,x0,y0,x1,y1);
    }
}
















//Day23
//part1:15358
//part2:51436
//Elapsed: 4.7860003 secs



pub fn part1(data:&[String])->usize
{
    let mut f = Field::new(data);
    let mut am = f.ampho.clone();
    let mut best = 20_000;//15540;
    let max_depth= 16;
    
    go2(0,max_depth,&mut best,&mut f,&mut am,0);
    best
}




pub fn part2(data:&[String])->usize
{
    let mut f = Field::new(data);
    let mut am = f.ampho.clone();
    let mut best = 100_000;
    let max_depth = 40;
    go2(0,max_depth,&mut best,&mut f,&mut am,0);
    best
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day23");
    println!("part1:{}",part1(data));


    let data2 = vec![
        data[0].to_string(),
        data[1].to_string(),
        data[2].to_string(),

        "  #D#C#B#A#".to_string(),
        "  #D#B#A#C#".to_string(),

        data[3].to_string(),
        data[4].to_string(),
    ];

    println!("part2:{}",part2(&data2[..]));
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

#[test]
fn test_part2_2()
{
    let v = vec![
    "#############".to_string(),
    "#A..B.......#".to_string(),
    "###.#.#C#D###".to_string(),
    "  #A#B#C#D#".to_string(),
    "  #A#B#C#D#".to_string(),
    "  #A#B#C#D#".to_string(),
    "  #########".to_string(),
    ];
    assert_eq!(part1(&v),23);
}


#[test]
fn test_part1() 
{
    let v = vec![
    "#############".to_string(),
    "#...........#".to_string(),
    "###B#C#B#D###".to_string(),
    "  #A#D#C#A#".to_string(),
    "  #########".to_string(),
    ];

    assert_eq!(part1(&v), 12521);
}

#[test]
fn test_part2() {
 
 
    let v = vec![
        "#############".to_string(),
        "#...........#".to_string(),
        "###B#C#B#D###".to_string(),
        "  #D#C#B#A#".to_string(),
        "  #D#B#A#C#".to_string(),
        "  #A#D#C#A#".to_string(),
        "  #########".to_string(),
        ];
     
        assert_eq!(part2(&v), 44169);
}



#[test]
fn test_part3() {

    let v = vec![
    "#############".to_string(),
    "#CC.C.C.....#".to_string(),
    "###.#B#.#D###".to_string(),
    "  #A#B#.#D#".to_string(),
    "  #A#B#.#D#".to_string(),
    "  #A#B#A#D#".to_string(),
    "  #########".to_string(),
    ];
    assert_eq!(part2(&v), 2511);
}

#[test]
fn test_part1_() {

    let v = vec![
    "#############".to_string(),
    "#...........#".to_string(),
    "###C#A#B#D###".to_string(),
    "  #D#C#A#B#".to_string(),
    "  #########".to_string(),
    ];
    assert_eq!(part2(&v), 15358);
}

