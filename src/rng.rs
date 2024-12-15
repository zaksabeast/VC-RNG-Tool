use super::div::Div;

pub struct Rng {
    r_add: u8,
    r_sub: u8,
    add_div: Div,
    sub_div: Div,
}

impl Rng {
    pub fn new(state: u16, add_div: Div, sub_div: Div) -> Self {
        let r_add = (state >> 8) as u8;
        let r_sub = state as u8;
        Self {
            r_add,
            r_sub,
            add_div,
            sub_div,
        }
    }

    pub fn next(&mut self) -> [u8; 2] {
        self.add_div.next();
        self.sub_div.next();

        let (r_add, add_overload) = self.r_add.overflowing_add(self.add_div.value());
        self.r_add = r_add;
        let sub_div = self.sub_div.value().wrapping_add(add_overload as u8);
        self.r_sub = self.r_sub.wrapping_sub(sub_div);

        [self.r_add, self.r_sub]
    }

    pub fn next_u16(&mut self) -> u16 {
        let [r_add, r_sub] = self.next();
        (r_add as u16) << 8 | r_sub as u16
    }

    pub fn div(&self) -> u8 {
        self.add_div.value()
    }
}
