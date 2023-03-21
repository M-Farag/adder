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
    fn can_hold(&self, other:Rectangle) -> bool {
        if self.width > other.width && self.height > other.height {
            true
        } else {
            false
        }
    }
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
}
