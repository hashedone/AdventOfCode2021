use std::collections::{HashMap};

fn get_codes(codes:&HashMap<String,char>,a:char,b:char)->char
{
    let cc:String = [a.to_string(),b.to_string()].join("") as String;
        
    if codes.contains_key(&cc) {
        return codes[&cc]
    }
    ' '
}

fn compact(t:&Vec<(char,i64)>)->Vec<(char,i64)>
{
    let mut res:Vec<(char,i64)> = Vec::new();
    let mut last = ' ';

    res.push(t[0]);
    last = t[0].0;

    for i in 1..t.len()
    {
        if last==t[i].0 
        {
            res.last_mut().unwrap().1+=t[i].1;
            //res[res.len()-1].1 += 
        }
          else
        {
            res.push(t[i]);
            last = t[i].0;
        }
    }
   // res.push(t[t.len()-1]);
    res
}

//N(2)C(1)B(1)
//N(1)C(1)N(1)B(1)C(1)H(1)B(1)

fn get_v(res:&mut Vec<(char,i64)>,codes:&HashMap<String,char>,a:(char,i64),b:(char,i64))
{ 
    let aa = get_codes(codes,a.0,a.0);
    if a.1>1 && aa!=' '
    {
        for _ in 0..a.1-1 {
            res.push((a.0,1));
            res.push(( aa,1));   
        }
        res.push((a.0,1));
    }
    else {
        res.push(a);
    }

    let ab = get_codes(codes,a.0,b.0);
    if ab!=' ' { res.push((ab,1)); }
/*
    let bb = get_codes(codes,b.0,b.0);
    if b.1>1 && bb!=' '
    {
        for _ in 0..b.1-1 {
            res.push((b.0,1));
            res.push((bb,1));   
        }
        res.push((b.0,1));
    }
    else {
        res.push(b);
    }
     */
}
//NNNNN
//NCNCNCNCN

fn printc(vect:&Vec<(char,i64)>)
{
    for (c,cnt) in vect.iter() {
        if  *cnt==1 { print!("{}",*c);}
        else {print!("{}{}",*c,*cnt);}
    }
    
    println!();
}

//NNCB
//N(2)C(1)B(1)
//N(1)C(1)N(1)B(1)C(1)H(1)B(1)
//N(1)B(1)C(2)N(1)B(3)C(1)B(1)H(1)C(1)B(1)
//N(1)B(3)C(1)N(1)C(2)N(1)B(2)N(1)B(1)N(1)B(2)C(1)H(1)B(1)H(2)B(1)C(1)H(1)B(1)
fn expand(vect:&Vec<(char,i64)>,codes:&HashMap<String,char>)->Vec<(char,i64)>
{
    let mut res : Vec<(char,i64)> = vec![];
    if vect.len()<50 { printc(&vect); }
   
    for i in 0..vect.len()-1 
    {
//        res.push(vect[i]);
//        res.append(&mut get_v(codes, vect[i],vect[i+1]) );
        get_v(&mut res,codes, vect[i],vect[i+1]);
        //println!("{}b:{:?}",i,res);        
        //println!("{}a:{:?}",i,res);
    }

    res.push(vect[vect.len()-1]);

    
    res = compact(&res.clone());
    if res.len()<50  { printc(&res); }
    res
}

pub fn count(data:&[String],cnt:usize)->i64
{
    let equation = data[0].to_string();
    let mut code  = HashMap::new();
    
    for line in data.iter()
    {
        if line.contains(" -> ")
        {
            let tt : Vec<&str> = line.split(" -> ").collect();
            code.insert(tt[0].to_string(),tt[1].chars().nth(0).unwrap());
        }        
    }

    let mut res = equation;

    println!("{:?}",code);
    
    let mut vect = Vec::new();

    for i in 0..res.len() {
        vect.push((res.chars().nth(i).unwrap(),1));
    }

    vect = compact(&vect.clone());

    for ccc in 0..cnt
    {
        println!("c={:?} {}",ccc,vect.len());
        
        
        vect = expand(&vect,&code);
        //println!("bc-{:?}",vect);
        //vect = compact(&vect);
        //println!("ac-{:?}",vect);
    }

    let mut freq = HashMap::new();
    
    for (c,cnt) in vect.iter() {       
        freq.insert(c,freq.get(c).unwrap_or(&0)+cnt);
    }
    
    let maxv = freq.values().max().unwrap();
    let minv = freq.values().min().unwrap();

    println!("{:?}",freq);
    
    maxv-minv
}

pub fn part1(data:&[String])->i64
{
    count(data,10)
}

pub fn part2(data:&[String])->i64
{
    count(data,40)
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day2");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "NNCB".to_string(),
        "".to_string(),
        "CH -> B".to_string(),
        "HH -> N".to_string(),
        "CB -> H".to_string(),
        "NH -> C".to_string(),
        "HB -> C".to_string(),
        "HC -> B".to_string(),
        "HN -> C".to_string(),
        "NN -> C".to_string(),
        "BH -> H".to_string(),
        "NC -> B".to_string(),
        "NB -> B".to_string(),
        "BN -> B".to_string(),
        "BB -> N".to_string(),
        "BC -> B".to_string(),
        "CC -> N".to_string(),
        "CN -> C".to_string(),
    ];
    assert_eq!(part1(&v),1588);
}

#[test]
fn test2()
{
    let v = vec![
        "NNCB".to_string(),
        "".to_string(),
        "CH -> B".to_string(),
        "HH -> N".to_string(),
        "CB -> H".to_string(),
        "NH -> C".to_string(),
        "HB -> C".to_string(),
        "HC -> B".to_string(),
        "HN -> C".to_string(),
        "NN -> C".to_string(),
        "BH -> H".to_string(),
        "NC -> B".to_string(),
        "NB -> B".to_string(),
        "BN -> B".to_string(),
        "BB -> N".to_string(),
        "BC -> B".to_string(),
        "CC -> N".to_string(),
        "CN -> C".to_string(),
   ];
    assert_eq!(part2(&v),2188189693529);
}