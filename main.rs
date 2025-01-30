

fn main(){
    println!("hello lade!");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    println!("Hello {}!", input.trim());
}
