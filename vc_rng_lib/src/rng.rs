use super::div::Div;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SpecialTrait {
    None,
    Shiny,
    MaxDv,
}

pub fn possible_special_trait(pokes: &[Poke]) -> SpecialTrait {
    pokes
        .iter()
        .filter_map(|poke| match poke.special_trait() {
            SpecialTrait::None => None,
            special_trait => Some(special_trait),
        })
        .next()
        .unwrap_or(SpecialTrait::None)
}

#[derive(Debug, Clone, Copy)]
enum Offset {
    Plus(u8),
    Minus(u8),
}

impl Offset {
    fn from_i8(value: i8) -> Self {
        if value < 0 {
            Offset::Minus(value.unsigned_abs())
        } else {
            Offset::Plus(value as u8)
        }
    }
}

impl Default for Offset {
    fn default() -> Self {
        Offset::Plus(0)
    }
}

#[derive(Debug)]
pub struct Poke {
    pub hp: u8,
    pub atk: u8,
    pub def: u8,
    pub spe: u8,
    pub spc: u8,
}

impl Poke {
    fn new(atkdef: u8, spespc: u8) -> Self {
        let atk = atkdef >> 4;
        let def = atkdef & 0xf;
        let spe = spespc >> 4;
        let spc = spespc & 0xf;
        let hp = ((atk & 1) * 8) + ((def & 1) * 4) + ((spe & 1) * 2) + (spc & 1);

        Self {
            hp,
            atk,
            def,
            spe,
            spc,
        }
    }

    pub fn max_dvs(&self) -> bool {
        self.hp == 15 && self.atk == 15 && self.def == 15 && self.spe == 15 && self.spc == 15
    }

    pub fn is_shiny(&self) -> bool {
        self.spe == 10
            && self.spc == 10
            && self.def == 10
            && [0x2, 0x3, 0x6, 0x7, 0xA, 0xB, 0xE, 0xF].contains(&self.atk)
    }

    pub fn special_trait(&self) -> SpecialTrait {
        if self.is_shiny() {
            return SpecialTrait::Shiny;
        }

        if self.max_dvs() {
            return SpecialTrait::MaxDv;
        }

        SpecialTrait::None
    }
}

fn advance_rng(r_add: u8, r_sub: u8, a_div: u8, s_div: u8) -> [u8; 2] {
    let (r_add, add_overload) = r_add.overflowing_add(a_div);
    let r_sub = r_sub.wrapping_sub(s_div.wrapping_add(add_overload as u8));
    [r_add, r_sub]
}

fn apply_offset(value: u8, offset: Offset) -> u8 {
    match offset {
        Offset::Plus(offset) => value.wrapping_add(offset),
        Offset::Minus(offset) => value.wrapping_sub(offset),
    }
}

#[derive(Debug, Clone)]
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

    pub fn state(&self) -> u16 {
        ((self.r_add as u16) << 8) | self.r_sub as u16
    }

    fn next_with_div_offset(&mut self, div_offset: Offset) -> [u8; 2] {
        self.add_div.next();
        self.sub_div.next();

        [self.r_add, self.r_sub] = advance_rng(
            self.r_add,
            self.r_sub,
            apply_offset(self.add_div.value(), div_offset),
            apply_offset(self.sub_div.value(), div_offset),
        );

        [self.r_add, self.r_sub]
    }

    fn next_with_div_inc(&mut self, div_offset: Offset) -> [u8; 2] {
        self.add_div.next();
        self.sub_div.next();

        self.add_div
            .set_value(apply_offset(self.add_div.value(), div_offset));
        self.sub_div
            .set_value(apply_offset(self.sub_div.value(), div_offset));

        [self.r_add, self.r_sub] = advance_rng(
            self.r_add,
            self.r_sub,
            self.add_div.value(),
            self.sub_div.value(),
        );

        [self.r_add, self.r_sub]
    }

    pub fn next(&mut self) -> [u8; 2] {
        self.next_with_div_offset(Offset::default())
    }

    pub fn next_u16(&mut self) -> u16 {
        let [r_add, r_sub] = self.next();
        ((r_add as u16) << 8) | r_sub as u16
    }

    pub fn adiv(&self) -> u8 {
        self.add_div.value()
    }

    pub fn sdiv(&self) -> u8 {
        self.sub_div.value()
    }

    fn generate_starter_rands(
        &self,
        a_div_1_offset: Offset,
        s_div_1_offset: Offset,
        a_div_2_offset: Offset,
        s_div_2_offset: Offset,
    ) -> [[u8; 2]; 2] {
        let mut poke_rng = self.clone();
        // Advance once for the A press
        poke_rng.next();
        let r_add_0 = poke_rng.r_add;
        let r_sub_0 = poke_rng.r_sub;
        // Get the normal rand[1] values
        poke_rng.next();
        let normal_a_div_1 = poke_rng.add_div.value();
        let normal_s_div_1 = poke_rng.sub_div.value();

        let poke_a_div_1 = normal_a_div_1.wrapping_mul(2).wrapping_add(10);
        let poke_a_div_1 = apply_offset(poke_a_div_1, a_div_1_offset);

        let poke_s_div_1 = normal_s_div_1.wrapping_mul(2).wrapping_add(10);
        let poke_s_div_1 = apply_offset(poke_s_div_1, s_div_1_offset);

        let poke_rand_1 = advance_rng(r_add_0, r_sub_0, poke_a_div_1, poke_s_div_1);

        let poke_s_div_2 = poke_s_div_1.wrapping_sub(normal_s_div_1);
        // TODO: Fix this
        let poke_a_div_2 = poke_s_div_2;

        let poke_s_div_2 = apply_offset(poke_s_div_2, s_div_2_offset);
        let poke_a_div_2 = apply_offset(poke_a_div_2, a_div_2_offset);

        let poke_rand_2 = advance_rng(poke_rand_1[0], poke_rand_1[1], poke_a_div_2, poke_s_div_2);

        [poke_rand_1, poke_rand_2]
    }

    fn potential_starters(&self) -> Vec<Poke> {
        let mut result = Vec::new();

        for a_div_1_offset in -1..=1 {
            for s_div_1_offset in -1..=1 {
                for a_div_2_offset in -1..=1 {
                    for s_div_2_offset in -1..=1 {
                        let [[_, atkdef], [_, spespc]] = self.generate_starter_rands(
                            Offset::from_i8(a_div_1_offset),
                            Offset::from_i8(s_div_1_offset),
                            Offset::from_i8(a_div_2_offset),
                            Offset::from_i8(s_div_2_offset),
                        );

                        let poke = Poke::new(atkdef, spespc);
                        result.push(poke);
                    }
                }
            }
        }

        result
    }

    pub fn has_potential_special_starter(&self) -> SpecialTrait {
        let pokes = self.potential_starters();
        possible_special_trait(&pokes)
    }

    fn generate_celebi_rands(&self, extra_consumed_rands: u8, div_off: u8) -> [[u8; 2]; 2] {
        let mut poke_rng = self.clone();

        for _ in 0..595 {
            poke_rng.next();
        }

        let adiv_index = poke_rng.add_div.index();
        let sdiv_index = poke_rng.sub_div.index();

        poke_rng.next_with_div_offset(Offset::Minus(0xc));

        for _ in 0..extra_consumed_rands {
            poke_rng.next();
        }

        poke_rng.next_with_div_inc(Offset::Plus(div_off));
        poke_rng.add_div.set_index(adiv_index);
        poke_rng.add_div.decrement_index(2);
        poke_rng.sub_div.set_index(sdiv_index);
        poke_rng.sub_div.decrement_index(2);

        for _ in 0..12 {
            poke_rng.next();
        }

        poke_rng.next_with_div_inc(Offset::Plus(0x6f));

        poke_rng.add_div.set_index(0);
        poke_rng.sub_div.set_index(3);

        let poke_rand_1 = poke_rng.next_with_div_inc(Offset::Plus(0xc));
        let poke_rand_2 = poke_rng.next_with_div_inc(Offset::Plus(0xe4));

        [poke_rand_1, poke_rand_2]
    }

    pub fn potential_celebis(&self) -> Vec<Poke> {
        let mut result = vec![];

        for extra_consumed_rands in [2, 3] {
            for div_off in [0xba, 0xbb] {
                let [[_, atkdef], [_, spespc]] =
                    self.generate_celebi_rands(extra_consumed_rands, div_off);
                let poke = Poke::new(atkdef, spespc);
                result.push(poke);
            }
        }

        result
    }

    pub fn has_potential_special_celebi(&self) -> SpecialTrait {
        let pokes = self.potential_celebis();
        possible_special_trait(&pokes)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn advances_the_rng() {
        let state = 0x9fe3;
        let adiv = Div::new(468, 0x78);
        let sdiv = Div::new(16139, 0x78);
        let mut rng = Rng::new(state, adiv, sdiv);
        let expected: Vec<u16> = vec![
            0x2958, 0xc5bb, 0x740b, 0x3549, 0x0874, 0xee8e, 0xe695, 0xf08b, 0x0d6d, 0x3c3e, 0x7dfd,
            0xd1a9, 0x3742, 0xafca, 0x393e, 0xd6a1, 0x85f1, 0x462e, 0x1a59, 0x0072, 0xf879, 0x036d,
            0x2050, 0x4f21, 0x91df, 0xe58b, 0x4b24, 0xc4ab, 0x4f1f, 0xec82, 0x9bd1, 0x5d0e, 0x3139,
            0x1751, 0x1057, 0x1b4c, 0x382e, 0x68fe, 0xaabc, 0xfe68, 0x6500, 0xde87, 0x69fb, 0x075c,
            0xb7ac, 0x79e9, 0x4d13, 0x342b, 0x2d31, 0x3825, 0x5607, 0x86d7, 0xc894, 0x1d3e, 0x84d7,
            0xfd5e, 0x89d1, 0x2732, 0xd782, 0x9abe, 0x6fe8, 0x5600, 0x4f05, 0x5bf9, 0x79db, 0xa9aa,
            0xec67, 0x4111, 0xa8a9, 0x222e, 0xaea2, 0x4c03, 0xfd52, 0xc08e, 0x95b8, 0x7dcf, 0x77d4,
            0x83c8, 0xa1a9, 0xd278, 0x1534, 0x6ade, 0xd276, 0x4cfb, 0xd86e, 0x77ce, 0x281c, 0xeb59,
            0xc182, 0xa999, 0xa39e, 0xb091, 0xcf72, 0x0040, 0x43fc, 0x99a6, 0x013d, 0x7bc2, 0x0834,
            0xa795,
        ];

        for (index, item) in expected.iter().enumerate() {
            assert_eq!(rng.next_u16(), *item, "Failed at {}", index);
        }
    }
}
