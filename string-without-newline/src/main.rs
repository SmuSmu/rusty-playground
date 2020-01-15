fn main() {
    let test1: String = "


    Ein String mit
    
Enter


".to_string();
    let x: &[_] = &['\n', '\r', '\t'];
    let test2: String = test1.trim_matches(x).to_string();
    let test3: String = test1.replace(x, " ").trim().to_string();

    println!("Test1 : {}", test1);
    println!("Test2 : {}", test2);
    println!("Test3 : {}", test3);
}
