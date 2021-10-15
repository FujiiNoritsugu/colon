// Version 1.0
use std::thread;
use std::time::Instant;
const MAX_SIZE:i32 = 1000000;

fn main() {
    let start = Instant::now();

    for p in 0..=MAX_SIZE{
        thread::spawn(move ||{ pom(&p)});
    }    

    let end = start.elapsed();
    println!("{}.{:03}秒経過しました。", end.as_secs(), end.subsec_nanos() / 1_000_000); 
}

fn pom(i:&i32){
    println!("*i:{}",*i);
}
