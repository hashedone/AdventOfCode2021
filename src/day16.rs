
fn string_to_binary(table:&str)->String
{
    let mut res = vec![];

    for c in table.chars()
    {                
        let s =         
        match c 
        {
            '0' => "0000",
            '1' => "0001",
            '2' => "0010",
            '3' => "0011",
            '4' => "0100",
            '5' => "0101",
            '6' => "0110",
            '7' => "0111",
            '8' => "1000",
            '9' => "1001",
            'A' => "1010",
            'B' => "1011",
            'C' => "1100",
            'D' => "1101",
            'E' => "1110",
            'F' => "1111",
             _  => panic!("error unknown code"),
        };

        res.push(s);
    }   

    res.join("")
}

fn vector_to_binary(table:&str)->u64
{
    table.chars().fold(0,|acc,x| ((acc<<1) | x.to_digit(10).unwrap() as u64))
}

fn parse_packet(bin:&String,id:usize)->(u64,usize)
{
    let mut i = id;

   
    let vers = &bin[i+ 0..i+ 3];
    let ver = vector_to_binary(vers);
    let ids = &bin[i+ 3..i+ 6];
    let id = vector_to_binary(ids);
    i+=6; 

    let mut res = 0u64;

    res+=ver;

    if id!=4 //&&!literal
    {       
        let lt_id= vector_to_binary(&bin[i..i+1]);
        let mut number_sub=0;
        let mut number_len=0;
        i+=1;

        if lt_id==1
        {
            number_sub = vector_to_binary(&bin[i..i+11]);
            i+=11;
          //  println!("number_sub:{}",number_sub);

            for vid in 0..number_sub {
               // println!("{}",vid);
                
                let r = parse_packet(bin,i);
                res+=r.0;
                i=r.1;
            }
        }
          else 
        {
            number_len = vector_to_binary(&bin[i..i+15]) as usize ;
            i+=15;
         //   println!("number_len:{}",number_len);
            let limit = i+number_len;

            while i<limit {
                let r = parse_packet(bin,i);
                res+=r.0;
                i=r.1;   
            }
        }
    }
      else 
    {
        let mut ss ="".to_string();
        let mut bit_one = true;
    
        while bit_one 
        {
            bit_one = bin.chars().nth(i).unwrap()=='1';
            i+=1;
            ss.push_str(&bin[i..i+4]);
            i+=4;
        }
   
        let sv = vector_to_binary(&ss.to_owned());
    }

    (res,i)

}

fn parse(data:&String)->u64
{
    let bin = string_to_binary(data);
    parse_packet(&bin,0).0
}

pub fn part1(data:&[String])->u64
{
    parse(&data[0])
}

pub fn part2(data:&[String])->u64
{
    parse(&data[0])
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day16");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}


#[test]
fn test0()
{
    let v = vec![
        "D2FE28".to_string(),
        ];
    assert_eq!(part1(&v),2021);
}

#[test]
fn test01()
{
    let v = vec![
        "38006F45291200".to_string(),
        ];
    assert_eq!(part1(&v),31);
}

#[test]
fn test02()
{
    let v = vec![
        "EE00D40C823060".to_string(),
        ];
    assert_eq!(part1(&v),13);
}


#[test]
fn test1()
{
    let v = vec![
        "8A004A801A8002F478".to_string(),
        ];
    assert_eq!(part1(&v),16);
}

#[test]
fn test2()
{
    let v = vec![
        "620080001611562C8802118E34".to_string(),
       ];
    assert_eq!(part1(&v),12);
}

#[test]
fn test3()
{
    let v = vec![
        "C0015000016115A2E0802F182340".to_string(),
       ];
    assert_eq!(part1(&v),23);
}

#[test]
fn test4()
{
    let v = vec![
        "A0016C880162017C3686B18A3D4780".to_string(),
        ];
    assert_eq!(part1(&v),31);
}
