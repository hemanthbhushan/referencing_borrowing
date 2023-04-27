fn main() {
    let s1 = String::from("hello");
    let mut s2 = String::from("hello");

    let len = calculate_length(&s1);
    let len2 = change(&mut s2);

    println!("The length of '{}' is {}.", s1, len);
    println!("The length of '{}' is {}.", s2, len2);
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(some_string: &mut String) -> usize {
    some_string.push_str(",world");
    some_string.len()
}
