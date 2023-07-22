fn main() {
    println!("{}", 100);
    unsafe {
        srand();
    }
    for _ in 0..100 {
        println!("{}", 9);
        for i in 1..=7 {
            print!("{} ", unsafe { rand() % i + 1 });
        }
        print!("{}", unsafe { rand() % 8 + 1 });
        println!();
    }
}

extern "C" {
  fn srand() -> u32;
  fn rand() -> u32;
}
