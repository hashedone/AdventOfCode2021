//use std::{collections::HashSet, fmt::format};
//use crate::tools;

fn add(a:String,b:String)->String
{
    format!("[{},{}]",a,b)
}

fn is_split(s:String)->bool
{
    let ss = s.replace("[", ",");
    let ss = ss.replace("]", ",");
    let t = ss.split(',').collect::<Vec<&str>>();

    t.into_iter()
     .filter(|&s| !s.is_empty())
     .map(|s| s.parse::<i32>().unwrap())
     .filter(|&n| n>=10)
     .count()>0
}

fn is_explode(s:String)->Option<(usize,usize)>
{
    let mut b=0;
    for i in 0..s.len()
    {
        let d1 = s.chars().nth(i  ).unwrap();
        
        if d1=='[' && b==4
        {
            let mut c=0;
            for j in i+1..s.len()
            {
                let cc = s.chars().nth(j).unwrap();
                if cc==',' { c+=1; }
                if cc=='[' { break; }
                if cc==']' { if c==1 {return Some((i,j));} else { break;}  }            
            }
        }
        if d1=='[' { b+=1; }
        if d1==']' { b-=1; }
    }
    None
}
fn search_digit(s:&str,id:i32,delta:i32)->Option<usize>
{
    let mut id = id;
    while id>=0 && id<s.len() as i32
    {
        if s.chars().nth(id as usize).unwrap().is_digit(10)
        {
            if id-1>=0 && s.chars().nth((id-1) as usize).unwrap().is_digit(10) {return Some((id-1) as usize);}
            return Some(id as usize);
        }
        id+=delta;        
    }
    None
}

fn add_number(ll:&str,lf:usize,ln:i32)->String {

    let a = lf as usize;
    let mut b = lf as usize;
    if lf+1<ll.len() && ll.chars().nth(a+1).unwrap().is_digit(10) {
        b=a+1;
    }

    //println!("number:(*{}*)",ll[a..b+1].to_string());
    let num = ll[a..b+1].to_string().parse::<i32>().unwrap() + ln;
    format!("{}{}{}",&ll[..a],num,&ll[b+1..])
}

fn explode(s:String)->String {
    let (a,b) = is_explode(s.to_string()).unwrap();
    let ll = &s[..a];
    let mm = &s[a+1..b];
    let rr = &s[b+1..];

    //println!("ll==={}",ll);
    //println!("mm==={}",mm);
    //println!("rr==={}",rr);

    let tab = mm.split(',').collect::<Vec<&str>>();
    let ln = tab[0].parse::<i32>().unwrap();
    let rn = tab[1].parse::<i32>().unwrap();
    let lf = search_digit(ll,ll.len() as i32-1,-1);
    let rf = search_digit(rr,0         , 1);
    //println!("{:?}",lf);
    //println!("{:?}",rf);
    let ll = if lf!=None { add_number(   ll,lf.unwrap(),ln) } else { ll.to_string() };
    let rr = if rf!=None { add_number(rr,rf.unwrap(),rn) } else { rr.to_string() };
    format!("{}{}{}",ll,"0",rr)
}

fn split(s:String)->String
{
    for i in 0..s.len()-1
    {
        let d1 = s.chars().nth(i  ).unwrap();
        let d2 = s.chars().nth(i+1).unwrap();
        if d1.is_digit(10) &&
           d2.is_digit(10)
        {
            let n = d1.to_digit(10).unwrap()*10 + d2.to_digit(10).unwrap();
            let l = n/2;
            let r = (n+1)/2;
            return format!("{}[{},{}]{}",&s[..i],l,r,&s[i+2..]);
        }
    }
    "".to_string()
}
fn find_coma(s:&str)->usize
{
    let mut b= 0;
    for (id,c) in s.chars().enumerate()
    {
        match c
        {
            '[' => b+=1,
            ']' => b-=1,
            ',' => if b==1 { return id; },
             _  => {},
        }
    }
    0
}

fn magnitude(s:String)->i32
{
    //[[[[5,0],[7,4]],[5,5]],[6,6]]
    if s.chars().next().unwrap()=='['
    {
        let c = find_coma(&s);
        return 3*magnitude(s[1..c].to_owned()) + 2*magnitude(s[c+1..s.len()-1].to_owned());
    }
      else
    {
        
        return s.parse::<i32>().unwrap()
    }
}

fn big_sum(a:String,b:String)->String
{
    let mut ab = add(a,b).to_string();
    // println!("after addition: {}",ab);
        
    while is_split(ab.to_string()) || is_explode(ab.to_string())!=None {
        while is_explode(ab.to_string())!=None 
        { 
            //println!("explode");
            ab = explode(ab.to_string());
            //println!("after explode:  {}",ab);
            while is_split(ab.to_string()) 
            { 
                //println!("split");
                ab = split(ab.to_string());
                //println!("after split:    {}",ab);
            }
        }
        while is_split(ab.to_string()) 
        { 
            //println!("split");
            ab = split(ab.to_string());
            //println!("after split:    {}",ab);
        }
    }
    ab
}

fn compute_sum(data:&[String])->String
{
    let mut acc=data[0].to_string();
    println!("acc:{}",acc);
    for i in 1..data.len() {
        acc = big_sum(acc, data[i].to_string());
        println!("acc:{}",acc);
    }
    acc
}


fn compute(data:&[String])->i32
{
    magnitude(compute_sum(data))
}

pub fn part1(data:&[String])->i32
{

    0
    /*
    let yy = tools::get_between(data,"y=","");
    let x0 = tools::i32_get_between(&xx[..],  "","..");    


    let mut pos = 0;
    let mut depth = 0;
    
    for line in data {
        let cmd : Vec<&str> = line.split(' ').collect();
        let x   : i32       = cmd[1].parse().unwrap();

        match cmd[0] {
            "forward" => pos  +=x,
            "up"      => depth-=x,
            "down"    => depth+=x,
            _         => {},
        }
    }

    pos*depth
     */
}

pub fn part2(data:&[String])->i32
{
    0
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day18");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn test1()
{
//    let v = vec![
    //];
    //assert_eq!(part1("[[[[1,1],[2,2]],[3,3]],[4,4]]".to_string()),"[[[[3,0],[5,3]],[4,4]],[5,5]]");
}


#[test]
fn test2()
{
    //let v = vec![
      //  "[[[[3,0],[5,3]],[4,4]],[5,5]]".to_string()
    //];
    //assert_eq!(part1(v),"[[[[3,0],[5,3]],[4,4]],[5,5]]");
}


#[test]
fn tests_1()
{
    assert_eq!(split("[[[[0,7],4],[15,[0,13]]],[1,1]]".to_string()),"[[[[0,7],4],[[7,8],[0,13]]],[1,1]]".to_string());
}

#[test]
fn tests_2()
{
    assert_eq!(split("[[[[0,7],4],[[7,8],[0,13]]],[1,1]]".to_string()),"[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]".to_string());
}


#[test]
fn tests_3()
{
    assert_eq!(add("[1,2]".to_string(), "[[3,4],5]".to_string()),"[[1,2],[[3,4],5]]".to_string());
}

 

#[test]
fn tests_4()
{
    assert_eq!(magnitude("[[1,2],[[3,4],5]]".to_string()), 143);
}

#[test]
fn tests_5()
{
    assert_eq!(magnitude("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".to_string()),  1384);
}

#[test]
fn tests_6()
{
    assert_eq!(magnitude("[[[[1,1],[2,2]],[3,3]],[4,4]]".to_string()),  445);
}

#[test]
fn tests_7()
{
    assert_eq!(magnitude("[[[[3,0],[5,3]],[4,4]],[5,5]]".to_string()),  791);
}

#[test]
fn tests_8()
{
    assert_eq!(magnitude("[[[[5,0],[7,4]],[5,5]],[6,6]]".to_string()),  1137);
}

#[test]
fn tests_9()
{
    assert_eq!(magnitude("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]".to_string()),  3488);
}


#[test]
fn tests_10()
{
    assert_eq!(explode("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]".to_string()),"[[[[0,7],4],[7,[[8,4],9]]],[1,1]]".to_string());
}

#[test]
fn tests_11()
{
    assert_eq!(explode("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]".to_string()),"[[[[0,7],4],[15,[0,13]]],[1,1]]".to_string());
}

#[test]
fn tests_12()
{
    assert_eq!(explode("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]".to_string()),"[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".to_string());
}

//after addition: [[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]
//after explode:  [[[[0,7],4],[7,[[8,4],9]]],[1,1]]
//after explode:  [[[[0,7],4],[15,[0,13]]],[1,1]]
//after split:    [[[[0,7],4],[[7,8],[0,13]]],[1,1]]
//after split:    [[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]
//after explode:  [[[[0,7],4],[[7,8],[6,0]]],[8,1]]



#[test]
fn testt_13()
{
    assert_eq!(is_explode("[[[[[4,3],4],4],[7,[[8,4],9]]],[1,1]]".to_string()),Some((4,8)));
}

#[test]
fn testt_14()
{
    assert_eq!(is_explode("[[[[0,7],4],[7,[[8,4],9]]],[1,1]]".to_string()),Some((16,20)));
}

#[test]
fn testt_15()
{
    assert_eq!(is_explode("[[[[0,7],4],[[7,8],[0,[6,7]]]],[1,1]]".to_string()),Some((22,26)));
}


#[test]
fn test2_1()
{
    let v = vec![
        "[[[0,[5,8]],[[1,7],[9,6]]],[[4,[1,2]],[[1,4],2]]]".to_string(),
        "[[[5,[2,8]],4],[5,[[9,9],0]]]".to_string(),
        "[6,[[[6,2],[5,6]],[[7,6],[4,7]]]]".to_string(),
        "[[[6,[0,7]],[0,9]],[4,[9,[9,0]]]]".to_string(),
        "[[[7,[6,4]],[3,[1,3]]],[[[5,5],1],9]]".to_string(),
        "[[6,[[7,3],[3,2]]],[[[3,8],[5,7]],4]]".to_string(),
        "[[[[5,4],[7,7]],8],[[8,3],8]]".to_string(),
        "[[9,3],[[9,9],[6,[4,9]]]]".to_string(),
        "[[2,[[7,7],7]],[[5,8],[[9,3],[0,2]]]]".to_string(),
        "[[[[5,2],5],[8,[3,7]]],[[5,[7,5]],[4,4]]]".to_string(),
        ];
    assert_eq!(compute_sum(&v),"[[[[6,6],[7,6]],[[7,7],[7,0]]],[[[7,7],[7,7]],[[7,8],[9,9]]]]".to_string());
}

#[test]
fn test2_2()
{
    let v = vec![
        "[[[0,[4,5]],[0,0]],[[[4,5],[2,6]],[9,5]]]".to_string(),
        "[7,[[[3,7],[4,3]],[[6,3],[8,8]]]]".to_string(),
        "[[2,[[0,8],[3,4]]],[[[6,7],1],[7,[1,6]]]]".to_string(),
        "[[[[2,4],7],[6,[0,5]]],[[[6,8],[2,8]],[[2,1],[4,5]]]]".to_string(),
        "[7,[5,[[3,8],[1,4]]]]".to_string(),
        "[[2,[2,2]],[8,[8,1]]]".to_string(),
        "[2,9]".to_string(),
        "[1,[[[9,3],9],[[9,0],[0,7]]]]".to_string(),
        "[[[5,[7,4]],7],1]".to_string(),
        "[[[[4,2],2],6],[8,7]]".to_string(),
    ];
    assert_eq!(compute_sum(&v),"[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]");
}

#[test]
fn test2_3()
{
    assert_eq!(big_sum("[[[[4,3],4],4],[7,[[8,4],9]]]".to_string(),"[1,1]".to_string()),"[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".to_string());
}