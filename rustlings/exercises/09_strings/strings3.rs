fn trim_me(input: &str) -> &str {
    // TODO: Remove whitespace from both ends of a string.
    let s = input.trim();
    s
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There are multiple ways to do this.
    let mut g = String::new();
    g = input.to_string();
    g.push_str(" world!");
    g
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons".
    let mut g = String::new();
    g = input.to_string();
    let m = g.replace("cars","balloons");
    m
}

fn main() {
    // You can optionally experiment here.
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn trim_a_string() {
        assert_eq!(trim_me("Hello!     "), "Hello!");
        assert_eq!(trim_me("  What's up!"), "What's up!");
        assert_eq!(trim_me("   Hola!  "), "Hola!");
    }

    #[test]
    fn compose_a_string() {
        assert_eq!(compose_me("Hello"), "Hello world!");
        assert_eq!(compose_me("Goodbye"), "Goodbye world!");
    }
    // #[test] 
    // fn compose_a_string(){
    //     assert!(compose_me("Hello"));
    // }

    #[test]
    fn replace_a_string() {
        assert_eq!(
            replace_me("I think cars are cool"),
            "I think balloons are cool",
        );
        assert_eq!(
            replace_me("I love to look at cars"),
            "I love to look at balloons",
        );
    }
}
