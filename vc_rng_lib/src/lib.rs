mod div;
pub mod rng;

use div::Div;
use rng::{Rng, SpecialTrait};
use wasm_bindgen::prelude::*;

#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[derive(Debug, Clone, Copy, PartialEq)]
#[wasm_bindgen]
pub enum Filter {
    Any,
    Shiny,
    MaxDv,
}

impl PartialEq<SpecialTrait> for Filter {
    fn eq(&self, other: &SpecialTrait) -> bool {
        match (self, other) {
            (Filter::Shiny, SpecialTrait::Shiny) => true,
            (Filter::MaxDv, SpecialTrait::MaxDv) => true,
            (Filter::Any, SpecialTrait::Shiny) => true,
            (Filter::Any, SpecialTrait::MaxDv) => true,
            _ => false,
        }
    }
}

#[wasm_bindgen]
pub struct Options {
    div: u16,
    index: usize,
    state: u16,
    start_advance: usize,
    end_advance: usize,
    filter: Filter,
}

#[wasm_bindgen]
impl Options {
    pub fn new(
        div: u16,
        index: usize,
        state: u16,
        start_advance: usize,
        end_advance: usize,
        filter: Filter,
    ) -> Self {
        Self {
            div,
            index,
            state,
            start_advance,
            end_advance,
            filter,
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
#[wasm_bindgen]
pub struct Starter {
    pub state: u16,
    pub advance: usize,
    pub shiny: bool,
    pub max_dv: bool,
}

#[wasm_bindgen]
pub fn generate_starters(opts: Options) -> Vec<Starter> {
    let (add_div, sub_div) = Div::new_pair(opts.index, (opts.div >> 8) as u8, opts.div as u8);
    let mut rng = Rng::new(opts.state, add_div, sub_div);
    let mut starters = Vec::new();
    for advance in opts.start_advance..=opts.end_advance {
        let special_trait = rng.possible_special_trait();
        if opts.filter == special_trait {
            starters.push(Starter {
                state: rng.state(),
                advance,
                shiny: special_trait == SpecialTrait::Shiny,
                max_dv: special_trait == SpecialTrait::MaxDv,
            });
        }
        rng.next();
    }
    starters
}
