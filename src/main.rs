use rand::Rng;

fn main() {
    let secret_number = rand::thread_rng().gen_range(1, 101);

    let mut v = Vec::new();
    v.push(5);
    v.push(6);
    v.push(7);
    v.push(8);

    let v = vec![100, 32, 57];
    for i in &v {
        println!("{}", i);
    }
}
