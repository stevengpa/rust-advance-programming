pub fn vectors_practice() {
    let mut vec1: Vec<i32> = Vec::new();
    vec1.push(1);
    vec1.push(3);
    vec1.push(2);
    vec1.push(5);
    vec1.push(6);
    vec1.push(7);
    vec1.insert(0, -1);

    println!("Vec1 {:?}", vec1);

    // Sort
    vec1.sort();
    println!("Sorted Vec1 {:?}", vec1);
    vec1.sort_by(|a, b|b.cmp(a));
    println!("Sorted Vec1 {:?}", vec1);

    // Remove
    vec1.remove(4);
    println!("Remove Vec1 4 index {:?}", vec1);
    vec1.pop();
    println!("Pop Vec1 {:?}", vec1);

    // Slice
    let top_3: &[i32] = &vec1[0..3];
    println!("Top 3 {:?}", top_3);
    println!("Vec1 {:?}", vec1);

    // SubSetting (split, split_at)

    // Split
    let vec2 = vec![1, 2, 3, 4, 5];
    let vec_split: Vec<&[i32]> = vec2.split(|&x| x == 3).collect();
    println!("Vec2 {:?}", vec2);
    println!("Vec Split {:?}", vec_split);

    // Split At
    let (vec_split_left, vec_split_right): (&[i32], &[i32]) = vec2.split_at(2);
    println!("Vec2 {:?}", vec2);
    println!("Vec Left {:?} - Vec Right {:?}", vec_split_left, vec_split_right);

    // Iterate
    for num in vec_split_left.iter() {
        println!("Vec Left Num: {:?}", num);
    }

    let vec3: Vec<i32> = vec![0; 5];
    for num in vec3.iter() {
        println!("Vec3 Num: {:?}", num);
    }
}