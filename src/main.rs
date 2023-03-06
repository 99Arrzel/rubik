use std::fmt::format;

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
    fn f(&mut self);
    fn u(&mut self);
    fn r(&mut self);
    fn b(&mut self);
    fn l(&mut self);
    fn d(&mut self);
    fn get_face(&self, face: &RubikFace) -> [u8; 9]; // 3x3 array of u8
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
        //Visual help https://ruwix.com/online-puzzle-simulators/
        RubikCube {
            front: RubikFace {
                face: [1, 2, 3, 4, 5, 6, 7, 8, 9],
            },
            back: RubikFace {
                face: [10, 11, 12, 13, 14, 15, 16, 17, 18],
            },
            left: RubikFace {
                face: [19, 20, 21, 22, 23, 24, 25, 26, 27],
            },
            right: RubikFace {
                face: [28, 29, 30, 31, 32, 33, 34, 35, 36],
            },
            top: RubikFace {
                face: [37, 38, 39, 40, 41, 42, 43, 44, 45],
            },
            bottom: RubikFace {
                face: [46, 47, 48, 49, 50, 51, 52, 53, 54],
            },
        }
    }
    fn r(&mut self) {
        rotate_face(&mut self.right);
        //Dictionary to swap the faces
    }
    fn f(&mut self) {}
    fn u(&mut self) {}
    fn b(&mut self) {}
    fn l(&mut self) {}
    fn d(&mut self) {}
    fn get_face(&self, face: &RubikFace) -> [u8; 9] {
        face.face
    }
}

fn get_color(val: u8) -> String {
    if val < 10 {
        return format!("⬜");
    }
    if val < 19 {
        return format!("🟧");
    }
    if val < 28 {
        return format!("🟩");
    }
    if val < 37 {
        return format!("🟥");
    }
    if val < 46 {
        return format!("🟦");
    }
    return format!("🟨");
}

fn printer(
    top: [u8; 9],
    bottom: [u8; 9],
    left: [u8; 9],
    right: [u8; 9],
    front: [u8; 9],
    back: [u8; 9],
    letters: bool,
) {
    for i in 0..3 {
        for j in 0..6 {
            if j > 2 {
                print!(
                    "{}",
                    if letters {
                        format!("|{}|", front[i * 3 + (j - 3)])
                    } else {
                        get_color(front[i * 3 + (j - 3)])
                    }
                );
            }
            if j <= 2 {
                if letters {
                    print!("|⬛|");
                } else {
                    print!("⬛");
                }
            }
        }
        println!();
    }
    for i in 0..3 {
        for j in 0..12 {
            if j <= 2 {
                print!(
                    "{}",
                    if letters {
                        format!("|{}|", back[i * 3 + (j)])
                    } else {
                        get_color(back[i * 3 + (j)])
                    }
                );
            }
            if j > 2 && j <= 5 {
                print!(
                    "{}",
                    if letters {
                        format!("|{}|", left[i * 3 + (j - 3)])
                    } else {
                        get_color(left[i * 3 + (j - 3)])
                    }
                );
            }
            if j > 5 && j <= 8 {
                print!(
                    "{}",
                    if letters {
                        format!("|{}|", right[i * 3 + (j - 6)])
                    } else {
                        get_color(right[i * 3 + (j - 6)])
                    }
                );
            }
            if j > 8 && j <= 11 {
                print!(
                    "{}",
                    if letters {
                        format!("|{}|", top[i * 3 + (j - 9)])
                    } else {
                        get_color(top[i * 3 + (j - 9)])
                    }
                );
            }
        }
        println!();
    }
    for i in 0..3 {
        for j in 0..6 {
            if j > 2 {
                print!(
                    "{}",
                    if letters {
                        format!("|{}|", bottom[i * 3 + (j - 3)])
                    } else {
                        get_color(bottom[i * 3 + (j - 3)])
                    }
                );
            }
            if j <= 2 {
                if letters {
                    print!("|⬛|");
                } else {
                    print!("⬛");
                }
            }
        }
        println!();
    }
    println!("------------------------------------")
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

fn main() {
    let mut cube = RubikCube::new();
    let letters = true;
    let (top, bottom, left, right, front, back) = get_faces(&cube);
    printer(top, bottom, left, right, front, back, letters);
    cube.r();

    let (top, bottom, left, right, front, back) = get_faces(&cube);
    printer(top, bottom, left, right, front, back, letters);
}
