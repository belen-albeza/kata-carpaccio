use kata_carpaccio::run;

fn main() {
    let mut stdin = std::io::stdin().lock();
    let output = run(&mut stdin).unwrap();
    println!("{}", output);
}
