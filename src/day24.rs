use std::collections::HashMap;

struct Alu {
    vals   : HashMap<char,i64>,
    number : String,
    id     : usize
}

impl Alu
{
    fn new(s:&str)->Self
    {
        let mut r = Self {
            vals   : HashMap::new(),
            number : s.to_string(),
            id     : 0,
        };
        r.vals.insert('x',0);
        r.vals.insert('y',0);
        r.vals.insert('z',0);
        r.vals.insert('w',0);
        r
    }

    fn set(&mut self,v:&str,val:i64)
    {
        let vc = v.chars().next().unwrap();
        self.vals.insert(vc,val);
    }

    fn get(&self,v:&str)->i64
    {
        let c = v.chars().next().unwrap();
        if c=='-' || c.is_digit(10)
        {
            v.parse::<i64>().unwrap()
        }
          else
        {            
            *self.vals.get(&c).unwrap()
        }
    }

    #[allow(unused)]
    fn valid(&self)->bool
    {
        self.get("z")==0
    }

    fn read(&mut self)->i64
    {
        let res = self.number.chars().nth(self.id).unwrap().to_digit(10).unwrap();
        self.id+=1;
        res as i64
    }

    fn number(&self)->i64
    {
        self.number.chars().nth(self.id).unwrap().to_digit(10).unwrap() as i64
    }

    fn print_state(&self,number:&str)
    {
        println!("{} = x:{} y:{} z:{} w:{}",number,self.get("x"),self.get("y"),self.get("z"),self.get("w"))
    }
}

#[allow(unused)]
fn execute(data:&[String],number:&str,break_line:usize)->i64
{
    let mut alu = Alu::new(number);
    
    
    for (command,line) in data.iter().enumerate() {
        let cmd : Vec<&str> = line.split(' ').collect();
        let p1 = cmd[          1];
        let p2 = cmd[cmd.len()-1];

        match cmd[0] {
            "inp" => alu.set(p1, alu.number()),
            "add" => alu.set(p1, alu.get(p1) + alu.get(p2)),
            "mul" => alu.set(p1, alu.get(p1) * alu.get(p2)),
            "div" => 
            {
                if alu.get(p2)==0 { return -1;}
                alu.set(p1, alu.get(p1) / alu.get(p2))
            }
            "mod" => 
            {
                if alu.get(p1)< 0 { return -1; }
                if alu.get(p2)<=0 { return -1; }
                alu.set(p1, alu.get(p1) % alu.get(p2))
            }
            "eql" => alu.set(p1, (alu.get(p1)==alu.get(p2)) as i64),
            _     => panic!("unknown command"),
        }
    
        if command==break_line
        {
            alu.print_state(number);
            return alu.get("z");
        }
        //println!("{}",line);
        //

        if cmd[0]=="inp" { alu.read(); }
    }

    if alu.get("x")==0 && alu.get("y")==0 && alu.get("z")==0 
    {
        alu.print_state(number);
    }
    
//    alu.valid()
    alu.get("z")
}


// inp w    inp w    inp w    inp w     inp w     inp w    inp w    inp w    inp w    inp w    inp w    inp w    inp w     inp w
// mul x 0  mul x 0  mul x 0  mul x 0   mul x 0   mul x 0  mul x 0  mul x 0  mul x 0  mul x 0  mul x 0  mul x 0  mul x 0   mul x 0
// add x z  add x z  add x z  add x z   add x z   add x z  add x z  add x z  add x z  add x z  add x z  add x z  add x z   add x z
// mod x 26 mod x 26 mod x 26 mod x 26  mod x 26  mod x 26 mod x 26 mod x 26 mod x 26 mod x 26 mod x 26 mod x 26 mod x 26  mod x 26
// div z 1  div z 1  div z 1  div z 1   div z 26  div z 1  div z 1  div z 26 div z 1  div z 26 div z 26 div z 26 div z 26  div z 26
// add x 12 add x 11 add x 10 add x 10  add x -16 add x 14 add x 12 add x -4 add x 15 add x -7 add x -8 add x -4 add x -15 add x -8
// eql x w  eql x w  eql x w  eql x w   eql x w   eql x w  eql x w  eql x w  eql x w  eql x w  eql x w  eql x w  eql x w   eql x w
// eql x 0  eql x 0  eql x 0  eql x 0   eql x 0   eql x 0  eql x 0  eql x 0  eql x 0  eql x 0  eql x 0  eql x 0  eql x 0   eql x 0
// mul y 0  mul y 0  mul y 0  mul y 0   mul y 0   mul y 0  mul y 0  mul y 0  mul y 0  mul y 0  mul y 0  mul y 0  mul y 0   mul y 0
// add y 25 add y 25 add y 25 add y 25  add y 25  add y 25 add y 25 add y 25 add y 25 add y 25 add y 25 add y 25 add y 25  add y 25
// mul y x  mul y x  mul y x  mul y x   mul y x   mul y x  mul y x  mul y x  mul y x  mul y x  mul y x  mul y x  mul y x   mul y x
// add y 1  add y 1  add y 1  add y 1   add y 1   add y 1  add y 1  add y 1  add y 1  add y 1  add y 1  add y 1  add y 1   add y 1
// mul z y  mul z y  mul z y  mul z y   mul z y   mul z y  mul z y  mul z y  mul z y  mul z y  mul z y  mul z y  mul z y   mul z y
// mul y 0  mul y 0  mul y 0  mul y 0   mul y 0   mul y 0  mul y 0  mul y 0  mul y 0  mul y 0  mul y 0  mul y 0  mul y 0   mul y 0
// add y w  add y w  add y w  add y w   add y w   add y w  add y w  add y w  add y w  add y w  add y w  add y w  add y w   add y w
// add y 6  add y 12 add y 5  add y 10  add y 7   add y 0  add y 4  add y 12 add y 14 add y 13 add y 10 add y 11 add y 9   add y 9
// mul y x  mul y x  mul y x  mul y x   mul y x   mul y x  mul y x  mul y x  mul y x  mul y x  mul y x  mul y x  mul y x   mul y x
// add z y  add z y  add z y  add z y   add z y   add z y  add z y  add z y  add z y  add z y  add z y  add z y  add z y   add z y

fn calc(hash:&mut HashMap<(i32,usize),bool>,pos:usize,yo:i32,zo:i32,delta:i32)->bool
{
    if hash.contains_key(&(zo,pos))
    {
        return *hash.get(&(zo,pos)).unwrap();
    }

    if pos==14 {
        return zo==0;
    }

    let div_z = [ 1  , 1  , 1  , 1  , 26  , 1  , 1  , 26 , 1  , 26 , 26 , 26 ,  26 , 26];
    let add_x = [ 12 , 11 , 10 , 10 , -16 , 14 , 12 , -4 , 15 , -7 , -8 , -4 , -15 , -8];
    let add_y = [ 6  , 12 , 5  , 10 , 7   , 0  , 4  , 12 , 14 , 13 , 10 , 11 ,  9  ,  9];

    let mut digit = if delta>0 {1} else {9};

//    for digit in (1..=9).into_iter().rev()
    
    while (0..=9).contains(&digit)
    {
       // let mut x;// = xo;
        let mut y = yo;
        let mut z = zo;
        let     w = digit as i32;

        //x*=0;
        //x+=z;
        //x%=26;
        let mut x=z%26;
        z/=div_z[pos];
        x+=add_x[pos];
        x = if x==w {1} else {0};
        x = 1-x;//if x==0 {1} else {0};
        y*=0;
        y+=25;
        y*=x;
        y+=1;
        z*=y;
        y*=0;
        y+=w;
        y+=add_y[pos];
        y*=x;
        z+=y;

        if calc(hash,pos+1,y,z,delta)
        {
            print!("{}",digit);
            hash.insert((zo,pos), true);
            return true;
        }
        digit+=delta;
    }

    hash.insert((zo,pos), false);
    false
}


pub fn part1(_data:&[String])
{
    let mut h = HashMap::new();
    calc(&mut h,0,0,0,-1);
}

pub fn part2(_data:&[String])
{
    let mut h = HashMap::new();
    calc(&mut h,0,0,0,1);
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    //99893999291967
    //34171911181211
    println!("Day24");
    print!("part1: reverse("); part1(data); println!(")");
    print!("part2: reverse("); part2(data); println!(")");
}

#[test]
fn test1()
{
    let v = vec![
        "inp z".to_string(),
        "inp x".to_string(),
        "mul z 3".to_string(),
        "eql z x".to_string(),
    ];

    assert_eq!(execute(&v,"26111111111111",usize::MAX),1);
}

#[test]
fn test2()
{
    let v = vec![
        "inp z".to_string(),
        "inp x".to_string(),
        "mul z 3".to_string(),
        "eql z x".to_string(),
    ];

    assert_eq!(execute(&v,"27111111111111",usize::MAX),0);
}



#[test]
fn test3()
{
    let v = vec![
        "inp w".to_string(),
        "add z w".to_string(),
        "mod z 2".to_string(),
        "div w 2".to_string(),
        "add y w".to_string(),
        "mod y 2".to_string(),
        "div w 2".to_string(),
        "add x w".to_string(),
        "mod x 2".to_string(),
        "div w 2".to_string(),
        "mod w 2".to_string(),
    ];

    assert_eq!(execute(&v,"37111111111111",usize::MAX),1);
}


#[test]
fn test4()
{
    let v = vec![
        "inp w".to_string(),
        "add z w".to_string(),
        "mod z 2".to_string(),
        "div w 2".to_string(),
        "add y w".to_string(),
        "mod y 2".to_string(),
        "div w 2".to_string(),
        "add x w".to_string(),
        "mod x 2".to_string(),
        "div w 2".to_string(),
        "mod w 2".to_string(),
    ];

    assert_eq!(execute(&v,"27111111111111",usize::MAX),0);
}
