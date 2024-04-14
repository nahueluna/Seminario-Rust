pub fn ej10() {
    let a1 = [1, 2, 3, 4, 5];
    let a2 = [5, 4, 3, 2, 1];
    let mut a3 = a1;

    for index in 0..a3.len() {
        a3[index] = a1[index] + a2[index];
    }

    println!("Array 1: {:?}", a1);
    println!("Array 2: {:?}", a2);
    println!("Array resultante: {:?}", a3);
}
