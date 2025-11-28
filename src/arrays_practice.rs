pub fn arrays_practice() {
    let nums: [i32; 5] = [1, 2, 3, 4, 5];
    println!("num index 2 {:?}", nums[2]);

    for i in 0..nums.len() {
        println!("nums - index {:?} with value {:?}", i, nums[i]);
    }

    for element in nums.iter() {
        println!("nums - {:?}", element);
    }

    let matrix: [[i32; 2]; 3] = [
      [1, 2],
      [3, 4],
      [5, 6],
    ];

    println!("matrix value at [1,1] ...: {:?}", matrix[1][1]);

    for  (r, row) in matrix.iter().enumerate() {
        for (c, col) in row.iter().enumerate() {
            println!("matrix [{:?}, {:?}] value ...: {:?}", r, c, matrix[r][c]);
        }
    }
}