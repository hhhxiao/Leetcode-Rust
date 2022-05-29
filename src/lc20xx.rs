pub fn final_value_after_operations(operations: Vec<String>) -> i32 {
    let mut sum = 0;
    for x in operations {
        sum += match x.as_str() {
            "++X" => 1,
            "X++" => 1,
            "--X" => -1,
            "X--" => -1,
            _ => 0,
        };
    }
    sum
}
