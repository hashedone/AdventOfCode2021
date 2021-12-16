fn string_to_binary(table:&str)->String
{
    table.chars()
         .map(|c|
             format!("{:04b}", c.to_digit(16).unwrap()) )
         .collect::<String>()
}

fn vector_to_binary(table:&str)->u64
{
    table.chars()
         .fold(0,|acc,x| ((acc<<1) | x.to_digit(10).unwrap() as u64))
}

fn oper(id:u64,acum:Option<i64>,val:i64)->i64
{
    match id {
        0 =>    acum.unwrap_or(0) +     val                                    ,
        1 =>    acum.unwrap_or(1) *     val                                    ,
        2 =>    acum.unwrap_or(val).min(val)                                   ,
        3 =>    acum.unwrap_or(val).max(val)                                   ,
        5 => if acum==None { val } else if acum.unwrap() >val { 1 } else { 0 } ,
        6 => if acum==None { val } else if acum.unwrap() <val { 1 } else { 0 } ,
        7 => if acum==None { val } else if acum.unwrap()==val { 1 } else { 0 } ,
        _ => panic!("unknown id")
    }
}

fn parse_packet1(bin:&str,id:usize)->(u64,usize)
{
    let mut i = id;
    let ver = num_from_bits(bin,&mut i,3);
    let id  = num_from_bits(bin,&mut i,3);
    let mut res = ver;

    if id!=4
    {       
        let lt_id = num_from_bits(bin,&mut i,1);

        if lt_id==1
        {
            let number_sub = num_from_bits(bin,&mut i,11);

            for _ in 0..number_sub 
            {
                let r = parse_packet1(bin,i);
                res+=r.0;
                i   =r.1;
            }
        }
          else 
        {
            let number_len = num_from_bits(bin,&mut i,15);
            let limit      = i + number_len as usize;

            while i<limit 
            {
                let r = parse_packet1(bin,i);
                res+= r.0;
                i   = r.1;   
            }
        }
    }
      else 
    {
        loop
        {
            i+=5;
            if bin.chars().nth(i-5).unwrap()!='1' { break; }
        }
    }

    (res,i)
}

fn num_from_bits(bin:&str,i:&mut usize,num:usize)->u64
{   
    let res = vector_to_binary(&bin[*i..*i + num]);   
    *i+=num; 
    res
}

fn parse_packet2(bin:&str,id:usize)->(u64,usize)
{
    let mut i = id;
    let ver = num_from_bits(bin,&mut i,3);
    let id = num_from_bits(bin,&mut i,3);

    let mut res = ver;

    if id!=4
    {       
        let lt_id = num_from_bits(bin,&mut i,1);

        if lt_id==1
        {
            let number_sub = num_from_bits(bin,&mut i,11);
            let mut acc: Option<i64> = None;
          
            for _ in 0..number_sub 
            {
                let r = parse_packet2(bin,i);
                acc = Some(oper(id, acc,r.0 as i64));
                i=r.1;
            }

            res = ( acc.unwrap_or(0)) as u64;
        }
          else 
        {
            let number_len = num_from_bits(bin,&mut i,15);
            let limit = i + number_len as usize;
            let mut acc: Option<i64> = None;

            while i<limit 
            {
                let r = parse_packet2(bin,i);

                acc = Some(oper(id, acc,r.0 as i64));
                res+=r.0;
                i=r.1;   
            }
            res = (acc.unwrap_or(0)) as u64;
        }
    }
      else 
    {
        let mut buffer ="".to_string();
        let mut bit_one = true;
    
        while bit_one 
        {
            bit_one = bin.chars().nth(i).unwrap()=='1'; i+=1;
            buffer.push_str(&bin[i..i+4]);  i+=4;
        }
        
        res = vector_to_binary(&buffer.to_owned());    
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
