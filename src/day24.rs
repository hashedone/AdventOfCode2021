extern crate rand;

use rand::Rng;
use std::collections::HashMap;


// not working yet


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

    /*
    fn valid(&self)->bool
    {
        self.get("z")==0
    }
     */

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

fn execute(data:&[String],number:&str)->i64
{
    let mut alu = Alu::new(number);
    
    
    for (_command,line) in data.iter().enumerate() {
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
    
        //println!("{}",line);
        //alu.print_state();

        if cmd[0]=="inp" { alu.read(); }
    }

    if alu.get("x")==0 && alu.get("y")==0 && alu.get("z")==0 
    {
        alu.print_state(number);
    }
    

//    alu.valid()
    alu.get("z")
}

pub fn part1(data:&[String])->u64
{
    let mut min_val = 11_111_111_111_111u64;
    let     max_val = 99_999_999_999_999u64;
    let mut best_number = 0u64;
    let mut rng = rand::thread_rng();

    let rand_n1: u32 = rng.gen();


//    let mut number: u64 = min_val + (rng.gen::<u64>()%delta);
    let mut number: u64 = max_val;// min_val;
    loop 
    {
        let delta = max_val-min_val+1;

        //87 744 899 768 809
        if number>=min_val && number<=max_val
        {
            let num = &number.to_string();


            if !num.contains('0')
            {
                //println!("> {}",number);
                //println!("{}",num);

                if execute(data, num)==0
                {
                    best_number = number;
                  //  println!("{}",best_number);
                    min_val = number+1;
                }   
              //  break;
                //println!("<");
                if number%100000==0
                {
                    println!("checked: {}",number);
                }
                    }
        }
        if best_number==max_val { break; }

        number-=1;
    }
    best_number
}

pub fn part2(_data:&[String])->u64
{
    0
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day24");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
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

    assert_eq!(execute(&v,"26111111111111"),1);
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

    assert_eq!(execute(&v,"27111111111111"),0);
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

    assert_eq!(execute(&v,"37111111111111"),1);
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

    assert_eq!(execute(&v,"27111111111111"),0);
}
/*

   inp w
   x =0
   y =6
   z=6    
   inp w
   
   add x z
   mod x 26
   div z 1
   add x 11
   eql x w
   eql x 0
   mul y 0
   add y 25
   mul y x
   add y 1
   mul z y
   mul y 0
   add y w
   add y 12
   mul y x
   add z y
   inp w
   mul x 0
   add x z
   mod x 26
   div z 1
   add x 10
   eql x w
   eql x 0
   mul y 0
   add y 25
   mul y x
   add y 1
   mul z y
   mul y 0
   add y w
   add y 5
   mul y x
   add z y
   inp w
   mul x 0
   add x z
   mod x 26
   div z 1
   add x 10
   eql x w
   eql x 0
   mul y 0
   add y 25
   mul y x
   add y 1
   mul z y
   mul y 0
   add y w
   add y 10
   mul y x
   add z y
   inp w
   mul x 0
   add x z
   mod x 26
   div z 26
   add x -16
   eql x w
   eql x 0
   mul y 0
   add y 25
   mul y x
   add y 1
   mul z y
   mul y 0
   add y w
   add y 7
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 14
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 0
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 12
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 4
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -4
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 12
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 15
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 14
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -7
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 13
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -8
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 10
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -4
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 11
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -15
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 9
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -8
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 9
mul y x
add z y

//y=(w+9)*x
//z = -

 */













/* 


w=i1
x=(i0+6)%26 + 11
z=i0+6
y=i0+6

eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 12
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 10
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 5
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 10
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 10
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -16
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 7
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 14
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 0
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 12
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 4
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -4
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 12
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 1
add x 15
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 14
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -7
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 13
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -8
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 10
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -4
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 11
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -15
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 9
mul y x
add z y
inp w
mul x 0
add x z
mod x 26
div z 26
add x -8
eql x w
eql x 0
mul y 0
add y 25
mul y x
add y 1
mul z y
mul y 0
add y w
add y 9
mul y x
add z y

*/