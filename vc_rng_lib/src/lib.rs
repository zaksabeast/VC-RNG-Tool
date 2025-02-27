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
pub struct PokeOptions {
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
impl PokeOptions {
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
pub struct Spread {
    pub state: u16,
    pub advance: usize,
    pub shiny: bool,
    pub max_dv: bool,
}

#[wasm_bindgen]
pub fn generate_starters(opts: PokeOptions) -> Vec<Spread> {
    let add_div = Div::new(opts.adiv_index, opts.adiv);
    let sub_div = Div::new(opts.sdiv_index, opts.sdiv);
    let mut rng = Rng::new(opts.state, add_div, sub_div);
    let mut spreads = Vec::new();
    for advance in opts.start_advance..=opts.end_advance {
        let special_trait = rng.has_potential_special_starter();
        if opts.filter == special_trait {
            spreads.push(Spread {
                state: rng.state(),
                advance,
                shiny: special_trait == SpecialTrait::Shiny,
                max_dv: special_trait == SpecialTrait::MaxDv,
            });
        }
        rng.next();
    }
    spreads
}

#[wasm_bindgen]
pub fn generate_celebi(opts: PokeOptions) -> Vec<Spread> {
    let add_div = Div::new(opts.adiv_index, opts.adiv);
    let sub_div = Div::new(opts.sdiv_index, opts.sdiv);
    let mut rng = Rng::new(opts.state, add_div, sub_div);
    let mut spreads = Vec::new();
    for advance in opts.start_advance..=opts.end_advance {
        let special_trait = rng.has_potential_special_celebi();
        if opts.filter == special_trait {
            spreads.push(Spread {
                state: rng.state(),
                advance,
                shiny: special_trait == SpecialTrait::Shiny,
                max_dv: special_trait == SpecialTrait::MaxDv,
            });
        }
        rng.next();
    }
    spreads
}

#[wasm_bindgen]
pub struct RandOptions {
    adiv: u8,
    sdiv: u8,
    adiv_index: usize,
    sdiv_index: usize,
    state: u16,
    start_advance: usize,
    end_advance: usize,
}

#[wasm_bindgen]
impl RandOptions {
    pub fn new(
        adiv: u8,
        sdiv: u8,
        adiv_index: usize,
        sdiv_index: usize,
        state: u16,
        start_advance: usize,
        end_advance: usize,
    ) -> Self {
        Self {
            adiv,
            sdiv,
            adiv_index,
            sdiv_index,
            state,
            start_advance,
            end_advance,
        }
    }
}

#[wasm_bindgen]
pub struct RngState {
    pub rand: u16,
    pub advance: usize,
    pub add_div: u8,
    pub sub_div: u8,
}

#[wasm_bindgen]
pub fn generate_rng_states(opts: RandOptions) -> Vec<RngState> {
    let adiv = Div::new(opts.adiv_index, opts.adiv);
    let sdiv = Div::new(opts.sdiv_index, opts.sdiv);
    let mut rng = Rng::new(opts.state, adiv, sdiv);
    ((opts.start_advance + 1)..opts.end_advance)
        .map(|advance| {
            let rand = rng.next_u16();
            RngState {
                rand,
                advance,
                add_div: rng.adiv(),
                sub_div: rng.sdiv(),
            }
        })
        .collect()
}
