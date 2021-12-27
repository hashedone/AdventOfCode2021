use std::collections::{HashSet,HashMap};
use crate::tools;

#[derive(Debug,Clone,PartialEq, Eq, PartialOrd, Ord)]
struct Scanner
{
          beams : Vec<(i64,i64,i64)>,
             id : i64,
              x : i64,
              y : i64,
              z : i64,
          trans : i64,
           used : bool,
         points : Vec<(i64,i64,i64)>,
     all_points : Vec<Vec<(i64,i64,i64)>>
}

fn inverse_matrix(m:Vec<(i64,i64,i64)>)->Vec<(i64,i64,i64)>
{
    let m00:f32 = m[0].0 as f32;
    let m01:f32 = m[0].1 as f32;
    let m02:f32 = m[0].2 as f32;
    let m10:f32 = m[1].0 as f32;
    let m11:f32 = m[1].1 as f32;
    let m12:f32 = m[1].2 as f32;
    let m20:f32 = m[2].0 as f32;
    let m21:f32 = m[2].1 as f32;
    let m22:f32 = m[2].2 as f32;
        
    let det : f32 = m00 * m11 * m22 - m21 * m12 -
                    m01 * m10 * m22 - m12 * m20 +
                    m02 * m10 * m21 - m11 * m20;

    let invdet : f32 = 1.0 / det;
    
    let mn00 = (m11 * m22 - m21 * m12) * invdet;
    let mn01 = (m02 * m21 - m01 * m22) * invdet;
    let mn02 = (m01 * m12 - m02 * m11) * invdet;
    let mn10 = (m12 * m20 - m10 * m22) * invdet;
    let mn11 = (m00 * m22 - m02 * m20) * invdet;
    let mn12 = (m10 * m02 - m00 * m12) * invdet;
    let mn20 = (m10 * m21 - m20 * m11) * invdet;
    let mn21 = (m20 * m01 - m00 * m21) * invdet;
    let mn22 = (m00 * m11 - m10 * m01) * invdet;
    
    vec![(mn00 as i64,mn01 as i64,mn02 as i64),
         (mn10 as i64,mn11 as i64,mn12 as i64),
         (mn20 as i64,mn21 as i64,mn22 as i64)]
}

fn get_inverse_matrix(id:usize)->Vec<(i64,i64,i64)>
{
    inverse_matrix(get_view_matrix(id))
}

//matrices from http://www.euclideanspace.com/maths/algebra/matrix/transforms/examples/index.htm
fn get_view_matrix(id:usize)->Vec<(i64,i64,i64)>
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
    fn new(id:i64)->Self
    {
        Self {
            id,
            beams: Vec::new(),
            x:0, y:0, z:0,
            used : false,
            trans : -1,
            points: Vec::new(),
            all_points: Vec::new(),
        }
    }

    fn dist(p1:(i64,i64,i64),p2:(i64,i64,i64))->usize
    {
        (p2.0-p1.0).abs() as usize +
        (p2.1-p1.1).abs() as usize +
        (p2.2-p1.2).abs() as usize
    }

    fn dist_ok(p1:(i64,i64,i64),p2:(i64,i64,i64))->bool
    {
        (p2.0-p1.0).abs()<=1000 &&
        (p2.1-p1.1).abs()<=1000 &&
        (p2.2-p1.2).abs()<=1000
    }

    fn get_xyz(x:i64,y:i64,z:i64,t:i64)->i64
    {
        match t {
             1 =>  x,
             2 =>  y,
             3 =>  z,
            -1 => -x,
            -2 => -y,
            -3 => -z,
            _  => panic!("eee")
        }
    }

    fn get_point(x:i64,y:i64,z:i64,transform:(i64,i64,i64))->(i64,i64,i64)
    {    
        (
            Scanner::get_xyz(x,y,z,transform.0),
            Scanner::get_xyz(x,y,z,transform.1),
            Scanner::get_xyz(x,y,z,transform.2)
        )
    }

    fn transform(p:(i64,i64,i64),mtx:&[(i64,i64,i64)])->(i64,i64,i64)
    {
        let x = p.0*mtx[0].0 + p.1*mtx[0].1 + p.2*mtx[0].2;
        let y = p.0*mtx[1].0 + p.1*mtx[1].1 + p.2*mtx[1].2;
        let z = p.0*mtx[2].0 + p.1*mtx[2].1 + p.2*mtx[2].2;
        (x,y,z)
    }

    fn get_trans(p:(i64,i64,i64),transform_id:usize)->(i64,i64,i64)
    {
        let x = p.0;
        let y = p.1;
        let z = p.2;

        match transform_id 
        {
             0 => ( x,  y,  z),
             1 => ( x, -z,  y),
             2 => ( x, -y, -z),
             3 => ( x,  z, -y),
             4 => (-y,  x,  z),
             5 => ( z,  x,  y),
             6 => ( y,  x, -z),
             7 => (-z,  x, -y),
             8 => (-x, -y,  z),
             9 => (-x, -z, -y),
            10 => (-x,  y, -z),
            11 => (-x,  z,  y),
            12 => ( y, -x,  z),
            13 => ( z, -x, -y),
            14 => (-y, -x, -z),
            15 => (-z, -x,  y),
            16 => (-z,  y,  x),
            17 => ( y,  z,  x),
            18 => ( z, -y,  x),
            19 => (-y, -z,  x),
            20 => (-z, -y, -x),
            21 => (-y,  z, -x),
            22 => ( z,  y, -x),
            23 => ( y, -z, -x),
            _ => {panic!("error")}
        } 

    }

    fn transformp(&mut self,tr: &Vec<(i64,i64,i64)>)
    {
        self.all_points = vec![];

        for i in 0..24 
        {
            let mtx = &get_view_matrix(i);//get_inverse_matrix(i);
            let new_points = self.beams.iter().map( |p|
                    Scanner::get_trans(*p,i)
            ).collect();

            self.all_points.push(new_points);
            self.points = self.beams.clone();
        }


    }
    
    fn claim_found(&mut self,new_points:&Vec<(i64,i64,i64)>,dx:i64,dy:i64,dz:i64)
    {
        self.x      = dx;
        self.y      = dy;
        self.z      = dz;
        self.used   = true;
    }

    fn match_points(&self,s1_points:&HashSet<(i64,i64,i64)>,s2_points:&Vec<(i64,i64,i64)>)->(bool,i64,i64,i64)
    {
        let mut points = vec![];
        for p in s1_points.iter() 
        {
            points.push((p.0,p.1,p.2));
        }

        let mut h = HashMap::new();

        for p1 in points.iter() {
            for p2 in s2_points {
                let dx = p1.0 - p2.0;
                let dy = p1.1 - p2.1;
                let dz = p1.2 - p2.2;

                let v = *h.get(&(dx,dy,dz)).unwrap_or(&0);
                h.insert((dx,dy,dz),   v+1);
            }
        }
        if h.values().into_iter().any(|v| *v>=12)
        {
            for (p,v) in h.into_iter()
            {
                if v>=12
                {
                    return (true,p.0,p.1,p.2);
                }                
            }
        }

        (false,0,0,0)
    }
    
    fn get_points(&self,id:usize)->&Vec<(i64,i64,i64)> {
        &self.all_points[id]
    }

    #[allow(unused)]
    fn min_max_x(&mut self)->(i64,i64)
    {
        let min_x = self.beams.iter().map(|v| v.0).min().unwrap();
        let max_x = self.beams.iter().map(|v| v.0).max().unwrap();
        for i in &mut self.beams
        {
            (*i).0 -= min_x;
        }
        
        //self.beams.sort_unstable();
        (min_x,max_x)
    }

    #[allow(unused)]
    fn common(&self,s:&Scanner)->i64
    {
        let mut h : HashSet<i64> = HashSet::new();
        for b in &self.beams {
            h.insert(b.0);
        }
        let mut cnt=0;
        for b2 in &s.beams {
            if h.contains(&b2.0) { cnt+=1; }
        }
        cnt
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

fn add_points(points:&mut HashSet<(i64,i64,i64)>,beams:&[(i64,i64,i64)],dx:i64,dy:i64,dz:i64)
{
    for p in beams {
        let x = p.0 + dx;
        let y = p.1 + dy;
        let z = p.2 + dz;
        points.insert((x,y,z));
    }
}



fn gen_transforms()->Vec<(i64,i64,i64)>
{
    let mut res = vec![];
    
    for signs in 0..8 {
    for x_pos in 0..3 {
    for y_pos in 0..3 {
    for z_pos in 0..3 {
        if x_pos!=y_pos && 
           y_pos!=z_pos && 
           x_pos!=z_pos {
            let mut x = x_pos+1;
            let mut y = y_pos+1;
            let mut z = z_pos+1;
            if signs&1!=0 { x*=-1; }
            if signs&2!=0 { y*=-1; }
            if signs&4!=0 { z*=-1; }
            res.push((x,y,z));

            if x==1 && y==2 && z==3 {
                //println!("neutral:{}",res.len()-1);
            }
            //println!("[{},{},{}]",x,y,z);
        }
    }}}}

    res.sort();
    res.dedup();
    res
}

fn compute(data:&[String])->(usize,usize)
{
    let mut scanners: Vec<Scanner> = Vec::new();

    for l in data 
    {
        if l.contains("--- scanner")
        {            
            let id = tools::i32_get_between(l,"--- scanner "," ---") as i64;
            scanners.push(Scanner::new(id));
        }
        else if l.contains(',')
        {
            let tab = l.split(',').collect::<Vec<&str>>();
            let x = tab[0].parse::<i64>().unwrap();
            let y = tab[1].parse::<i64>().unwrap();
            let z = tab[2].parse::<i64>().unwrap();
            let s = scanners.last_mut().unwrap();
            s.beams.push((x,y,z));
        }
    }
    
    let tr = gen_transforms();
    let mut final_points = HashSet::new();

    for f in scanners.iter_mut()
    {
        f.transformp(&tr);
    }


    let s0_points = scanners[0].get_points(0).clone();

    scanners[0].claim_found(&s0_points,0,0,0);
    add_points(&mut final_points,&scanners[0].points,0,0,0);

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
                        b.claim_found(&new_points,dx,dy,dz);
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

    return (final_points.len(),max_dist);
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

#[test]
fn testr2()
{
    let v = vec![
    ];
    assert_eq!(part2(&v),900);
}

#[test]
fn test_mtx0()
{
    assert_eq!(
    inverse_matrix(vec![( 1, 0, 0),
                        ( 0, 1, 0),
                        ( 0, 0, 1)]),
                   vec![( 1, 0, 0),
                        ( 0, 1, 0),
                        ( 0, 0, 1)]
    );
}

#[test]
fn test_mtx1()
{
    assert_eq!(
    inverse_matrix(vec![( 0, 0, 1),
                        ( 0,-1, 0),
                        ( 1, 0, 0)]),
                   vec![( 0, 0, 1),
                        ( 0,-1, 0),
                        ( 1, 0, 0)]
    );
}

#[test]
fn test_mtx2()
{
    assert_eq!(
    inverse_matrix(vec![( 0,-1, 0),
                        ( 0, 0,-1),
                        ( 1, 0, 0)]),
                   vec![( 0, 0, 1),
                        (-1, 0, 0),
                        ( 0,-1, 0)]
    );
}

#[test]
fn test_mtx3()
{
    assert_eq!(
    inverse_matrix(vec![( 0,-1, 0),
                        ( 1, 0, 0),
                        ( 0, 0, 1)]),
                   vec![( 0, 1, 0),
                        (-1, 0, 0),
                        ( 0, 0, 1)]
    );
}

#[test]
fn test_mtx4()
{
    assert_eq!(
    inverse_matrix(vec![( 0,-1, 0),
                        ( 0, 0, 1),
                        (-1, 0, 0)]),
                   vec![( 0, 0,-1),
                        (-1, 0, 0),
                        ( 0, 1, 0)]
    );
}
