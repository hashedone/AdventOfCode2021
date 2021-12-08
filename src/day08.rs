pub fn part1(data:&Vec<String>)->i32
{
    let mut count = 0;
    
    for line in data {
        let cmd  : Vec<&str> = line.split('|').collect();
        let cmd2 : Vec<&str> = cmd[1].split_whitespace().collect();

        count+= cmd2.iter()
                    .map(|s| s.len())
                    .filter(|&n| n==2 || n==3 || n==7 || n==4)
                    .count();
    }

    count as i32
}

fn conv(cmd2:&str,pos:&Vec<char>)->String
{
    let mut s = "".to_string();
    for c in cmd2.chars() 
    {
        let d = pos[c as usize - 'a' as usize];
        s.push(d as char);
    }
    s
}
fn sort_str(s:&str)->String {
    let mut chars: Vec<char> = s.to_string().chars().collect();
    chars.sort();
    chars.into_iter().collect::<String>()    
}

fn calc(data0:String)->i32
{
    let line = data0.to_string();
    let cmd : Vec<&str> = line.split('|').collect();
    let cmd1 : Vec<_> = cmd[0].split_whitespace().map(|s| sort_str(s).to_owned()).collect();
    let cmd2 : Vec<_> = cmd[1].split_whitespace().map(|s| sort_str(s).to_owned()).collect();
 
    //println!("1:{:?}",&cmd1);
    //println!("2:{:?}",&cmd2);

    let digits = vec![
        "abcefg" ,//0
        "cf"     ,//1
        "acdeg"  ,//2
        "acdfg"  ,//3
        "bcdf"   ,//4
        "abdfg"  ,//5
        "abdefg" ,//6
        "acf"    ,//7
        "abcdefg",//8
        "abcdfg" ,//9
    ];

    for a in 'a'..='g'
    {
        for b in 'a'..='g' {
        if b!=a 
        {
            for c in 'a'..='g' {
            if c!=a && c!=b
            {
                for d in 'a'..='g' {
                    if d!=a && d!=b && d!=c
                {        
                    for e in 'a'..='g' {
                    if e!=a && e!=b && e!=c && e!=d
                    {            
                        for f in 'a'..='g' {
                        if f!=a && f!=b && f!=c && f!=d && f!=e
                        {                    
                            for g in 'a'..='g' {
                            if g!=a && g!=b && g!=c && g!=d && g!=e && g!=f
                            {                    
                                let mut res = [0,0,0,0];
                                let mut cnt = 0;

                                let mut ok=0;
                                for command in cmd1.iter() 
                                {
                                    let ss = conv(&command.to_owned(),&[a,b,c,d,e,f,g].to_vec());
                                    let so = sort_str(&ss.to_owned());
                                    if digits.contains(&&so[..]){
                                        ok+=1;
                                    }
                                    else {
                                        break;
                                    }
                                }

                                if ok==10
                                {
                                    for i in 0..4 
                                    {
                                        let ss = conv(&cmd2[i].to_owned(),&[a,b,c,d,e,f,g].to_vec());
                                        let sorted = sort_str(&ss[..]);

                                        if digits.contains(&&sorted[..])
                                        {
                                            let ii = digits.iter().position(|&s| s.to_string()==sorted).unwrap();
                                            res[i] = ii as i32;
                                            cnt+=1;    
                                        }
                                    }

                                    return res[0]*1000 + res[1]*100 + res[2]*10 + res[3]*1;
                                }
                            }}
                        }}
                    }}
                }}
            }}
                
        }}
    }

    -1
}

pub fn part2(data:&Vec<String>)->i32
{
    data.iter()
        .map(|d| calc(d.clone()))
        .sum()
}

#[allow(unused)]
pub fn solve(data:&Vec<String>)
{    
    println!("Day8");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn test01()
{
    let v = vec![
        "acedgfb cdfbe gcdfa fbcad dab cefabd cdfgeb eafb cagedb ab |cdfeb fcadb cdfeb cdbaf".to_string()
    ];
    assert_eq!(part2(&v),5353);
}

#[test]
fn test2()
{
    let v = vec![
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |        fdgacbe cefdb cefbgd gcbe".to_string(),
        "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |        fcgedb cgb dgebacf gc".to_string(),
        "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |        cg cg fdcagb cbg".to_string(),
        "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |        efabcd cedba gadfec cb".to_string(),
        "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |        gecf egdcabf bgf bfgea".to_string(),
        "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |        gebdcfa ecba ca fadegcb".to_string(),
        "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |        cefg dcbef fcge gbcadfe".to_string(),
        "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |        ed bcgafe cdgba cbgef".to_string(),
        "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |        gbdfcae bgc cg cgb".to_string(),
        "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |        fgae cfgab fg bagce".to_string(),
    ];
    assert_eq!(part1(&v),26);
}


#[test]
fn test3()
{
    let v = vec![
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb |        fdgacbe cefdb cefbgd gcbe".to_string(),
        "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec |        fcgedb cgb dgebacf gc".to_string(),
        "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef |        cg cg fdcagb cbg".to_string(),
        "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega |        efabcd cedba gadfec cb".to_string(),
        "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga |        gecf egdcabf bgf bfgea".to_string(),
        "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf |        gebdcfa ecba ca fadegcb".to_string(),
        "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf |        cefg dcbef fcge gbcadfe".to_string(),
        "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd |        ed bcgafe cdgba cbgef".to_string(),
        "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg |        gbdfcae bgc cg cgb".to_string(),
        "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc |        fgae cfgab fg bagce        ".to_string(),
    ];
    assert_eq!(part2(&v),61229);
}


#[test]
fn test4()
{
    let v = vec![
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe".to_string(),
    ];
    assert_eq!(part2(&v),8394);
}

#[test]
fn test5()
{
    let v = vec![
        "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc".to_string() ,
    ];
    assert_eq!(part2(&v ),9781);
}

#[test]
fn test6()
{
    let v = vec![
        "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg".to_string()  ,
    ];
    assert_eq!(part2(&v),1197);
}

#[test]
fn test7()
{
    let v = vec![
        "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb".to_string()
    ];
    assert_eq!(part2(&v ),9361);
}

#[test]
fn test8()
{
    let v = vec![
        "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea".to_string()
    ];
    assert_eq!(part2(&v ),4873);
}

#[test]
fn test9()
{
    let v = vec![
        "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb".to_string(),
    ];
    assert_eq!(part2(&v),8418);
}

#[test]
fn test10()
{
    let v = vec![
        "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe".to_string(),
    ];
    assert_eq!(part2(&v),4548);
}

#[test]
fn test11()
{
    let v = vec![
        "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef".to_string(),
    ];
    assert_eq!(part2(&v),1625);
}
#[test]
fn test12()
{
    let v = vec![
        "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb".to_string()  
    ];
    assert_eq!(part2(&v),8717);
}
#[test]
fn test13()
{
    let v = vec![
        "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce".to_string()  
    ];
    assert_eq!(part2(&v),4315);
}
