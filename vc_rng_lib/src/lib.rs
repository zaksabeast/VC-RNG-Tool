pub mod div;
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
        matches!(
            (self, other),
            (Filter::Shiny, SpecialTrait::Shiny)
                | (Filter::MaxDv, SpecialTrait::MaxDv)
                | (Filter::Any, SpecialTrait::Shiny)
                | (Filter::Any, SpecialTrait::MaxDv)
        )
    }
}

#[wasm_bindgen]
pub struct Options {
    adiv: u8,
    sdiv: u8,
    adiv_index: usize,
    sdiv_index: usize,
    state: u16,
    start_advance: usize,
    end_advance: usize,
    filter: Filter,
}

#[wasm_bindgen]
impl Options {
    pub fn new(
        adiv: u8,
        sdiv: u8,
        adiv_index: usize,
        sdiv_index: usize,
        state: u16,
        start_advance: usize,
        end_advance: usize,
        filter: Filter,
    ) -> Self {
        Self {
            adiv,
            sdiv,
            adiv_index,
            sdiv_index,
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
    let add_div = Div::new(opts.adiv_index, opts.adiv);
    let sub_div = Div::new(opts.sdiv_index, opts.sdiv);
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
