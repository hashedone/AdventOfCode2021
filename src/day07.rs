fn count(data:&Vec<String>,part_two:bool)->i64
{
    let tab : Vec<i64> = data[0].split(',')
                                .map(|d| d.parse().unwrap())
                                .collect();

    let (minv,maxv) = (*tab.iter().min().unwrap(),*tab.iter().max().unwrap());

    let sums : Vec<i64> = 
    (0..=maxv-minv)
    .into_iter()
    .scan(0,|acc,x| { *acc+=x; if part_two {Some(*acc)} else {Some(x)}})
    .collect();

    (minv..=maxv)
    .into_iter()
    .map(|p|
    {        
        tab.iter()
           .map(|&x| { sums[i64::abs(x - p) as usize] })
           .sum()
    })
    .min()
    .unwrap()
}

pub fn part1(data:&Vec<String>)->i64
{
    count(data,false)
}

pub fn part2(data:&Vec<String>)->i64
{
    count(data,true)
}

#[allow(unused)]
pub fn solve(data:&Vec<String>)
{    
    println!("Day7");
    println!("part1:{}",part1(data));
    println!("part2:{}",part2(data));
}

#[test]
fn test1()
{
    let v = vec![
        "16,1,2,0,4,2,7,1,2,14".to_string()
        ];
    assert_eq!(part1(&v),37);
}

#[test]
fn test2()
{
    let v = vec![
        "16,1,2,0,4,2,7,1,2,14".to_string()
        ];
    assert_eq!(part2(&v),168);
}

#[test]
fn test3()
{
    let v = vec![
        "1101,1,29,67,1102,0,1,65,1008,65,35,66,1005,66,28,1,67,65,20,4,0,1001,65,1,65,1106,0,8,99,35,67,101,99,105,32,110,39,101,115,116,32,112,97,115,32,117,110,101,32,105,110,116,99,111,100,101,32,112,114,111,103,114,97,109,10,767,559,240,1779,740,550,84,819,108,728,212,650,264,899,1231,90,127,46,219,1494,192,473,163,1456,1048,22,4,1121,9,37,180,531,1167,395,456,952,1807,199,606,293,1003,236,235,1294,102,4,724,672,458,511,431,162,48,900,229,4,570,316,111,520,720,343,1064,314,261,1102,397,1585,610,509,1374,157,128,353,440,435,864,1489,425,762,320,1369,233,903,1181,1203,598,9,938,659,214,2,141,468,1485,631,265,156,165,987,2,856,564,120,325,16,743,849,822,51,18,1153,1100,143,301,402,717,126,937,391,36,802,1399,512,461,250,679,646,303,556,574,948,362,395,402,253,1631,1,203,76,48,246,115,117,15,953,926,210,1257,96,47,147,1058,1129,1166,183,375,1404,697,237,191,760,431,38,1778,159,698,411,160,289,23,836,554,841,289,892,276,877,216,751,1273,388,392,289,78,324,1142,896,767,416,780,373,117,1,71,235,302,307,906,321,480,991,1586,1491,1617,653,499,1323,156,455,19,669,169,415,284,743,439,705,980,1350,210,118,797,81,67,894,183,10,439,950,1126,576,828,85,681,517,19,872,119,164,882,31,244,195,640,41,313,888,303,224,433,462,389,329,254,488,570,286,945,1377,32,1101,206,457,584,662,1176,9,1164,227,467,239,1094,131,420,871,942,52,1276,14,72,7,1099,702,53,118,263,645,181,60,105,780,1102,550,85,225,1159,250,1424,8,1060,458,79,299,744,76,1513,338,68,179,323,644,65,293,66,153,699,819,966,678,207,538,372,284,101,224,305,103,1276,577,316,350,647,314,1256,595,1332,133,83,654,1072,63,446,46,5,92,939,608,940,257,851,1715,742,96,1497,240,1154,30,69,803,13,47,380,97,349,742,581,768,94,454,25,330,899,584,1425,447,207,1621,329,429,22,361,3,24,534,361,695,61,680,517,43,129,1686,301,1090,211,680,362,855,700,392,354,871,154,485,654,203,1417,208,1228,243,317,899,106,307,62,157,186,291,475,616,137,113,1367,24,778,431,1563,36,651,131,259,165,765,226,28,1410,456,1601,11,21,323,214,208,1444,11,108,49,1182,89,564,1266,478,1324,538,1572,488,1546,434,1168,615,285,507,561,100,1092,30,866,946,840,322,625,106,101,157,209,531,63,133,103,715,666,1655,81,1439,1016,32,441,86,1597,1273,443,732,160,162,528,727,150,107,21,111,10,502,302,1315,643,84,318,1488,315,150,5,248,675,167,691,101,412,584,992,1317,18,1046,164,359,111,1105,96,16,301,463,680,443,433,477,420,1141,362,1840,12,57,1094,806,23,708,243,1060,894,403,941,958,240,903,497,1342,1068,35,399,381,19,499,339,0,226,108,292,1607,281,72,283,316,182,224,33,488,786,1456,25,104,201,549,827,890,1520,931,70,763,25,633,464,822,751,327,144,62,1205,78,1007,216,324,316,289,682,1359,198,204,199,29,580,10,338,45,150,217,290,734,985,1654,201,934,0,793,956,549,230,1337,183,115,229,31,122,90,1264,122,1292,278,78,256,919,365,444,455,1235,484,45,1646,21,895,218,179,1311,141,238,1330,40,593,518,95,466,233,125,777,150,315,606,265,935,13,89,961,394,341,88,485,57,725,665,616,889,577,100,154,686,842,772,581,1311,604,41,62,1439,313,320,225,1115,279,176,995,12,70,739,96,4,2,37,252,1164,1243,899,856,10,219,233,1430,443,1011,30,378,81,39,167,1060,9,601,663,89,718,1192,1579,511,85,180,236,1079,556,496,215,192,718,300,1282,475,984,535,1760,1137,439,759,221,125,1298,542,1119,446,204,90,16,84,63,176,26,123,1157,140,518,1115,514,701,1207,547,39,970,240,584,77,66,44,858,560,21,648,309,1096,618,220,28,75,1442,233,1,86,325,244,161,218,6,229,1104,275,754,60,186,882,232,133,1288,42,697,152,252,396,345,38,672,980,1514,468,102,563,871,313,358,97,28,1018,830,182,32,1335,525,490,419,1182,946,362,57,496,799,194,504,1615,440,566,481,283,1422,133,919,185,695,871,1422,1372,250,96,438,743,954,1363,349,814,1235,642,461,160,135,131,61,250,188,125,698,346,470,603,1391,460,578,404,3,14,1715,1271,856,1334,28,739,274,628,70,456,393,5,326,382,70,244,101,560,424,1521,25,1441,147,851,1207,747,84,703,172,101,87,357,421,91,939,595,581,149,626,797,1485,419,192,828,1031,1283,333,614,479,1344,520,1434,1422,877".to_string()
        ];
    assert_eq!(part1(&v),355150);
}


#[test]
fn test4()
{
    let v = vec![
        "1101,1,29,67,1102,0,1,65,1008,65,35,66,1005,66,28,1,67,65,20,4,0,1001,65,1,65,1106,0,8,99,35,67,101,99,105,32,110,39,101,115,116,32,112,97,115,32,117,110,101,32,105,110,116,99,111,100,101,32,112,114,111,103,114,97,109,10,767,559,240,1779,740,550,84,819,108,728,212,650,264,899,1231,90,127,46,219,1494,192,473,163,1456,1048,22,4,1121,9,37,180,531,1167,395,456,952,1807,199,606,293,1003,236,235,1294,102,4,724,672,458,511,431,162,48,900,229,4,570,316,111,520,720,343,1064,314,261,1102,397,1585,610,509,1374,157,128,353,440,435,864,1489,425,762,320,1369,233,903,1181,1203,598,9,938,659,214,2,141,468,1485,631,265,156,165,987,2,856,564,120,325,16,743,849,822,51,18,1153,1100,143,301,402,717,126,937,391,36,802,1399,512,461,250,679,646,303,556,574,948,362,395,402,253,1631,1,203,76,48,246,115,117,15,953,926,210,1257,96,47,147,1058,1129,1166,183,375,1404,697,237,191,760,431,38,1778,159,698,411,160,289,23,836,554,841,289,892,276,877,216,751,1273,388,392,289,78,324,1142,896,767,416,780,373,117,1,71,235,302,307,906,321,480,991,1586,1491,1617,653,499,1323,156,455,19,669,169,415,284,743,439,705,980,1350,210,118,797,81,67,894,183,10,439,950,1126,576,828,85,681,517,19,872,119,164,882,31,244,195,640,41,313,888,303,224,433,462,389,329,254,488,570,286,945,1377,32,1101,206,457,584,662,1176,9,1164,227,467,239,1094,131,420,871,942,52,1276,14,72,7,1099,702,53,118,263,645,181,60,105,780,1102,550,85,225,1159,250,1424,8,1060,458,79,299,744,76,1513,338,68,179,323,644,65,293,66,153,699,819,966,678,207,538,372,284,101,224,305,103,1276,577,316,350,647,314,1256,595,1332,133,83,654,1072,63,446,46,5,92,939,608,940,257,851,1715,742,96,1497,240,1154,30,69,803,13,47,380,97,349,742,581,768,94,454,25,330,899,584,1425,447,207,1621,329,429,22,361,3,24,534,361,695,61,680,517,43,129,1686,301,1090,211,680,362,855,700,392,354,871,154,485,654,203,1417,208,1228,243,317,899,106,307,62,157,186,291,475,616,137,113,1367,24,778,431,1563,36,651,131,259,165,765,226,28,1410,456,1601,11,21,323,214,208,1444,11,108,49,1182,89,564,1266,478,1324,538,1572,488,1546,434,1168,615,285,507,561,100,1092,30,866,946,840,322,625,106,101,157,209,531,63,133,103,715,666,1655,81,1439,1016,32,441,86,1597,1273,443,732,160,162,528,727,150,107,21,111,10,502,302,1315,643,84,318,1488,315,150,5,248,675,167,691,101,412,584,992,1317,18,1046,164,359,111,1105,96,16,301,463,680,443,433,477,420,1141,362,1840,12,57,1094,806,23,708,243,1060,894,403,941,958,240,903,497,1342,1068,35,399,381,19,499,339,0,226,108,292,1607,281,72,283,316,182,224,33,488,786,1456,25,104,201,549,827,890,1520,931,70,763,25,633,464,822,751,327,144,62,1205,78,1007,216,324,316,289,682,1359,198,204,199,29,580,10,338,45,150,217,290,734,985,1654,201,934,0,793,956,549,230,1337,183,115,229,31,122,90,1264,122,1292,278,78,256,919,365,444,455,1235,484,45,1646,21,895,218,179,1311,141,238,1330,40,593,518,95,466,233,125,777,150,315,606,265,935,13,89,961,394,341,88,485,57,725,665,616,889,577,100,154,686,842,772,581,1311,604,41,62,1439,313,320,225,1115,279,176,995,12,70,739,96,4,2,37,252,1164,1243,899,856,10,219,233,1430,443,1011,30,378,81,39,167,1060,9,601,663,89,718,1192,1579,511,85,180,236,1079,556,496,215,192,718,300,1282,475,984,535,1760,1137,439,759,221,125,1298,542,1119,446,204,90,16,84,63,176,26,123,1157,140,518,1115,514,701,1207,547,39,970,240,584,77,66,44,858,560,21,648,309,1096,618,220,28,75,1442,233,1,86,325,244,161,218,6,229,1104,275,754,60,186,882,232,133,1288,42,697,152,252,396,345,38,672,980,1514,468,102,563,871,313,358,97,28,1018,830,182,32,1335,525,490,419,1182,946,362,57,496,799,194,504,1615,440,566,481,283,1422,133,919,185,695,871,1422,1372,250,96,438,743,954,1363,349,814,1235,642,461,160,135,131,61,250,188,125,698,346,470,603,1391,460,578,404,3,14,1715,1271,856,1334,28,739,274,628,70,456,393,5,326,382,70,244,101,560,424,1521,25,1441,147,851,1207,747,84,703,172,101,87,357,421,91,939,595,581,149,626,797,1485,419,192,828,1031,1283,333,614,479,1344,520,1434,1422,877".to_string()
        ];
    assert_eq!(part2(&v),98368490);
}

