use kata_carpaccio::run;

fn main() {
    let stdin = std::io::stdin().lock();
    let output = run(stdin).unwrap();
    println!("{}", output);
}
