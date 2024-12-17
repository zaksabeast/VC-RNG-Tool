const DIV_CYCLE_LENGTH: usize = 0x4000;

const DIV_INCREMENTS: [u8; 16] = [
    0x12, 0x12, 0x12, 0x13, 0x12, 0x12, 0x13, 0x12, 0x12, 0x13, 0x12, 0x12, 0x13, 0x12, 0x12, 0x13,
];

#[derive(Debug, Clone)]
pub struct Div {
    index: usize,
    value: u8,
    adjusted_indexes: Vec<usize>,
}

impl Div {
    pub fn new(index: usize, value: u8, adjusted_indexes: Vec<usize>) -> Self {
        Self {
            index,
            value,
            adjusted_indexes,
        }
    }

    pub fn current_increment(&self) -> u8 {
        let is_adjusted = self.adjusted_indexes.iter().any(|&i| i == self.index);
        let expected_increment = DIV_INCREMENTS[self.index % 16];
        match (is_adjusted, expected_increment) {
            (true, 0x12) => 0x13,
            (true, 0x13) => 0x12,
            _ => expected_increment,
        }
    }

    pub fn next(&mut self) {
        let increment = self.current_increment();
        self.value = self.value.wrapping_add(increment);
        self.index = (self.index + 1) % DIV_CYCLE_LENGTH;
    }

    pub fn value(&self) -> u8 {
        self.value
    }

    pub fn set_value(&mut self, value: u8) {
        self.value = value;
    }

    pub fn decrement_index(&mut self, value: usize) {
        self.index = self.index.wrapping_sub(value) % DIV_CYCLE_LENGTH;
    }

    pub fn increment_index(&mut self, value: usize) {
        self.index = (self.index.wrapping_add(value)) % DIV_CYCLE_LENGTH;
    }

    pub fn increment_value(&mut self, value: u8) {
        self.value = self.value.wrapping_add(value);
    }
}
