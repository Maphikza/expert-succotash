fn main() {
    let answer = is_palindrome("Sos");
    println!("is is a palindrome? {}", answer);
    assert_eq!(is_palindrome("racecar"), true);
    assert_eq!(is_palindrome("hello"), false);
    assert_eq!(is_palindrome("A man a plan a canal Panama"), true);
}

fn is_palindrome(input: &str) -> bool {
    let string = String::from(input).replace(" ", "").to_lowercase();
    let mut reverse_string = String::new();
    for i in string.chars().rev() {
        reverse_string.push(i);
    }
    if string == reverse_string {
        return true;
    } else {
        return false;
    }
}
