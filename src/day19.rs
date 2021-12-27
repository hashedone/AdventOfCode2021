use std::collections::{HashSet,HashMap};
use crate::tools;

#[derive(Debug,Clone,PartialEq, Eq, PartialOrd, Ord)]
struct Scanner
{
          beams : Vec<(i32,i32,i32)>,
             id : i32,
              x : i32,
              y : i32,
              z : i32,
          trans : i32,
           used : bool,
     all_points : Vec<Vec<(i32,i32,i32)>>
}

//matrices from http://www.euclideanspace.com/maths/algebra/matrix/transforms/examples/index.htm
fn get_view_matrix(id:usize)->Vec<(i32,i32,i32)>
{
    let views = vec![
        [( 1, 0, 0),
         ( 0, 1, 0),
         ( 0, 0, 1)],
        [( 1, 0, 0),
         ( 0, 0,-1),
         ( 0, 1, 0)],
        [( 1, 0, 0),
         ( 0,-1, 0),
         ( 0, 0,-1)],
        [( 1, 0, 0),
         ( 0, 0, 1),
         ( 0,-1, 0)],

        [( 0,-1, 0),
         ( 1, 0, 0),
         ( 0, 0, 1)],
        [( 0, 0, 1),
         ( 1, 0, 0),
         ( 0, 1, 0)],
        [( 0, 1, 0),
         ( 1, 0, 0),
         ( 0, 0,-1)],
        [( 0, 0,-1),
         ( 1, 0, 0),
         ( 0,-1, 0)],

        [(-1, 0, 0),
         ( 0,-1, 0),
         ( 0, 0, 1)],
        [(-1, 0, 0),
         ( 0, 0,-1),
         ( 0,-1, 0)],
        [(-1, 0, 0),
         ( 0, 1, 0),
         ( 0, 0,-1)],
        [(-1, 0, 0),
         ( 0, 0, 1),
         ( 0, 1, 0)],

        [( 0, 1, 0),
         (-1, 0, 0),
         ( 0, 0, 1)],
        [( 0, 0, 1),
         (-1, 0, 0),
         ( 0,-1, 0)],
        [( 0,-1, 0),
         (-1, 0, 0),
         ( 0, 0,-1)],
        [( 0, 0,-1),
         (-1, 0, 0),
         ( 0, 1, 0)],

        [( 0, 0,-1),
         ( 0, 1, 0),
         ( 1, 0, 0)],
        [( 0, 1, 0),
         ( 0, 0, 1),
         ( 1, 0, 0)],
        [( 0, 0, 1),
         ( 0,-1, 0),
         ( 1, 0, 0)],
        [( 0,-1, 0),
         ( 0, 0,-1),
         ( 1, 0, 0)],

        [( 0, 0,-1),
         ( 0,-1, 0),
         (-1, 0, 0)],
        [( 0,-1, 0),
         ( 0, 0, 1),
         (-1, 0, 0)],
        [( 0, 0, 1),
         ( 0, 1, 0),
         (-1, 0, 0)],
        [( 0, 1, 0),
         ( 0, 0,-1),
         (-1, 0, 0)],
        ];

    views[id].to_vec()
}

impl Scanner
{
    fn new(id:i32)->Self
    {
        Self {
                    id,
                 beams : Vec::new(),
                     x : 0, 
                     y : 0, 
                     z : 0,
                  used : false,
                 trans : -1,
            all_points : Vec::new(),
        }
    }

    fn dist(p1:(i32,i32,i32),p2:(i32,i32,i32))->usize
    {
        (p2.0-p1.0).abs() as usize +
        (p2.1-p1.1).abs() as usize +
        (p2.2-p1.2).abs() as usize
    }

    fn transform(p:(i32,i32,i32),mtx:&[(i32,i32,i32)])->(i32,i32,i32)
    {
        let x = p.0*mtx[0].0 + p.1*mtx[0].1 + p.2*mtx[0].2;
        let y = p.0*mtx[1].0 + p.1*mtx[1].1 + p.2*mtx[1].2;
        let z = p.0*mtx[2].0 + p.1*mtx[2].1 + p.2*mtx[2].2;
        (x,y,z)
    }

    fn transformp(&mut self)
    {
        self.all_points = vec![];

        for i in 0..24 
        {
            let mtx = &get_view_matrix(i);
            let new_points = self.beams.iter().map( |p|                    
                    Scanner::transform(*p, mtx)
            ).collect();

            self.all_points.push(new_points);
        }
    }
    
    fn match_points(&self,s1_points:&HashSet<(i32,i32,i32)>,s2_points:&[(i32,i32,i32)])->(bool,i32,i32,i32)
    {
        let mut h = HashMap::new();

        for p1 in s1_points.iter() {
            for p2 in s2_points {
                let dx = p1.0 - p2.0;
                let dy = p1.1 - p2.1;
                let dz = p1.2 - p2.2;

                let v = *h.get(&(dx,dy,dz)).unwrap_or(&0);
                h.insert((dx,dy,dz),   v+1);
            }
        }
        
        for (p,v) in h.into_iter()
        {
            if v>=12
            {
                return (true,p.0,p.1,p.2);
            }                
        }
        
        (false,0,0,0)
    }
    
    fn get_points(&self,id:usize)->&Vec<(i32,i32,i32)> {
        &self.all_points[id]
    }

    #[allow(unused)]
    fn print(&self)
    {
        println!("Scaner {}",self.id);
        for i in self.beams.iter() {
            println!("{},{},{}",i.0,i.1,i.2);
        }
        println!();
    }
}

fn add_points(points:&mut HashSet<(i32,i32,i32)>,beams:&[(i32,i32,i32)],dx:i32,dy:i32,dz:i32)
{
    for p in beams {
        let x = p.0 + dx;
        let y = p.1 + dy;
        let z = p.2 + dz;
        points.insert((x,y,z));
    }
}

fn compute(data:&[String])->(usize,usize)
{
    let mut scanners: Vec<Scanner> = Vec::new();

    for l in data 
    {
        if l.contains("--- scanner")
        {            
            let id = tools::i32_get_between(l,"--- scanner "," ---");
            scanners.push(Scanner::new(id));
        }
        else if l.contains(',')
        {
            let tab = l.split(',').collect::<Vec<&str>>();
            let x = tab[0].parse::<i32>().unwrap();
            let y = tab[1].parse::<i32>().unwrap();
            let z = tab[2].parse::<i32>().unwrap();
            let s = scanners.last_mut().unwrap();
            s.beams.push((x,y,z));
        }
    }
    
    let mut final_points = HashSet::new();

    for f in scanners.iter_mut()
    {
        f.transformp();
    }

    scanners[0].used = true;
    add_points(&mut final_points,&scanners[0].beams,0,0,0);

    let mut found = true;
    let mut a_id = 0usize;
    let mut scanners_out = vec![];


    while found 
    {
        found = false;

        let sc = scanners[a_id].clone();
        for (scanner_id,b) in scanners.iter_mut().enumerate() 
        {
            if a_id!=scanner_id && !b.used
            {
                for id in 0..24
                {
                    let new_points = b.get_points(id).clone();
                    let (res,dx,dy,dz) = sc.match_points(&final_points,&new_points);

                    if res
                    {
                        b.used = true;
                        add_points(&mut final_points,&new_points,dx,dy,dz);

                        scanners_out.push((dx,dy,dz));
                        a_id = scanner_id;
                        found = true;

                        break;
                    }   
                }                    
            }
        }   
        
    }

    let mut max_dist= 0usize;

    for a in 0..scanners_out.len() {
    for b in 0..scanners_out.len() {    
        max_dist = max_dist.max(Scanner::dist(scanners_out[a], scanners_out[b]));        
    }
    }

    (final_points.len(),max_dist)
}

pub fn part1(data:&[String])->usize
{
    compute(data).0
}

pub fn part2(data:&[String])->usize
{
    compute(data).1
}

#[allow(unused)]
pub fn solve(data:&[String])
{    
    println!("Day19");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn testr1()
{
    let v = vec![
        "--- scanner 0 ---".to_string(),
        "404,-588,-901".to_string(),
        "528,-643,409".to_string(),
        "-838,591,734".to_string(),
        "390,-675,-793".to_string(),
        "-537,-823,-458".to_string(),
        "-485,-357,347".to_string(),
        "-345,-311,381".to_string(),
        "-661,-816,-575".to_string(),
        "-876,649,763".to_string(),
        "-618,-824,-621".to_string(),
        "553,345,-567".to_string(),
        "474,580,667".to_string(),
        "-447,-329,318".to_string(),
        "-584,868,-557".to_string(),
        "544,-627,-890".to_string(),
        "564,392,-477".to_string(),
        "455,729,728".to_string(),
        "-892,524,684".to_string(),
        "-689,845,-530".to_string(),
        "423,-701,434".to_string(),
        "7,-33,-71".to_string(),
        "630,319,-379".to_string(),
        "443,580,662".to_string(),
        "-789,900,-551".to_string(),
        "459,-707,401".to_string(),
        "".to_string(),
        "--- scanner 1 ---".to_string(),
        "686,422,578".to_string(),
        "605,423,415".to_string(),
        "515,917,-361".to_string(),
        "-336,658,858".to_string(),
        "95,138,22".to_string(),
        "-476,619,847".to_string(),
        "-340,-569,-846".to_string(),
        "567,-361,727".to_string(),
        "-460,603,-452".to_string(),
        "669,-402,600".to_string(),
        "729,430,532".to_string(),
        "-500,-761,534".to_string(),
        "-322,571,750".to_string(),
        "-466,-666,-811".to_string(),
        "-429,-592,574".to_string(),
        "-355,545,-477".to_string(),
        "703,-491,-529".to_string(),
        "-328,-685,520".to_string(),
        "413,935,-424".to_string(),
        "-391,539,-444".to_string(),
        "586,-435,557".to_string(),
        "-364,-763,-893".to_string(),
        "807,-499,-711".to_string(),
        "755,-354,-619".to_string(),
        "553,889,-390".to_string(),
        "".to_string(),
        "--- scanner 2 ---".to_string(),
        "649,640,665".to_string(),
        "682,-795,504".to_string(),
        "-784,533,-524".to_string(),
        "-644,584,-595".to_string(),
        "-588,-843,648".to_string(),
        "-30,6,44".to_string(),
        "-674,560,763".to_string(),
        "500,723,-460".to_string(),
        "609,671,-379".to_string(),
        "-555,-800,653".to_string(),
        "-675,-892,-343".to_string(),
        "697,-426,-610".to_string(),
        "578,704,681".to_string(),
        "493,664,-388".to_string(),
        "-671,-858,530".to_string(),
        "-667,343,800".to_string(),
        "571,-461,-707".to_string(),
        "-138,-166,112".to_string(),
        "-889,563,-600".to_string(),
        "646,-828,498".to_string(),
        "640,759,510".to_string(),
        "-630,509,768".to_string(),
        "-681,-892,-333".to_string(),
        "673,-379,-804".to_string(),
        "-742,-814,-386".to_string(),
        "577,-820,562".to_string(),
        "".to_string(),
        "--- scanner 3 ---".to_string(),
        "-589,542,597".to_string(),
        "605,-692,669".to_string(),
        "-500,565,-823".to_string(),
        "-660,373,557".to_string(),
        "-458,-679,-417".to_string(),
        "-488,449,543".to_string(),
        "-626,468,-788".to_string(),
        "338,-750,-386".to_string(),
        "528,-832,-391".to_string(),
        "562,-778,733".to_string(),
        "-938,-730,414".to_string(),
        "543,643,-506".to_string(),
        "-524,371,-870".to_string(),
        "407,773,750".to_string(),
        "-104,29,83".to_string(),
        "378,-903,-323".to_string(),
        "-778,-728,485".to_string(),
        "426,699,580".to_string(),
        "-438,-605,-362".to_string(),
        "-469,-447,-387".to_string(),
        "509,732,623".to_string(),
        "647,635,-688".to_string(),
        "-868,-804,481".to_string(),
        "614,-800,639".to_string(),
        "595,780,-596".to_string(),
        "".to_string(),
        "--- scanner 4 ---".to_string(),
        "727,592,562".to_string(),
        "-293,-554,779".to_string(),
        "441,611,-461".to_string(),
        "-714,465,-776".to_string(),
        "-743,427,-804".to_string(),
        "-660,-479,-426".to_string(),
        "832,-632,460".to_string(),
        "927,-485,-438".to_string(),
        "408,393,-506".to_string(),
        "466,436,-512".to_string(),
        "110,16,151".to_string(),
        "-258,-428,682".to_string(),
        "-393,719,612".to_string(),
        "-211,-452,876".to_string(),
        "808,-476,-593".to_string(),
        "-575,615,604".to_string(),
        "-485,667,467".to_string(),
        "-680,325,-822".to_string(),
        "-627,-443,-432".to_string(),
        "872,-547,-609".to_string(),
        "833,512,582".to_string(),
        "807,604,487".to_string(),
        "839,-516,451".to_string(),
        "891,-625,532".to_string(),
        "-652,-548,-490".to_string(),
        "30,-46,-14".to_string(),
        ];
    assert_eq!(part1(&v),79);
}

