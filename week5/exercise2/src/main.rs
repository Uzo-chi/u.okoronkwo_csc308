fn main() {
    let nums = vec![
        1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20,
    ];
    let mut even_nums = Vec::new();

    let evens = || {
        for num in nums {
            if num % 2 == 0 {
                even_nums.push(num);
            }
        }
    };

    evens();
    println!("Even numbers: {:?}", even_nums);
}
