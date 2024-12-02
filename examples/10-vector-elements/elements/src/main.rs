fn main() {
    let mut v = vec![1, 2, 3];
    v.push(4);
    //println!("{:?}", v); // Output: [1, 2, 3, 4]

    // extend adds each element of the given slice to the vector
    let more_numbers = vec![5, 6];
    v.extend(more_numbers);
    //println!("{:?}", v);

    // append adds the given vector to the vector, requires the vector to be mutable
    let mut other_numbers = vec![7, 8];
    v.append(&mut other_numbers);
    println!("{:?}", v);

    // insert items at a given index
    v.insert(0, 0);
    println!("{:?}", v); // Output: [0, 1, 2, 3, 4, 5, 6, 7, 8] 

    double_pad(&mut v, 9);
    println!("{:?}", v);

    let more_numbers = vec![10, 11];
    combine(&mut v, &more_numbers);
    println!("{:?}", v);    

    combine_general(&mut v, &more_numbers);
    println!("{:?}", v);    

    let mut mut_vec = more_numbers.clone();
    combine_clonefree(&mut v, &mut mut_vec);
    println!("{:?}", v);    

    let mut vstr = vec!["Hund","Huhn"];
    let vstr2 = vec!["Ente","Eber","Eidechse"];

    combine_general(&mut vstr, &vstr2);
    println!("{:?}", vstr);    


}

fn double_pad(vec: &mut Vec<i32>, value: i32) {
    vec.insert(0, value);
    vec.push(value);    
}

fn combine(v1: &mut Vec<i32>, v2: &Vec<i32>) {
    v1.extend(v2);
}

fn combine_general<T: Clone>(v1: &mut Vec<T>, v2: &Vec<T>) {
    v1.extend(v2.clone());
}

fn combine_clonefree<T>(v1: &mut Vec<T>, v2: &mut Vec<T>) {
    v1.append(v2);
}
