mod rubik_tools {
    //Remove warn dead code
    #![allow(dead_code)]
    fn get_color(val: u8) -> String {
        if val < 10 {
            return format!("⬜");
        }
        if val < 19 {
            return format!("🟨");
        }
        if val < 28 {
            return format!("🟩");
        }
        if val < 37 {
            return format!("🟦");
        }
        if val < 46 {
            return format!("🟧");
        }
        return format!("🟥");
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
}
