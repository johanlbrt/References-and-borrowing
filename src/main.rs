fn main() {
    let s1 = String::from("Hello!");
    let len = calc_lenght(&s1);
    println!("{} = {}",s1,len);
}

fn calc_lenght(s: &String) -> usize {
    s.len()
}