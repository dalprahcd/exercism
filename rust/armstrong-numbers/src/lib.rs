pub fn is_armstrong_number(num: u32) -> bool {
    let len = num.to_string().len() as u32;
    num == num
        .to_string()
        .chars()
        .filter_map(|c| c.to_digit(10))
        .map(|d| d.pow(len))
        .sum()
}
