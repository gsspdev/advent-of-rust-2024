// Write a function that returns the reference to the longer string
// without any new allocations
pub fn longer_wish<'a>(s1: &'a str, s2: &'a str) -> Option<&'a str> {
    // Your code here
    let s1_len = s1.trim().chars().count();
    let s2_len = s2.trim().chars().count();

    if s1_len == s2_len {
        return None;
    } else if s1_len > s2_len {
        return Some(&s1);
    } else {
        return Some(&s2);
    }
}
