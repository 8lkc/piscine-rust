#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {pub r: u8, pub g: u8, pub b: u8, pub a: u8,}

impl Color {
    pub fn swap(mut self, first: u8, second: u8) -> Color {
        if self.r == first {
            if self.g == second {std::mem::swap(&mut self.r, &mut self.g);}
            else if self.b == second {std::mem::swap(&mut self.r, &mut self.b);}
            else if self.a == second {std::mem::swap(&mut self.r, &mut self.a);}
        } else if self.g == first {
            if self.r == second {std::mem::swap(&mut self.g, &mut self.r);}
            else if self.b == second {std::mem::swap(&mut self.g, &mut self.b);}
            else if self.a == second {std::mem::swap(&mut self.g, &mut self.a);}
        } else if self.b == first {
            if self.r == second {std::mem::swap(&mut self.b, &mut self.r);}
            else if self.g == second {std::mem::swap(&mut self.b, &mut self.g);}
            else if self.a == second {std::mem::swap(&mut self.b, &mut self.a);}
        } else if self.a == first {
            if self.r == second {std::mem::swap(&mut self.a, &mut self.r);}
            else if self.g == second {std::mem::swap(&mut self.a, &mut self.g);}
            else if self.b == second {std::mem::swap(&mut self.a, &mut self.b);}
        }
        self
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_one() {
        let c = Color {r: 255, g: 200, b: 10, a: 30,};
        assert_eq!(c.swap(c.r, c.b), Color {r: 10, g: 200, b: 255,a: 30});
        assert_eq!(c.swap(c.r, c.g), Color {r: 200, g: 255, b: 10, a: 30});
        assert_eq!(c.swap(c.r, c.a), Color {r: 30, g: 200, b: 10, a: 255});
        assert_eq!(c.swap(c.g, c.r), Color {r: 200, g: 255, b: 10, a: 30});
        assert_eq!(c.swap(c.g, c.b), Color {r: 255, g: 10, b: 200, a: 30});
        assert_eq!(c.swap(c.g, c.a), Color {r: 255, g: 30, b: 10, a: 200});
        assert_eq!(c.swap(c.b, c.r), Color {r: 10, g: 200, b: 255, a: 30});
        assert_eq!(c.swap(c.b, c.g), Color {r: 255, g: 10, b: 200, a: 30});
        assert_eq!(c.swap(c.b, c.a), Color {r: 255, g: 200, b: 30, a: 10});
        assert_eq!(c.swap(c.a, c.r), Color {r: 30, g: 200, b: 10, a: 255});
        assert_eq!(c.swap(c.a, c.b), Color {r: 255, g: 200, b: 30, a: 10});
        assert_eq!(c.swap(c.a, c.g), Color {r: 255, g: 30, b: 10, a: 200});
    }
}
