
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

fn oper(id:u64,acum:Option<i64>,val:i64)->i64
{
    match id {
        0 => if acum==None { val } else {    acum.unwrap() +   val              },
        1 => if acum==None { val } else {    acum.unwrap() *   val              },        
        2 => if acum==None { val } else {    acum.unwrap().min(val)             },        
        3 => if acum==None { val } else {    acum.unwrap().max(val)             },
        5 => if acum==None { val } else { if acum.unwrap() >   val {1} else {0} },
        6 => if acum==None { val } else { if acum.unwrap() <   val {1} else {0} },
        7 => if acum==None { val } else { if acum.unwrap()==   val {1} else {0} },
        _ => {0},
    }
}

fn parse_packet1(bin:&String,id:usize)->(u64,usize)
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
                
                let r = parse_packet1(bin,i);
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
                let r = parse_packet1(bin,i);
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

fn parse_packet2(bin:&str,id:usize)->(u64,usize)
{
    let mut i = id;
    let vers = &bin[i..i+ 3];
    let ver = vector_to_binary(vers);
    let ids = &bin[i+3..i+ 6];
    let id = vector_to_binary(ids);
    i+=6; 

    let mut res = 0u64;

    res+=ver;

    if id!=4 //&&!literal
    {       
        let lt_id= vector_to_binary(&bin[i..i+1]);
      
        i+=1;

        if lt_id==1
        {
            let number_sub = vector_to_binary(&bin[i..i+11]);
            i+=11;
          //  println!("number_sub:{}",number_sub);

            let mut acc: Option<i64> = None;
          
            for _ in 0..number_sub {
               // println!("{}",vid);
                
                let r = parse_packet2(bin,i);
                //println!("r.0={}",r.0);
                acc = Some(oper(id, acc,r.0 as i64));
                i=r.1;
            }
            res-=ver;
            res = ( acc.unwrap_or(0)) as u64;
           // println!("res:{}",res);
        }
          else 
        {
            let number_len = vector_to_binary(&bin[i..i+15]) as usize ;
            i+=15;
         //   println!("number_len:{}",number_len);
            let limit = i+number_len;

            let mut acc: Option<i64> = None;

            while i<limit {
                let r = parse_packet2(bin,i);

              //  println!("r.0={}",r.0);
                acc = Some(oper(id, acc,r.0 as i64));

                res+=r.0;
                i=r.1;   
            }
            res = ( acc.unwrap_or(0)) as u64;
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
        let mut _accumlator=0i64;
   
        let sv = vector_to_binary(&ss.to_owned());
        res = sv;
      //  println!("{}",sv);
    }

    (res,i)

}

pub fn part1(data:&[String])->u64
{
    parse_packet1(&string_to_binary(&data[0]),0).0
}

pub fn part2(data:&[String])->u64
{
    parse_packet2(&string_to_binary(&data[0]),0).0
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day16");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

/*
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
    assert_eq!(part1(&v),6);
}
 */

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

#[test]
fn test2_1()
{
    let v = vec!["C200B40A82".to_string()];
    assert_eq!(part2(&v),3);
}
#[test]
fn test2_2()
{
    let v = vec!["04005AC33890".to_string()];
    assert_eq!(part2(&v),54);
}

#[test]
fn test2_3()
{
    let v = vec!["880086C3E88112".to_string()];
    assert_eq!(part2(&v),7);
}

#[test]
fn test2_4()
{
    let v = vec!["CE00C43D881120".to_string()];
    assert_eq!(part2(&v),9);
}

#[test]
fn test2_5()
{
    let v = vec!["D8005AC2A8F0".to_string()];
    assert_eq!(part2(&v),1);
}

#[test]
fn test2_6()
{
    let v = vec!["F600BC2D8F".to_string()];
    assert_eq!(part2(&v),0);
}

#[test]
fn test2_7()
{
    let v = vec!["9C005AC2F8F0".to_string()];
    assert_eq!(part2(&v),0);
}

#[test]
fn test2_8()
{
    let v = vec!["9C0141080250320F1802104A08".to_string()];
    assert_eq!(part2(&v),1);
}


