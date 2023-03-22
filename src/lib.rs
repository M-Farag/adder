pub fn add(left: usize, right: usize) -> usize {
    left + right
}

struct Rectangle {
    width:usize,
    height:usize
}

impl Rectangle {
    fn new(width:usize, height:usize) -> Self
    {
        Self { width, height }
    }
    fn can_hold(&self, other:&Rectangle) -> bool {
        if self.width > other.width && self.height > other.height {
            true
        } else {
            false
        }
    }
}

struct Guess {
    value:i32
}

impl Guess {
    fn new(value:i32) -> Self
    {
        if value > 100 || value < 0 {
            panic!("Only numbers between {} and {} are allowed",0,100);
        }
        Self { value }
    }
}


fn add_two(x:i32) -> i32
{
    x + 2
}

fn greeting(name:&str) -> String
{
    format!("Hello {}",name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    #[ignore = "time consuming"]
    fn fail_test() {
        panic!("Fail the test");
    }

    #[test]
    fn assert_large_can_hold_small() {
        let large = Rectangle::new(10, 10);
        let small = Rectangle::new(4,7);

        assert!(large.can_hold(&small));
    }

    #[test]
    fn assert_smaller_cannot_hold_larger() {
        let large = Rectangle::new(10, 10);
        let small = Rectangle::new(4,7);

        assert!( ! small.can_hold(&large));
    }

    #[test]
    fn assert_add_two_return_right_result() {
        assert_eq!(4, add_two(2));
    }

    #[test]
    fn test_greeting_contain_name() {
        let greeting_output = greeting("john");
        assert!(greeting_output.contains("john"),"We got {}",greeting_output);
    }

    #[test]
    #[should_panic(expected="Only numbers between")]
    fn test_guess_struct_panic_if_value_great_than_100()
    {
        let x = Guess::new(200);
    }
}
