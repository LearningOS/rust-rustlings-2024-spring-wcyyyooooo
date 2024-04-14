fn main() {
    let mut v1 = vec![1, 2, 3, 4];
    let mut v2 = v1.clone();
    v2.push(5);
    for ele in v1.iter() {
        println!("{}", ele);
    }
    for ele in v2.iter() {
        println!("{}", ele);
    }
}
