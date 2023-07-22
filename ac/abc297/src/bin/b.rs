#![allow(unused_imports)]
#![allow(dead_code)]
use std::collections::*;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut v = String::new();
    std::io::stdin().read_line(&mut v)?;
    let s = v.trim().as_bytes();

    let mut fb = None;

    enum Wtfr {
        R1,
        K,
        R2,
    }

    let mut wtfr = Wtfr::R1;
    let mut skip1 = false;
    let mut skip2 = false;

    for (i, c) in s.iter().enumerate() {
        if skip1 && skip2 {
            break;
        }

        match c {
            b'B' => {
                if !skip1 {
                    match fb {
                        Some(fb) => {
                            if (i - fb) % 2 == 0 {
                                print!("No");
                                return Ok(());
                            } else {
                                skip1 = true;
                            }
                        }
                        _ => fb = Some(i),
                    }
                }
            }
            b'R' => {
                if !skip2 {
                    match wtfr {
                        Wtfr::R1 => {
                            wtfr = Wtfr::K;
                        }
                        Wtfr::K => {
                            print!("No");
                            return Ok(());
                        }
                        Wtfr::R2 => {
                            skip2 = true;
                        }
                    }
                }
            }
            b'K' => {
                if !skip2 {
                    match wtfr {
                        Wtfr::K => {
                            wtfr = Wtfr::R2;
                        }
                        _ => {
                            print!("No");
                            return Ok(());
                        }
                    }
                }
            }
            _ => {}
        }
    }

    print!("Yes");
    Ok(())
}
