fn add(a:&String,b:&String)->String
{
    format!("[{},{}]",a,b)
}

fn is_split(s:String)->bool
{
    s.replace("[", ",")
     .replace("]", ",")
     .split(',')
     .filter(|&s| !s.is_empty())
     .map(|s| s.parse::<i64>().unwrap())
     .any(|n| n>=10)
}

fn is_explode(s:String)->Option<(usize,usize)>
{
    let mut brackets=0;

    for (i,d1) in s.chars().enumerate()
    {        
        match d1
        {
            '[' => { 
                    if brackets==4
                    {
                        let mut c=0;
                        
                        for (j,cc) in s[i+1..].chars().enumerate()
                        {
                            if cc==',' {    c+=1; if c>1 { break; }        }
                            if cc=='[' {                   break;          }
                            if cc==']' { if c==1 { return Some((i,j+i+1)); }
                                            else {                  break; } }
                        }
                    }
                    brackets+=1;         
                   },
            ']' => brackets-=1,
             _  => {}
        }
    }
    None
}

fn search_digit(s:&str,id:i64,delta:i64)->Option<usize>
{
    let mut id = id;
    while id>=0 && id<s.len() as i64
    {
        if s.chars().nth(id as usize).unwrap().is_digit(10)
        {
            if id>0 && s.chars().nth((id-1) as usize).unwrap().is_digit(10) { return Some((id-1) as usize); }
            return Some(id as usize);
        }
        id+=delta;        
    }
    None
}

fn add_number(ll:&str,lf:usize,ln:i64)->String {

    let a = lf as usize;
    let mut b = lf as usize;
    if lf+1<ll.len() && ll.chars().nth(a+1).unwrap().is_digit(10) 
    {
        b=a+1;
    }

    let num = ll[a..b+1].to_string().parse::<i64>().unwrap() + ln;
    format!("{}{}{}",&ll[..a],num,&ll[b+1..])
}

fn explode(s:String)->String {
    let (a,b) = is_explode(s.to_string()).unwrap();
    let ll = &s[   ..a];
    let mm = &s[a+1..b];
    let rr = &s[b+1.. ];

    //println!("ll==={}",ll);
    //println!("mm==={}",mm);
    //println!("rr==={}",rr);

    let tab = mm.split(',').collect::<Vec<&str>>();
    let ln = tab[0].parse::<i64>().unwrap();
    let rn = tab[1].parse::<i64>().unwrap();
    let lf = search_digit(ll,ll.len() as i64-1,-1);
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

fn magnitude(s:&str)->i64
{
    if s.chars().next().unwrap()=='['
    {
        let coma_pos = find_coma(&s);
        3*magnitude(&s[1..coma_pos]) + 2*magnitude(&s[coma_pos+1..s.len()-1])
    }
      else
    {        
        s.parse::<i64>().unwrap()
    }
}

fn big_sum(a:&String,b:&String)->String
{
    let mut ab = add(a,b).to_string();
    // println!("after addition: {}",ab);
        
    while is_split(ab.to_string()) || is_explode(ab.to_string())!=None 
    {
        if is_explode(ab.to_string())!=None 
        { 
            ab = explode(ab.to_string());
        }
        else if is_split(ab.to_string()) 
        { 
            ab = split(ab.to_string());
        }

    }
    ab
}

fn compute_sum(data:&[String])->String
{
    let mut acc = data[0].to_string();

    for c in data.iter().skip(1) {
        acc = big_sum(&acc, c);
    }
    acc
}

fn compute(data:&[String])->i64
{
    magnitude(&compute_sum(data).to_owned())
}

pub fn part1(data:&[String])->i64
{
    compute(data)
}

pub fn part2(data:&[String])->i64
{
    let mut res =0;// i64::MIN;
    for i in 0..data.len()
    {
        for j in 0..data.len() 
        {
            if i!=j
            {
                res = res.max(magnitude(&big_sum(&data[i],&data[j])));
                res = res.max(magnitude(&big_sum(&data[j],&data[i])));
            }   
        }
    }

    res
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day18");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
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
    assert_eq!(add(&"[1,2]".to_string(), &"[[3,4],5]".to_string()),"[[1,2],[[3,4],5]]".to_string());
}

#[test]
fn tests_4()
{
    assert_eq!(magnitude("[[1,2],[[3,4],5]]"), 143);
}

#[test]
fn tests_5()
{
    assert_eq!(magnitude("[[[[0,7],4],[[7,8],[6,0]]],[8,1]]"),  1384);
}

#[test]
fn tests_6()
{
    assert_eq!(magnitude("[[[[1,1],[2,2]],[3,3]],[4,4]]"),  445);
}

#[test]
fn tests_7()
{
    assert_eq!(magnitude("[[[[3,0],[5,3]],[4,4]],[5,5]]"),  791);
}

#[test]
fn tests_8()
{
    assert_eq!(magnitude("[[[[5,0],[7,4]],[5,5]],[6,6]]"),  1137);
}

#[test]
fn tests_9()
{
    assert_eq!(magnitude("[[[[8,7],[7,7]],[[8,6],[7,7]]],[[[0,7],[6,6]],[8,7]]]"),  3488);
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

#[test]
fn teste_01()
{
    assert_eq!(explode("[[[[[9,8],1],2],3],4]".to_string()), "[[[[0,9],2],3],4]".to_string());
}

#[test]
fn teste_02()
{
    assert_eq!(explode("[7,[6,[5,[4,[3,2]]]]]".to_string()), "[7,[6,[5,[7,0]]]]".to_string());
}

#[test]
fn teste_03()
{
    assert_eq!(explode("[[6,[5,[4,[3,2]]]],1]".to_string()), "[[6,[5,[7,0]]],3]".to_string());
}

#[test]
fn teste_04()
{
    assert_eq!(explode("[[3,[2,[1,[7,3]]]],[6,[5,[4,[3,2]]]]]".to_string()), "[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".to_string());
}

#[test]
fn teste_05()
{
    assert_eq!(explode("[[3,[2,[8,0]]],[9,[5,[4,[3,2]]]]]".to_string()), "[[3,[2,[8,0]]],[9,[5,[7,0]]]]".to_string());
}

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
    assert_eq!(big_sum(&"[[[[4,3],4],4],[7,[[8,4],9]]]".to_string(),&"[1,1]".to_string()),"[[[[0,7],4],[[7,8],[6,0]]],[8,1]]".to_string());
}

#[test]
fn testsi_1()
{
    let v = vec![
        "[1,1]".to_string(),
        "[2,2]".to_string(),
        "[3,3]".to_string(),
        "[4,4]".to_string(),
    ];
    assert_eq!(compute_sum(&v),"[[[[1,1],[2,2]],[3,3]],[4,4]]");
}

#[test]
fn testsi_2()
{
    let v = vec![
        "[1,1]".to_string(),
        "[2,2]".to_string(),
        "[3,3]".to_string(),
        "[4,4]".to_string(),
        "[5,5]".to_string(),
    ];
    assert_eq!(compute_sum(&v),"[[[[3,0],[5,3]],[4,4]],[5,5]]");
}

#[test]
fn testsi_3()
{
    let v = vec![
        "[1,1]".to_string(),
        "[2,2]".to_string(),
        "[3,3]".to_string(),
        "[4,4]".to_string(),
        "[5,5]".to_string(),
        "[6,6]".to_string(),
        ];
        assert_eq!(compute_sum(&v),"[[[[5,0],[7,4]],[5,5]],[6,6]]");
}

#[test]
fn test_part2()
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
 
    assert_eq!(part2(&v),3993);
}
