// Write a function that returns the reference to the longer string
// without any new allocations
pub fn longer_wish<'a>(s1: &'a str, s2: &'a str) -> Option<&'a str> {
    // Your code here
    let s1 = s1.trim();
    let s2 = s2.trim();

    // why make a whole variable to store a string's length when you can
    // evaluate it with an in-line match expression. it's nicer this way
    match (s1.chars().count(), s2.chars().count()) {
        (a, b) if a == b => None,
        (a, b) if a > b => Some(s1),
        _ => Some(s2),
    }
}
