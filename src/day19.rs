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
     used : bool,
}

fn inverse_matrix(m:Vec<(i32,i32,i32)>)->Vec<(i32,i32,i32)>
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
    
    vec![(mn00 as i32,mn01 as i32,mn02 as i32),
         (mn10 as i32,mn11 as i32,mn12 as i32),
         (mn20 as i32,mn21 as i32,mn22 as i32)]
}

fn get_inverse_matrix(id:usize)->Vec<(i32,i32,i32)>
{
    inverse_matrix(get_view_matrix(id))
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
            beams: Vec::new(),
            x:0,y:0,z:0,
            used:false
        }
    }

    fn dist(p1:(i32,i32,i32),p2:(i32,i32,i32))->i32
    {
        (p2.0-p1.0).abs() +
        (p2.1-p1.1).abs() +
        (p2.2-p1.2).abs()
    }


    fn get_fingerprint(&self)->HashMap<i32,usize>
    {
        let mut r : HashMap<i32,usize> = HashMap::new();

        for a in 0..self.beams.len()
        {
            let mut min_d = i32::MAX;
            let mut min_d1 = i32::MAX;
            let mut min_d2 = i32::MAX;

            for b in a+1..self.beams.len()
            {
                for c in a+1..self.beams.len()
                {
                    let d1 = Scanner::dist(self.beams[a],self.beams[b]);
                    let d2 = Scanner::dist(self.beams[a],self.beams[c]);
                    
                    if d1+d2<min_d 
                    {
                        min_d  = d1+d2;
                        min_d1 = d1;
                        min_d2 = d2;
                    }
                }                    
            }
            let area= (min_d2<<16) + min_d1;
            let v = *r.get(&area).unwrap_or(&0) as usize + 1;
            r.insert(area,v);
        }
        r
    }

    fn set_pos(&mut self,x:i32,y:i32,z:i32)
    {
        self.x = x;
        self.y = y;
        self.z = z;
    }

    fn set_points(&mut self,po:&Scanner)
    {
        self.beams = po.beams.clone();
        //scanners[b].set_points(&m);
    }

    fn transform(&mut self,mtx:&[(i32,i32,i32)])
    {
        for p in self.beams.iter_mut() 
        {
//            let x = p.0*mtx[0].0 + p.1*mtx[0].1 + p.2*mtx[0].2;
//            let y = p.0*mtx[1].0 + p.1*mtx[1].1 + p.2*mtx[1].2;
//            let z = p.0*mtx[2].0 + p.1*mtx[2].1 + p.2*mtx[2].2;
            let x = p.0*mtx[0].0 + p.0*mtx[1].0 + p.0*mtx[2].0;
            let y = p.1*mtx[0].1 + p.1*mtx[1].1 + p.1*mtx[2].1;
            let z = p.2*mtx[0].2 + p.2*mtx[1].2 + p.2*mtx[2].2;

            *p = (x,y,z);
        }
    }

    #[allow(unused)]
    fn min_max_x(&mut self)->(i32,i32)
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
    fn common(&self,s:&Scanner)->i32
    {
        let mut h : HashSet<i32> = HashSet::new();
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

fn add_points(points:&mut HashSet<(i32,i32,i32)>,beams:&[(i32,i32,i32)],dx:i32,dy:i32,dz:i32)
{
    for p in beams {
        let x = p.0 + dx;
        let y = p.1 + dy;
        let z = p.2 + dz;
        points.insert((x,y,z));
    }
}

fn find(points:&HashSet<(i32,i32,i32)>,b:&Scanner,scaner_pos:(i32,i32,i32))->(bool,i32,i32,i32,usize)
{
    let mut ss = 1;
    let mut best_cnt = 0;
    let mut best_x = 0;
    let mut best_y = 0;
    let mut best_z = 0;
    
    for p1 in points {
        for p2 in &b.beams {
            let dx = p1.0-p2.0;
            let dy = p1.1-p2.1;
            let dz = p1.2-p2.2;      
            
               // if dx.abs()<=1000 && dy.abs()<=1000 && dz.abs()<=1000
                {
                    let cnt = b.beams.iter().filter(|&a| 
                        points.contains(&(a.0+dx,
                                          a.1+dy,
                                          a.2+dz))).count();

                    if cnt>ss 
                    {
                        let cnt_wrong = b.beams.iter().filter(|&a| 
                            points.contains(&(a.0+dx,a.1+dy,a.2+dz)) && 
                                (i32::abs(a.0+dx-p1.0)<=1000) &&
                                (i32::abs(a.1+dy-p1.1)<=1000) &&
                                (i32::abs(a.2+dz-p1.2)<=1000)
                        ).count();
                            
                        // if !b.beams.iter().any(|p| (p.0+dx).abs()>1000 || (p.1+dy).abs()>1000 || (p.2+dz).abs()>1000)                

                        if cnt_wrong<=12
                        {
                            if cnt>ss 
                            {
                                ss= cnt;
                                println!("<{}>",ss)
                            }
                            if cnt<1 {
                                panic!("why?");
                            }

                            if cnt>best_cnt {
                                best_cnt = cnt;
                                best_x = dx;
                                best_y = dy;
                                best_z = dz;
                            }
                        }
                    }
                }

//                    }
//                }
//            }

            //if cnt>1
            //{
                //println!("{}",cnt);
            //}
            
        }
    }
    if best_cnt>=12
    {
        (true,best_x,best_y,best_z,best_cnt)
    }
    else {
        (false,0,0,0,0)
    }

}

fn find2(a:&Scanner,b:&Scanner,scaner_pos:(i32,i32,i32))->(bool,i32,i32,i32,usize)
{
    let mut points: HashSet<(i32,i32,i32)> = HashSet::new();

    for p in a.beams.iter() {
        points.insert((p.0,p.1,p.2));
    }

    let scanner1_x = a.x;
    let scanner1_y = a.y;
    let scanner1_z = a.z;

    let bc = b.beams.clone();
    
    for p1 in points.clone() {
        for p2 in &b.beams {
            let dx = p1.0-p2.0;
            let dy = p1.1-p2.1;
            let dz = p1.2-p2.2;      
            
                if (scanner1_x-dx).abs()<=2000 && 
                   (scanner1_y-dy).abs()<=2000 && 
                   (scanner1_z-dz).abs()<=2000 
                {
                    let common:Vec<(i32,i32,i32)> = bc.clone().into_iter().filter(|&s|
                        {
                            let px = s.0+dx;
                            let py = s.1+dy;
                            let pz = s.2+dz;

                            points.contains(&(px,py,pz)) &&
                            (px-scanner1_x).abs() <=1000 &&
                            (py-scanner1_y).abs() <=1000 &&
                            (pz-scanner1_z).abs() <=1000 &&
                            s.0<=1000 &&
                            s.1<=1000 &&
                            s.2<=1000 
                        }).collect();//::<Vec(i32,i32,i32)>();

                    let cnt = common.len();

                    if cnt>=10
                    {
                        /*
                        let cnt_wrong = b.beams.iter().filter(|&a| 
                            !points.contains(&(a.0+dx,a.1+dy,a.2+dz)) && 
                                (i32::abs(a.0+dx-p1.0)<=1000) &&
                                (i32::abs(a.1+dy-p1.1)<=1000) &&
                                (i32::abs(a.2+dz-p1.2)<=1000)
                        ).count(); */

                        //if cnt_wrong==0 
                        //{
                        return (true,dx,dy,dz,cnt);
                        //}
                    }    
                        // if !b.beams.iter().any(|p| (p.0+dx).abs()>1000 || (p.1+dy).abs()>1000 || (p.2+dz).abs()>1000)                
/*
                        if cnt_wrong<=12
                        {
                            if cnt>ss 
                            {
                                ss= cnt;
                                println!("<{}>",ss)
                            }
                            if cnt<1 {
                                panic!("why?");
                            }

                            if cnt>best_cnt {
                                best_cnt = cnt;
                                best_x = dx;
                                best_y = dy;
                                best_z = dz;
                            }
                        }
                        */
                    }
                }

//                    }
//                }
//            }

            //if cnt>1
            //{
                //println!("{}",cnt);
            //}
            
     }
    
    //if best_cnt>=12
    //{
      //  (true,best_x,best_y,best_z,best_cnt)
    //}
    //else {
        (false,0,0,0,0)
    //}

}



pub fn part1(data:&[String])->usize
{
    let mut scanners: Vec<Scanner> = Vec::new();

    for l in data 
    {
        if l.contains("--- scanner")
        {            
            let id = tools::i32_get_between(l,"--- scanner "," ---");
            scanners.push(Scanner::new(id));
            //"649,640,665".to_string(),
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

    let mut points = HashSet::new();
    let mut found = 0;
    let mut tick=0;


    add_points(&mut points,&scanners[0].beams,0,0,0);
    scanners[0].used = true;
    let mut offx = 0;
    let mut offy = 0;
    let mut offz = 0;


   // let mut hs = HashSet::new();
/*
    for f in scanners
    {
        let hh = f.get_fingerprint();
        for f in hh.keys() 
        {
            hs.insert(f);
        }
    }
   */  
    return 0;//hs.len();
    
    /*
    for i in 0..6 {
        for a in 0..scanners.len()
        {
            if scanners[a].used
            {
            for b in 0..scanners.len()            
            {
                if !scanners[b].used && a!=b
                {
                    let aa = &scanners[a];
                    println!("{} {}",a,b);

                    for i in 0..24 
                    {
                        let mut m = scanners[b].clone();                
                        m.transform(&get_inverse_matrix(i));

                        let res = find2(aa, &m, (aa.x, aa.y, aa.z));

                        if res.0
                        {
                            print!("added scaner:{}->{} count={} ",a,b,res.4);

                            add_points(&mut points,&m.beams,res.1,res.2,res.3);
                            scanners[b].x =  offx +res.1;
                            scanners[b].y =  offy +res.2;
                            scanners[b].z =  offz +res.3;

                            println!("at pos {},{},{}",scanners[b].x,scanners[b].y,scanners[b].z);

                            scanners[b].set_points(&m);
                            offx += res.1;
                            offy += res.2;
                            offz += res.3;
                            scanners[b].used = true;
                            break;
                        }    
                    }
                }
            }
        }
        }
    }
 */
    /*
    
    while found<scanners.len()
    {
        if tick>scanners.len() { break; }
        tick+=1;

        for (b_id,b) in scanners.iter_mut()
                                                  .filter(|s|!s.used)
                                                  .enumerate()
        {
            //println!("{}",b_id);
            if points.is_empty()
            {
                add_points(&mut points,&b.beams,0,0,0);
                println!("p0 --- {}",points.len());
                found+=1;
                b.used = true;
            }
            else
            {
                let mut win = false;
                let mut best = 0;
                let mut best_off = (0,0,0);
                let mut best_set : Vec<(i32,i32,i32)> = Vec::new();
                
                for i in 0..24 
                {
                    let mut m = b.clone();                
                    m.transform(&get_inverse_matrix(i));
                    let (success,dx,dy,dz,count) = find(&points,&m,(0,0,0));

                    if success && count>best && dx<=1000 && dy<=1000 && dz<=1000
                    {
                        best = count;
                        best_off = (dx,dy,dz);
                        best_set = m.beams.clone();
                        win = true;
                    }                                    
                }

                //println!("s: {},{},{}",dx,dy,dz);
                if win 
                {
                    b.used = true;
                    add_points(&mut points,&best_set,best_off.0,best_off.1,best_off.2);
                    println!("id {} sca {:?}",b_id,best_off);
                    scanners[b_id].set_pos(best_off.0,best_off.1,best_off.2);
                    println!("{:?}",best_set);
                    found+=1;
                    println!("p0 --- {}",points.len());
                }
                else
                {
                    print!(".");//{} fail",b_id);
                }
            }
            //b.min_max_x();
            //b.print();
        }
    }
    */
    //for (ai,a) in scanners.iter().enumerate() {
    //    for (bi,b) in scanners.iter().enumerate() {
    //        if a!=b {
    //            println!("{}->{}={}",ai,bi,a.common(b));
    //        }
    //    }        
    //}
    //points.len()
}

pub fn part2(data:&[String])->i32
{
    let d = &data[1].parse::<i32>().unwrap();
    *d
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



/*

( 0, 0,-1),
( 0,-1, 0),
(-1, 0, 0),



[( 0, 0, 1),
( 0, 1, 0),
(-1, 0, 0)],
 */