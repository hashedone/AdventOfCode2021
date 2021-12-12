use std::collections::{HashMap};

pub fn part1(data:&[String])->i32
{
    data.iter()
        .map(|line|
            {
              line.split('|')
                  .collect::<Vec<&str>>()[1]
                  .split_whitespace()
                  .map(|s| s.len())
                  .filter(|&n| n==2 || n==4 || n==3 || n==7)
                  .count() as i32
            }
        ).sum()
}

fn convert(s:&str,pos:&[char])->String
{
    s.chars()
     .map(|c| pos[c as usize - 'a' as usize])
     .collect::<String>()
}

fn sort_str(s:&str)->String 
{
    let mut chars: Vec<char> = s.to_string().chars().collect();
    chars.sort_unstable();
    chars.into_iter().collect::<String>()    
}

fn calc(line : String)->i32
{
    let cmd  : Vec<&str> = line.split('|').collect();
    let cmd1 : Vec<_>    = cmd[0].split_whitespace().map(|s| sort_str(s)).collect();
    let cmd2 : Vec<_>    = cmd[1].split_whitespace().map(|s| sort_str(s)).collect();

    let digits : HashMap<&str,i32> = 
    [    
        ("abcefg" ,0),
        ("cf"     ,1),
        ("acdeg"  ,2),
        ("acdfg"  ,3),
        ("bcdf"   ,4),
        ("abdfg"  ,5),
        ("abdefg" ,6),
        ("acf"    ,7),
        ("abcdefg",8),
        ("abcdfg" ,9),
    ].iter().cloned().collect();

    for a in 'a'..='g' {
    for b in 'a'..='g' { if b!=a {
    for c in 'a'..='g' { if c!=a && c!=b {
    for d in 'a'..='g' { if d!=a && d!=b && d!=c {        
    for e in 'a'..='g' { if e!=a && e!=b && e!=c && e!=d {            
    for f in 'a'..='g' { if f!=a && f!=b && f!=c && f!=d && f!=e {                    
    for g in 'a'..='g' { if g!=a && g!=b && g!=c && g!=d && g!=e && g!=f  
    {
            let mut ok=0;
            let perm = [a,b,c,d,e,f,g].to_vec();

            for command in cmd1.iter() 
            {
                let ss = convert(command,&perm);
                let so = sort_str(&ss.to_owned());
                if !digits.contains_key(&&so[..]) { break; }

                ok+=1;
            }

            if ok==10
            {
                let mut res = [0,0,0,0];

                for i in 0..4 
                {
                    let s      = convert(&cmd2[i],&perm);
                    let sorted = sort_str(&s[..]);
                    res[i]     = *digits.get(&&sorted[..]).unwrap();
                }

                return (res[0]*1000 + res[1]*100 + res[2]*10 + res[3]) as i32;
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

pub fn part2(data:&[String])->i32
{
    data.iter()
        .map(|d| calc(d.clone()))
        .sum()
}

#[allow(unused)]
pub fn solve(data:&[String])
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
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe".to_string(),
        "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc".to_string(),
        "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg".to_string(),
        "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb".to_string(),
        "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea".to_string(),
        "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb".to_string(),
        "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe".to_string(),
        "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef".to_string(),
        "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb".to_string(),
        "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce".to_string(),
    ];
    assert_eq!(part1(&v),26);
}


#[test]
fn test3()
{
    let v = vec![
        "be cfbegad cbdgef fgaecd cgeb fdcge agebfd fecdb fabcd edb | fdgacbe cefdb cefbgd gcbe".to_string(),
        "edbfga begcd cbg gc gcadebf fbgde acbgfd abcde gfcbed gfec | fcgedb cgb dgebacf gc".to_string(),
        "fgaebd cg bdaec gdafb agbcfd gdcbef bgcad gfac gcb cdgabef | cg cg fdcagb cbg".to_string(),
        "fbegcd cbd adcefb dageb afcb bc aefdc ecdab fgdeca fcdbega | efabcd cedba gadfec cb".to_string(),
        "aecbfdg fbg gf bafeg dbefa fcge gcbea fcaegb dgceab fcbdga | gecf egdcabf bgf bfgea".to_string(),
        "fgeab ca afcebg bdacfeg cfaedg gcfdb baec bfadeg bafgc acf | gebdcfa ecba ca fadegcb".to_string(),
        "dbcfg fgd bdegcaf fgec aegbdf ecdfab fbedc dacgb gdcebf gf | cefg dcbef fcge gbcadfe".to_string(),
        "bdfegc cbegaf gecbf dfcage bdacg ed bedf ced adcbefg gebcd | ed bcgafe cdgba cbgef".to_string(),
        "egadfb cdbfeg cegd fecab cgb gbdefca cg fgcdab egfdb bfceg | gbdfcae bgc cg cgb".to_string(),
        "gcafb gcf dcaebfg ecagb gf abcdeg gaef cafbge fdbac fegbdc | fgae cfgab fg bagce".to_string(),
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
