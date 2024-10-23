pub fn animal_habitat(animal: &str) -> (i32, &str) {
    // 此时的if是个语句了, 拥有了返回值, 需要加上分号
    let identifier: i32 = if animal == "crab" {
        1
    } else if animal == "gopher" {
        2
    } else if animal == "snake" {
        3
    } else {
        42
    };

    // 默认return返回
    if identifier == 1 {
        (identifier, "Beach")
    } else if identifier == 2 {
        (identifier, "Burrow")
    } else if identifier == 3 {
        (identifier, "Desert")
    } else {
        (identifier, "Unknown")
    }
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gopher_lives_in_burrow() {
        let result: (i32, &str) = animal_habitat("gopher");
        assert_eq!(result.0, 2);
        assert_eq!(result.1, "Burrow");
    }

    #[test]
    fn snake_lives_in_desert() {
        let result: (i32, &str) = animal_habitat("snake");
        assert_eq!(result.0, 3);
        assert_eq!(result.1, "Desert");
    }

    #[test]
    fn crab_lives_on_beach() {
        let result: (i32, &str) = animal_habitat("crab");
        assert_eq!(result.0, 1);
        assert_eq!(result.1, "Beach");
    }

    #[test]
    fn unknown_animal() {
        let result: (i32, &str) = animal_habitat("dinosaur");
        assert_eq!(result.0, 42);
        assert_eq!(result.1, "Unknown");
    }
}
