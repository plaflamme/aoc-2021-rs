use std::{
    collections::{HashMap, HashSet},
    ops::{Add, Index, Mul, Neg, Sub},
};

use itertools::Itertools;

use crate::{Day19, Solver};

sample!(
    Day19,
    "--- scanner 0 ---
404,-588,-901
528,-643,409
-838,591,734
390,-675,-793
-537,-823,-458
-485,-357,347
-345,-311,381
-661,-816,-575
-876,649,763
-618,-824,-621
553,345,-567
474,580,667
-447,-329,318
-584,868,-557
544,-627,-890
564,392,-477
455,729,728
-892,524,684
-689,845,-530
423,-701,434
7,-33,-71
630,319,-379
443,580,662
-789,900,-551
459,-707,401

--- scanner 1 ---
686,422,578
605,423,415
515,917,-361
-336,658,858
95,138,22
-476,619,847
-340,-569,-846
567,-361,727
-460,603,-452
669,-402,600
729,430,532
-500,-761,534
-322,571,750
-466,-666,-811
-429,-592,574
-355,545,-477
703,-491,-529
-328,-685,520
413,935,-424
-391,539,-444
586,-435,557
-364,-763,-893
807,-499,-711
755,-354,-619
553,889,-390

--- scanner 2 ---
649,640,665
682,-795,504
-784,533,-524
-644,584,-595
-588,-843,648
-30,6,44
-674,560,763
500,723,-460
609,671,-379
-555,-800,653
-675,-892,-343
697,-426,-610
578,704,681
493,664,-388
-671,-858,530
-667,343,800
571,-461,-707
-138,-166,112
-889,563,-600
646,-828,498
640,759,510
-630,509,768
-681,-892,-333
673,-379,-804
-742,-814,-386
577,-820,562

--- scanner 3 ---
-589,542,597
605,-692,669
-500,565,-823
-660,373,557
-458,-679,-417
-488,449,543
-626,468,-788
338,-750,-386
528,-832,-391
562,-778,733
-938,-730,414
543,643,-506
-524,371,-870
407,773,750
-104,29,83
378,-903,-323
-778,-728,485
426,699,580
-438,-605,-362
-469,-447,-387
509,732,623
647,635,-688
-868,-804,481
614,-800,639
595,780,-596

--- scanner 4 ---
727,592,562
-293,-554,779
441,611,-461
-714,465,-776
-743,427,-804
-660,-479,-426
832,-632,460
927,-485,-438
408,393,-506
466,436,-512
110,16,151
-258,-428,682
-393,719,612
-211,-452,876
808,-476,-593
-575,615,604
-485,667,467
-680,325,-822
-627,-443,-432
872,-547,-609
833,512,582
807,604,487
839,-516,451
891,-625,532
-652,-548,-490
30,-46,-14",
    "79",
    "3621"
);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Vector([i32; 3]);

impl Vector {
    fn x() -> Self {
        Self([1, 0, 0])
    }
    fn y() -> Self {
        Self([0, 1, 0])
    }
    fn z() -> Self {
        Self([0, 0, 1])
    }
    fn abs(&self) -> Self {
        Self([self[0].abs(), self[1].abs(), self[2].abs()])
    }
    fn cross(&self, rhs: &Vector) -> Self {
        Self([
            self[1] * rhs[2] - self[2] * rhs[1],
            self[2] * rhs[0] - self[0] * rhs[2],
            self[0] * rhs[1] - self[1] * rhs[0],
        ])
    }
    fn dot(&self, rhs: &Vector) -> i32 {
        self[0] * rhs[0] + self[1] * rhs[1] + self[2] * rhs[2]
    }
}

impl Neg for Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Self([self.0[0].neg(), self.0[1].neg(), self.0[2].neg()])
    }
}

impl Add<&Vector> for &Vector {
    type Output = Vector;

    fn add(self, add: &Vector) -> Self::Output {
        Vector([self[0] + add[0], self[1] + add[1], self[2] + add[2]])
    }
}

impl Add<&Vector> for Vector {
    type Output = Vector;

    fn add(self, add: &Vector) -> Self::Output {
        (&self).add(add)
    }
}

impl Add<Vector> for &Vector {
    type Output = Vector;

    fn add(self, add: Vector) -> Self::Output {
        self.add(&add)
    }
}

impl Sub<&Vector> for &Vector {
    type Output = Vector;

    fn sub(self, rhs: &Vector) -> Self::Output {
        self.add(rhs.neg())
    }
}

impl Index<usize> for Vector {
    type Output = i32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index]
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rotation([Vector; 3]);

impl Rotation {
    fn all() -> impl Iterator<Item = Rotation> {
        vec![
            (Vector::x(), [Vector::z(), Vector::y()]),
            (Vector::y(), [Vector::z(), Vector::x()]),
            (Vector::z(), [Vector::x(), Vector::y()]),
        ]
        .into_iter()
        .flat_map(|(facing, ups)| {
            [facing, -facing]
                .into_iter()
                .cartesian_product(ups.into_iter().flat_map(|up| [up, -up]))
                .map(|(dir, up)| {
                    let z_axis = dir;
                    let x_axis = up.cross(&z_axis);
                    let y_axis = z_axis.cross(&x_axis);
                    Rotation([x_axis, y_axis, z_axis])
                })
        })
    }

    fn default() -> Self {
        Self([Vector([1, 0, 0]), Vector([0, 1, 0]), Vector([0, 0, 1])])
    }

    fn transpose(&self) -> Self {
        let r1 = self.0[0];
        let r2 = self.0[1];
        let r3 = self.0[2];
        Rotation([
            Vector([r1[0], r2[0], r3[0]]),
            Vector([r1[1], r2[1], r3[1]]),
            Vector([r1[2], r2[2], r3[2]]),
        ])
    }
}

impl Mul<&Vector> for &Rotation {
    type Output = Vector;

    fn mul(self, v: &Vector) -> Self::Output {
        let r1 = &self.0[0];
        let r2 = &self.0[1];
        let r3 = &self.0[2];

        let x = v[0] * r1[0] + v[1] * r2[0] + v[2] * r3[0];
        let y = v[0] * r1[1] + v[1] * r2[1] + v[2] * r3[1];
        let z = v[0] * r1[2] + v[1] * r2[2] + v[2] * r3[2];
        Vector([x, y, z])
    }
}

impl Mul<&Rotation> for &Rotation {
    type Output = Rotation;

    fn mul(self, v: &Rotation) -> Self::Output {
        let c1 = Vector([v.0[0][0], v.0[1][0], v.0[2][0]]);
        let c2 = Vector([v.0[0][1], v.0[1][1], v.0[2][1]]);
        let c3 = Vector([v.0[0][2], v.0[1][2], v.0[2][2]]);

        let r1 = self.0[0];
        let r2 = self.0[1];
        let r3 = self.0[2];
        Rotation([
            Vector([r1.dot(&c1), r1.dot(&c2), r1.dot(&c3)]),
            Vector([r2.dot(&c1), r2.dot(&c2), r2.dot(&c3)]),
            Vector([r3.dot(&c1), r3.dot(&c2), r3.dot(&c3)]),
        ])
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct CoordinateTransform(Rotation, Vector);

impl CoordinateTransform {
    fn project(&self, v: &Vector) -> Vector {
        &self.0 * v + &self.1
    }

    fn apply(&self, s: &Scanner) -> Scanner {
        Scanner(s.0.iter().map(|v| self.project(v)).collect())
    }

    fn default() -> CoordinateTransform {
        CoordinateTransform(Rotation::default(), Vector([0, 0, 0]))
    }

    fn invert(&self) -> CoordinateTransform {
        let rot = self.0.transpose();
        let v = &rot * &self.1.neg();
        CoordinateTransform(rot, v)
    }
}

impl Add<&CoordinateTransform> for &CoordinateTransform {
    type Output = CoordinateTransform;

    fn add(self, rhs: &CoordinateTransform) -> Self::Output {
        CoordinateTransform(&rhs.0 * &self.0, &self.1 + &self.0 * &rhs.1)
    }
}

#[derive(Clone, Debug)]
pub struct Scanner(Vec<Vector>);

impl Scanner {
    fn re_orient(&self, rot: &Rotation) -> Scanner {
        Scanner(self.0.iter().map(|v| rot * v).collect())
    }
}

// find a coordinate transformation that makes maps s1 coordinates to s0 coordinates
fn find_coordinate_xform(s0: &Scanner, s1: &Scanner) -> Option<CoordinateTransform> {
    let beacons: HashSet<Vector> = s0.0.clone().into_iter().collect();
    Rotation::all().find_map(|rot| {
        // rotate s1
        let s1 = s1.re_orient(&rot);

        // find a translation that makes the 2 scanners overlap with 12 or more beacons
        s0.0.iter()
            .cartesian_product(s1.0.iter())
            .map(|(s0_pt, s1_pt)| s0_pt - s1_pt)
            .find(|v| {
                let t: HashSet<Vector> = s1.0.iter().map(|b1| b1 + v).collect();
                beacons.intersection(&t).count() >= 12
            })
            .map(|v| CoordinateTransform(rot, v))
    })
}

// find all valid transformations from all scanners to all other scanners
//   also computes transitive transformations
fn pairwise_transforms(scanners: Vec<Scanner>) -> HashMap<(usize, usize), CoordinateTransform> {
    let mut result = HashMap::new();
    scanners.iter().enumerate().combinations(2).for_each(|v| {
        let (base, s0) = v[0];
        let (other, s1) = v[1];
        if let Some(ct) = find_coordinate_xform(s0, s1) {
            result.insert((base, other), ct.clone());
            result.insert((other, base), ct.invert());
        }
    });

    // compute transitive transforms
    loop {
        let mut more = HashMap::new();
        for ((b0, t0), cs0) in result.iter() {
            for ((b1, t1), cs1) in result.iter() {
                if t0 == b1 {
                    let key = (*b0, *t1);
                    if !result.contains_key(&key) {
                        more.insert(key, cs0 + cs1);
                    }
                }
            }
        }
        if more.is_empty() {
            break;
        }
        result.extend(more);
    }

    result
}

fn solve_scanner_orientations(scanners: Vec<Scanner>) -> Vec<(Scanner, CoordinateTransform)> {
    let xforms = pairwise_transforms(scanners.clone());
    let base = scanners[0].clone();
    let mut result = vec![(base, CoordinateTransform::default())];
    for (idx, s) in scanners.into_iter().enumerate().skip(1) {
        let cs = xforms.get(&(0, idx)).unwrap();
        result.push((cs.apply(&s), cs.clone()));
    }
    result
}

impl Solver for Day19 {
    type Output = usize;

    type Input = Vec<Scanner>;

    fn parse(input: &str) -> Self::Input {
        crate::tools::empty_line_delimited_batches(input.lines())
            .map(|lines| {
                Scanner(
                    lines
                        .into_iter()
                        .skip(1)
                        .map(|l| {
                            let (x, y, z) =
                                l.split_terminator(',').tuples().exactly_one().ok().unwrap();
                            Vector([
                                x.parse::<i32>().unwrap(),
                                y.parse::<i32>().unwrap(),
                                z.parse::<i32>().unwrap(),
                            ])
                        })
                        .collect(),
                )
            })
            .collect()
    }

    fn part1(input: Self::Input) -> Self::Output {
        solve_scanner_orientations(input)
            .into_iter()
            .flat_map(|(s, _)| s.0)
            .unique()
            .count()
    }

    fn part2(input: Self::Input) -> Self::Output {
        let origins = solve_scanner_orientations(input)
            .into_iter()
            .map(|(_, cs)| cs.1)
            .collect_vec();

        origins
            .clone()
            .into_iter()
            .cartesian_product(origins.iter())
            .map(|(b0, b1)| (&b0 - b1).abs())
            .map(|v| v[0] + v[1] + v[2])
            .max()
            .unwrap() as usize
    }
}

#[cfg(test)]
mod test {
    use crate::Sample;

    use super::*;

    #[test]
    fn test_rotation() {
        assert_eq!(Rotation::all().count(), 24);
        let v = Vector([686, 422, 578]);
        let rotated = Rotation::all().map(|rot| &rot * &v).collect_vec();

        assert_eq!(rotated.iter().unique().count(), 24);
        assert!(rotated.contains(&Vector([-686, 422, -578])));

        let rotations = Rotation::all().collect_vec();
        let r1 = &rotations[3];
        let r2 = &rotations[15];

        let r = r1 * &v;
        let r = r2 * &r;
        let r1_2 = r1 * r2;
        assert_eq!(r, &r1_2 * &v);
    }

    #[test]
    fn test_find_coordinate_system() {
        let s = &Day19::parse(Day19::CONTENT);
        let map = pairwise_transforms(s.clone());

        let cs0_1 = find_coordinate_xform(&s[0], &s[1]).unwrap();
        assert_eq!(cs0_1.1, Vector([68, -1246, -43]));
        assert_eq!(cs0_1.apply(&s[1]).0[0], Vector([-618, -824, -621]));
        assert_eq!(map.get(&(0, 1)).cloned(), Some(cs0_1.clone()));

        let cs1_4 = find_coordinate_xform(&s[1], &s[4]).unwrap();

        let cs0_4 = &cs0_1 + &cs1_4;
        assert_eq!(cs0_4.1, Vector([-20, -1133, 1061]));
        assert!(cs0_4.apply(&s[4]).0.contains(&Vector([459, -707, 401])));
        assert_eq!(map.get(&(0, 4)).cloned(), Some(cs0_4.clone()));

        let cs1_3 = find_coordinate_xform(&s[1], &s[3]).unwrap();
        let cs0_3 = &cs0_1 + &cs1_3;
        assert_eq!(cs0_3.1, Vector([-92, -2380, -20]));
        assert_eq!(map.get(&(0, 3)).cloned(), Some(cs0_3));

        let cs4_2 = find_coordinate_xform(&s[4], &s[2]).unwrap();
        let cs0_2 = &cs0_4 + &cs4_2;
        assert_eq!(cs0_2.1, Vector([1105, -1205, 1229]));
        assert_eq!(map.get(&(0, 2)).cloned(), Some(cs0_2));
    }
}
