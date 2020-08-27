pub struct Rectanlge {
    length: u32, 
    width: u32,
}

impl Rectanlge {
    pub fn can_hold(&self, other: &Rectanlge) -> bool {
        //self.length > other.length && self.width > other.width
        self.length < other.length && self.width < other.width
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectanlge {
            length: 8,
            width: 7,
        };
        let smaller = Rectanlge {
            length: 5,
            width: 1,
        };

        assert!(larger.can_hold(&smaller));
    }

    #[test]
    fn smaller_cannot_hold_larger() {
        let larger = Rectanlge {
            length: 8,
            width: 7,
        };
        let smaller = Rectanlge {
            length: 5,
            width: 1,
        };

        assert!(!smaller.can_hold(&larger));
    }
}
