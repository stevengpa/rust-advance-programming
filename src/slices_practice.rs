pub fn slices_practice() {
    let nums = [1, 2, 3, 4, 5]; // array

    let slice = &nums[1..4];
    println!("slice {:?}", slice);
}