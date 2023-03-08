struct RubikFace {
    face: [u8; 9],
}
struct RubikCube {
    front: RubikFace,
    back: RubikFace,
    left: RubikFace,
    right: RubikFace,
    top: RubikFace,
    bottom: RubikFace,
}
trait RubikCubeTrait {
    fn new() -> Self;
    fn f(&mut self); //white
    fn u(&mut self); //orange
    fn r(&mut self); //blue
    fn b(&mut self); //yellow
    fn l(&mut self); //green
    fn d(&mut self); //red
    fn get_face(&self, face: &RubikFace) -> [u8; 9]; // 3x3 array of u8
    fn move_cube(&mut self, sequence: &str); //sequence of moves
}
fn get_faces(cube: &RubikCube) -> ([u8; 9], [u8; 9], [u8; 9], [u8; 9], [u8; 9], [u8; 9]) {
    let top = cube.get_face(&cube.top);
    let bottom = cube.get_face(&cube.bottom);
    let left = cube.get_face(&cube.left);
    let right = cube.get_face(&cube.right);
    let front = cube.get_face(&cube.front);
    let back = cube.get_face(&cube.back);
    (top, bottom, left, right, front, back)
}
fn sequence_swapper(
    face: &mut RubikFace,
    other_face: &mut RubikFace,
    sequence: [(usize, usize); 3],
) {
    let mut tmp = face.face;
    for i in 0..3 {
        tmp[sequence[i].0] = other_face.face[sequence[i].1];
    }
    for i in 0..3 {
        other_face.face[sequence[i].1] = face.face[sequence[i].0];
    }
    face.face = tmp;
}
fn rotate_face(face: &mut RubikFace) {
    let mut tmp = face.face;
    tmp[0] = face.face[6];
    tmp[1] = face.face[3];
    tmp[2] = face.face[0];
    tmp[3] = face.face[7];
    tmp[4] = face.face[4];
    tmp[5] = face.face[1];
    tmp[6] = face.face[8];
    tmp[7] = face.face[5];
    tmp[8] = face.face[2];
    face.face = tmp;
}
impl RubikCubeTrait for RubikCube {
    fn new() -> Self {
        RubikCube {
            front: RubikFace {
                //White
                face: [1, 2, 3, 4, 5, 6, 7, 8, 9],
            },
            back: RubikFace {
                //Yellow
                face: [10, 11, 12, 13, 14, 15, 16, 17, 18],
            },
            left: RubikFace {
                //Green
                face: [19, 20, 21, 22, 23, 24, 25, 26, 27],
            },
            right: RubikFace {
                //Blue
                face: [28, 29, 30, 31, 32, 33, 34, 35, 36],
            },
            top: RubikFace {
                //Orange
                face: [37, 38, 39, 40, 41, 42, 43, 44, 45],
            },
            bottom: RubikFace {
                //Red
                face: [46, 47, 48, 49, 50, 51, 52, 53, 54],
            },
        }
    }
    fn r(&mut self) {
        rotate_face(&mut self.right);
        sequence_swapper(&mut self.front, &mut self.top, [(2, 6), (5, 3), (8, 0)]);
        sequence_swapper(&mut self.front, &mut self.bottom, [(2, 2), (5, 5), (8, 8)]);
        sequence_swapper(&mut self.front, &mut self.left, [(2, 2), (5, 5), (8, 8)]);
    }
    fn f(&mut self) {
        rotate_face(&mut self.front);
        sequence_swapper(&mut self.left, &mut self.back, [(0, 0), (1, 1), (2, 2)]);
        sequence_swapper(&mut self.right, &mut self.top, [(0, 0), (1, 1), (2, 2)]);
        sequence_swapper(&mut self.left, &mut self.top, [(0, 0), (1, 1), (2, 2)]);
    }
    fn u(&mut self) {
        rotate_face(&mut self.top);
        sequence_swapper(&mut self.right, &mut self.front, [(2, 0), (5, 1), (8, 2)]);
        sequence_swapper(&mut self.back, &mut self.bottom, [(0, 6), (3, 7), (6, 8)]);
        sequence_swapper(&mut self.back, &mut self.right, [(6, 2), (3, 5), (0, 8)]);
        //ok
    }
    fn b(&mut self) {
        //Check if needs reverse
        rotate_face(&mut self.back);
        sequence_swapper(&mut self.left, &mut self.bottom, [(0, 0), (3, 3), (6, 6)]);
        sequence_swapper(&mut self.front, &mut self.top, [(6, 2), (3, 5), (0, 8)]);
        sequence_swapper(&mut self.left, &mut self.top, [(0, 8), (3, 5), (6, 2)]);
        //ok
    }
    fn l(&mut self) {
        rotate_face(&mut self.left);
        sequence_swapper(&mut self.front, &mut self.right, [(6, 0), (7, 3), (8, 6)]);
        sequence_swapper(&mut self.bottom, &mut self.back, [(0, 2), (1, 5), (2, 8)]);
        sequence_swapper(&mut self.bottom, &mut self.front, [(0, 8), (1, 7), (2, 6)]);
    }
    fn d(&mut self) {
        rotate_face(&mut self.bottom);
        sequence_swapper(&mut self.back, &mut self.left, [(6, 6), (7, 7), (8, 8)]);
        sequence_swapper(&mut self.right, &mut self.top, [(6, 6), (7, 7), (8, 8)]);
        sequence_swapper(&mut self.back, &mut self.right, [(6, 6), (7, 7), (8, 8)]);
    }
    fn get_face(&self, face: &RubikFace) -> [u8; 9] {
        face.face
    }
    fn move_cube(&mut self, sequence: &str) {
        //Read char by char, but check if next char is ' or 2
        let mut i = 0;
        while i < sequence.len() {
            let mut reverse = false;
            let mut double = false;
            let char = sequence.chars().nth(i).unwrap();
            if i + 1 < sequence.len() {
                let next_char = sequence.chars().nth(i + 1).unwrap();
                if next_char == '\'' {
                    reverse = true;
                    i += 1;
                } else if next_char == '2' {
                    double = true;
                    i += 1;
                }
            }
            //println!("{} {} {}", char, reverse, double);
            match char {
                'R' => {
                    self.r();
                    if reverse {
                        self.r();
                        self.r();
                    }
                    if double {
                        self.r();
                    }
                }
                'F' => {
                    self.f();
                    if reverse {
                        self.f();
                        self.f();
                    }
                    if double {
                        self.f();
                    }
                }
                'U' => {
                    self.u();
                    if reverse {
                        self.u();
                        self.u();
                    }
                    if double {
                        self.u();
                    }
                }
                'B' => {
                    self.b();
                    if reverse {
                        self.b();
                        self.b();
                    }
                    if double {
                        self.b();
                    }
                }
                'L' => {
                    self.l();
                    if reverse {
                        self.l();
                        self.l();
                    }
                    if double {
                        self.l();
                    }
                }
                'D' => {
                    self.d();
                    if reverse {
                        self.d();
                        self.d();
                    }
                    if double {
                        self.d();
                    }
                }
                _ => {}
            }
            i += 1;
        }
    }
}

fn main() {
    let vals = ["R", "RR2", "RU", "R2R'R'"];
    let mut cube = RubikCube::new();
    let actual = get_faces(&cube); //Start
    for val in vals {
        let mut count = 0;
        //ask input
        let start = std::time::Instant::now();
        loop {
            count += 1;
            cube.move_cube(val);
            if get_faces(&cube) == actual {
                break;
            }
        }
        println!("{} μs", start.elapsed().as_micros());
        println!("{} turns for {}", count, val);
    }
}
