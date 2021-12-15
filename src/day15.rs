use std::collections::{HashMap,BinaryHeap};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
struct Node
{
    distance : i32,
           x : i32,
           y : i32,
}

impl Node
{
    fn new(distance:i32,x:i32,y:i32)->Self
    {
        Self { distance,x,y }
    }
}

fn parse(data:&[String],field:&mut HashMap<(i32,i32),i32>)
{
    for line in data.iter().enumerate()
    {
        for x in line.1.chars().into_iter().enumerate()
        {
            let p = (x.0 as i32,line.0 as i32);
            field.insert(p, x.1.to_digit(10).unwrap() as i32);
        }
    }
}

fn ok(x:i32,y:i32,dx:usize,dy:usize)->bool
{
    !(x<0 || y<0 || x>dx as i32-1 || y>dy as i32-1)
}

fn compute(field:&mut HashMap<(i32,i32),i32>,dx:usize,dy:usize)->i32
{    
    let mut dist = vec![vec![i32::MAX;dx];dy];
    dist[0][0] = 0;

    let mut queue : BinaryHeap<Node> = BinaryHeap::new(); 
    queue.push( Node::new(i32::MAX,0,0) );

    while !queue.is_empty() 
    {
        let node = queue.pop().unwrap();
        let (distance,px,py) = (i32::MAX-node.distance,node.x,node.y);
        let neigh = vec![(px+1,py  ),
                         (px  ,py+1),
                         (px-1,py  ),
                         (px  ,py-1)];

        for (sx,sy) in neigh
        {
            if ok(sx,sy,dx,dy) && dist[sy as usize][sx as usize] > distance + field[&(sx,sy)]
            {
                let distance = distance + field[&(sx,sy)];
                dist[sy as usize][sx as usize] = distance;
                queue.push(Node::new(i32::MAX-distance,sx,sy) );
            }
        }
    }

    dist[dy-1][dx-1]
}

pub fn part1(data:&[String])->i32
{
    let mut field : HashMap<(i32,i32),i32> = HashMap::new();
    parse(data,&mut field);
    compute(&mut field,data[0].len(),data.len())
}

pub fn part2(data:&[String])->i32
{
    let dy = data.len();
    let dx = data[0].len();
    let mut field : HashMap<(i32,i32),i32> = HashMap::new();

    parse(data,&mut field);

    for y in 0..dy {
        for x in dx..dx*5 {
            let mut v = field.get(&((x-dx) as i32,y as i32)).unwrap() + 1;
            if v>9 {v=1;}
            field.insert((x as i32,y as i32), v as i32);
        }
    }

    for y in dy..dy*5 {
        for x in 0..dx*5 {            
            let mut v = field.get(&(x as i32,(y-dy) as i32)).unwrap() + 1;
            if v>9 {v=1;}
            field.insert((x as i32,y as i32), v as i32);
        }
    }
    
    compute(&mut field,dx*5,dy*5)    
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day15");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn test1_1()
{
    let v = vec![
        "1163751742".to_string(),
        "1381373672".to_string(),
        "2136511328".to_string(),
        "3694931569".to_string(),
        "7463417111".to_string(),
        "1319128137".to_string(),
        "1359912421".to_string(),
        "3125421639".to_string(),
        "1293138521".to_string(),
        "2311944581".to_string()
    ];
    assert_eq!(part1(&v),40);
}

#[test]
fn test1_2()
{
    let v = vec![
        "116".to_string(),
        "138".to_string(),
    ];
    assert_eq!(part1(&v),12);
}

#[test]
fn test1_3()
{
    let v = vec![
"11637517422274862853338597396444961841755517295286".to_string(),
"13813736722492484783351359589446246169155735727126".to_string(),
"21365113283247622439435873354154698446526571955763".to_string(),
"36949315694715142671582625378269373648937148475914".to_string(),
"74634171118574528222968563933317967414442817852555".to_string(),
"13191281372421239248353234135946434524615754563572".to_string(),
"13599124212461123532357223464346833457545794456865".to_string(),
"31254216394236532741534764385264587549637569865174".to_string(),
"12931385212314249632342535174345364628545647573965".to_string(),
"23119445813422155692453326671356443778246755488935".to_string(),
"22748628533385973964449618417555172952866628316397".to_string(),
"24924847833513595894462461691557357271266846838237".to_string(),
"32476224394358733541546984465265719557637682166874".to_string(),
"47151426715826253782693736489371484759148259586125".to_string(),
"85745282229685639333179674144428178525553928963666".to_string(),
"24212392483532341359464345246157545635726865674683".to_string(),
"24611235323572234643468334575457944568656815567976".to_string(),
"42365327415347643852645875496375698651748671976285".to_string(),
"23142496323425351743453646285456475739656758684176".to_string(),
"34221556924533266713564437782467554889357866599146".to_string(),
"33859739644496184175551729528666283163977739427418".to_string(),
"35135958944624616915573572712668468382377957949348".to_string(),
"43587335415469844652657195576376821668748793277985".to_string(),
"58262537826937364893714847591482595861259361697236".to_string(),
"96856393331796741444281785255539289636664139174777".to_string(),
"35323413594643452461575456357268656746837976785794".to_string(),
"35722346434683345754579445686568155679767926678187".to_string(),
"53476438526458754963756986517486719762859782187396".to_string(),
"34253517434536462854564757396567586841767869795287".to_string(),
"45332667135644377824675548893578665991468977611257".to_string(),
"44961841755517295286662831639777394274188841538529".to_string(),
"46246169155735727126684683823779579493488168151459".to_string(),
"54698446526571955763768216687487932779859814388196".to_string(),
"69373648937148475914825958612593616972361472718347".to_string(),
"17967414442817852555392896366641391747775241285888".to_string(),
"46434524615754563572686567468379767857948187896815".to_string(),
"46833457545794456865681556797679266781878137789298".to_string(),
"64587549637569865174867197628597821873961893298417".to_string(),
"45364628545647573965675868417678697952878971816398".to_string(),
"56443778246755488935786659914689776112579188722368".to_string(),
"55172952866628316397773942741888415385299952649631".to_string(),
"57357271266846838237795794934881681514599279262561".to_string(),
"65719557637682166874879327798598143881961925499217".to_string(),
"71484759148259586125936169723614727183472583829458".to_string(),
"28178525553928963666413917477752412858886352396999".to_string(),
"57545635726865674683797678579481878968159298917926".to_string(),
"57944568656815567976792667818781377892989248891319".to_string(),
"75698651748671976285978218739618932984172914319528".to_string(),
"56475739656758684176786979528789718163989182927419".to_string(),
"67554889357866599146897761125791887223681299833479".to_string(),
];
assert_eq!(part1(&v),315);
}

#[test]
fn test2_1()
{
    let v = vec![
        "1163751742".to_string(),
        "1381373672".to_string(),
        "2136511328".to_string(),
        "3694931569".to_string(),
        "7463417111".to_string(),
        "1319128137".to_string(),
        "1359912421".to_string(),
        "3125421639".to_string(),
        "1293138521".to_string(),
        "2311944581".to_string()
    ];
    assert_eq!(part2(&v),315);
}

/*
impl Ord for Node
{
    fn cmp(&self, other: &Self) -> Ordering {
        other.distance.cmp(&self.distance)
    }
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(other.cmp(self))
    }
}
*/

//let mut qq : BinaryHeap<Node> = BinaryHeap::new();
//
//qq.push(Node::new(70, 2, 1));
//qq.push(Node::new(170, 2, 2));
//qq.push(Node::new(10, 2, 3));
//qq.push(Node::new(60, 2, 4));
//
//println!("{:?}",qq.pop());
//println!("{:?}",qq.pop());
//println!("{:?}",qq.pop());
//println!("{:?}",qq.pop());
