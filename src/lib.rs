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


fn add_two(x:i32) -> i32
{
    x + 2
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
}
