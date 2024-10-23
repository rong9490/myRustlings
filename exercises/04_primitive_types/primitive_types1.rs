// Booleans (`bool`)

fn main() -> () {
    let is_morning: bool = true;
    if is_morning {
        assert_eq!(is_morning, true);
        println!("Good morning!");
    } else {
        assert_eq!(is_morning, false);
    }

    let is_evening: bool = false;
    if is_evening {
        assert_eq!(is_evening, true);
        println!("Good evening!");
    } else {
        assert_eq!(is_evening, false);
    }
}
