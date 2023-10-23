// strings3.rs
//
// Execute `rustlings hint strings3` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

fn trim_me(input: &str) -> String {
    // TODO: Remove whitespace from both ends of a string!
    // let s:string;
    // for c in str.chars{
    //     if s==""&&c==' '{ continue;}
    //     s.push(c);
    // }
    // while (s.get(s.len()-1)==' ') {s.pop();}
    // s;
    input.trim().to_string()
    // str.to_string().trim_matches(char::from(0));
}

fn compose_me(input: &str) -> String {
    // TODO: Add " world!" to the string! There's multiple ways to do this!
    input.to_string()+" world!"
    // format!("{}{}",input.to_string()," world!")
}

fn replace_me(input: &str) -> String {
    // TODO: Replace "cars" in the string with "balloons"!
    input.replace("cars","balloons")
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

    #[test]
    fn replace_a_string() {
        assert_eq!(replace_me("I think cars are cool"), "I think balloons are cool");
        assert_eq!(replace_me("I love to look at cars"), "I love to look at balloons");
    }
}
