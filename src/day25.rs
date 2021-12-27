struct Sea 
{
    dx    : usize,
    dy    : usize,
    field : Vec<Vec<char>>
}

impl Sea
{
    fn new(data:&[String])->Self
    {
        let dx = data[0].len();
        let dy = data.len();
        let mut res = Self {
            dx,  
            dy,
            field : vec![vec!['.';dx];dy]
        };

        for y in 0..dy
        {
            for x in 0..dx
            {
                res.field[y][x] = data[y].chars().nth(x).unwrap();
            }                
        }
        res
    }

    fn get_x(&self,x:i32)->usize
    {
        ((x+self.dx as i32) as usize) % self.dx
    }

    fn get_y(&self,y:i32)->usize
    {
        ((y+self.dy as i32) as usize) % self.dy
    }

    fn get(&self,x:usize,y:usize)->char
    {
        self.field[y][x]
    }

    fn set(&mut self,x:usize,y:usize,c:char)
    {
        self.field[y][x] = c;
    }

    fn move_x(&mut self)->bool
    {
        let mut moved = false;
        for y in 0..self.dy
        {
            let mut to_move = vec![];

            for x in 0..self.dx
            {
                let prev = self.get_x(self.dx as i32+x as i32    );
                let act  = self.get_x(self.dx as i32+x as i32 + 1);
                if self.get(prev, y)=='>' && self.get(act, y)=='.' {
                    to_move.push(x);
                }
            }    

            if to_move.len()>0 
            {
                moved   = true;

                for x in to_move 
                {
                    let prev = self.get_x(self.dx as i32+x as i32    );
                    let act  = self.get_x(self.dx as i32+x as i32 + 1);
                    self.set(prev, y, '.');
                    self.set(act , y, '>');    
                }
            }            
        }
        moved
    }


    fn move_y(&mut self)->bool
    {
        let mut moved = false;
        for x in 0..self.dx
        {
            moved = false;
            let mut to_move = vec![];

            for y in 0..self.dy
            {
                let prev = self.get_y(self.dy as i32+y as i32    );
                let act  = self.get_y(self.dy as i32+y as i32 + 1);
                if self.get(x,prev)=='v' && self.get(x,act)=='.' {
                    to_move.push(y);
                }
            }      

            if to_move.len()>0 
            {
                moved = true;   

                for y in to_move
                {
                    let prev = self.get_y(self.dy as i32+y as i32    );
                    let act  = self.get_y(self.dy as i32+y as i32 + 1);
                    self.set(x,prev, '.');
                    self.set(x,act , 'v');    
                }
            }
        }
        moved
    }

    fn print(&self,round:i32)
    {
        println!("round:{}",round);

        for y in 0..self.dy
        {
            for x in 0..self.dx
            {
                print!("{}",self.field[y][x]);
            }
            println!();
        }
        println!();
    }

    fn get_safe_round(&mut self)->i32
    {
        let mut round = 0;
        let mut moved = true;
        
        self.print(round);

        while moved
        {
            moved = false;
            
            moved|=self.move_x();
            moved|=self.move_y();
            
            round+=1;
        }
        
        round
    }
}

pub fn part1(data:&[String])->i32
{
    Sea::new(data).get_safe_round()
}

pub fn part2(data:&[String])->i32
{
0
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day25");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "v...>>.vv>".to_string(),
        ".vv>>.vv..".to_string(),
        ">>.>v>...v".to_string(),
        ">>v>>.>.v.".to_string(),
        "v>v.vv.v..".to_string(),
        ">.>>..v...".to_string(),
        ".vv..>.>v.".to_string(),
        "v.v..>>v.v".to_string(),
        "....v..v.>".to_string(),
    ];
    assert_eq!(part1(&v),58);
}
