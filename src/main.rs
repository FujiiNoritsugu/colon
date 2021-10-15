// Version 1.0
use std::thread;
use std::time::Instant;
const MAX_SIZE:i32 = 1000000;

fn main() {
    let start = Instant::now();

    let handle1 = thread::spawn(||{
        for p in 0..500000{
            pom(&p);
        }    
    });

    let handle2 = thread::spawn(||{
        for p in 500000..=MAX_SIZE{
            pom(&p);
        }    
    });

    handle1.join().unwrap();
    handle2.join().unwrap();

    let end = start.elapsed();
    println!("{}.{:03}秒経過しました。", end.as_secs(), end.subsec_nanos() / 1_000_000); 
}

fn pom(i:&i32){
	println!("{}", *i);
}
