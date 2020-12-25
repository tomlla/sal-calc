mod libx;
mod lib;
mod dir1;

fn example2() {
    libx::org::create_user();
    lib::xx::gr();
    dir1::zz::judo();
}

fn example1() {
    let mut guess = String::new();
    println!("Hello, world!");
    let r = std::io::stdin().read_line(&mut guess);
    let r_err: usize = r.expect("Failed to read");
    println!("r_err: {}", r_err);
    println!("guess: {}", guess);
    let msg = guess + "c";
    println!("msg: {}", msg);
    let x = Some(1);
    if x.is_none() {
        return;
    }
    let z = x.unwrap();
    let result_z = z * (if b { 1 } else { 0 });
    println!("result_z: {}", result_z);
}

fn testLoop() {
    let range1: std::ops::Range<i32> = 1..5;

    (0..=20).collect::<Vec<u32>>();
}

