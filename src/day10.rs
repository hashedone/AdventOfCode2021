fn compute(line:&str)->(i32,Vec<char>)
{
    let mut stack = vec![];

    for c in line.chars()
    {
        if "[<{(".contains(c)             
        {
            stack.push(c);
        }
          else
        {
            let (score,opposite) = match c {
                ')' => (    3,'('),
                ']' => (   57,'['),
                '}' => ( 1197,'{'),
                '>' => (25137,'<'),
                 _  => (    0,' '),
            };

            if stack.last().unwrap_or(&' ')!=&opposite
            {
                return (score,stack);
            }

           stack.pop();
        }
    }        

    stack.reverse();
    (0,stack)
}

pub fn part1(data:&[String])->i32
{
    data.iter()
        .map(|l| compute(l).0)
        .sum()
}

fn score(c:&[char])->i64
{
    c.iter()
     .fold(0, |acc,&x|
        5*acc + match x {
            ')' | '(' => 1,
            ']' | '[' => 2,
            '}' | '{' => 3,
            '>' | '<' => 4,
             _        => 0,
        }
     )
}

pub fn part2(data:&[String])->i64
{
    let mut tab = data.iter()
                      .filter_map(|l|
                      {
                        let res = compute(l);
                        if res.0==0 { Some(score(&res.1)) }
                               else { None                }
                      })
                      .collect::<Vec<i64>>();
                    
    tab.sort_unstable();
    tab[tab.len()/2]
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day10");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "[({(<(())[]>[[{[]{<()<>>".to_string(),
        "[(()[<>])]({[<{<<[]>>(".to_string(),
        "{([(<{}[<>[]}>{[]{[(<()>".to_string(),
        "(((({<>}<{<{<>}{[]{[]{}".to_string(),
        "[[<[([]))<([[{}[[()]]]".to_string(),
        "[{[{({}]{}}([{[{{{}}([]".to_string(),
        "{<[[]]>}<{[{[{[]{()[[[]".to_string(),
        "[<(<(<(<{}))><([]([]()".to_string(),
        "<{([([[(<>()){}]>(<<{{".to_string(),
        "<{([{{}}[<[[[<>{}]]]>[]]".to_string(),
    ];
    assert_eq!(part1(&v),26397);
}

#[test]
fn test2()
{
    let v = vec![
        "[({(<(())[]>[[{[]{<()<>>".to_string(),
        "[(()[<>])]({[<{<<[]>>(".to_string(),
        "{([(<{}[<>[]}>{[]{[(<()>".to_string(),
        "(((({<>}<{<{<>}{[]{[]{}".to_string(),
        "[[<[([]))<([[{}[[()]]]".to_string(),
        "[{[{({}]{}}([{[{{{}}([]".to_string(),
        "{<[[]]>}<{[{[{[]{()[[[]".to_string(),
        "[<(<(<(<{}))><([]([]()".to_string(),
        "<{([([[(<>()){}]>(<<{{".to_string(),
        "<{([{{}}[<[[[<>{}]]]>[]]".to_string(),
    ];
    assert_eq!(part2(&v),288957);
}

#[allow(unused)]
fn to_vec(s:&str)->Vec<char>
{
    s.chars().collect::<Vec<char>>()
}

#[test]
fn test51()
{
    assert_eq!(score(&to_vec("}}]])})]")), 288957);
}

#[test]
fn test52()
{
    assert_eq!(score(&to_vec(")}>]})")), 5566);
}

#[test]
fn test53()
{
    assert_eq!(score(&to_vec("}}>}>))))")), 1480781);
}

#[test]
fn test54()
{
    assert_eq!(score(&to_vec("]]}}]}]}>")), 995444);
}

#[test]
fn test55()
{
    assert_eq!(score(&to_vec("])}>")), 294);
}