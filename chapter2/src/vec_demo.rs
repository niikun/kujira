pub fn run() {
    let vec1: Vec<i32> = vec![1, 2, 3];
    let mut vec2: Vec<i32> = Vec::new();
    let mut vec3: Vec<&str> = Vec::new();
    vec2.push(4);
    vec2.push(5);
    vec2.remove(0);
    vec2.push(6);
    vec3.push("ichikun");
    vec3.push("niikun");

    println!("{:?}", vec1);
    println!("{:?}", vec2);
    println!("{:?}", vec3);
}
