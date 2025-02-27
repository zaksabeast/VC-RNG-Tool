/* tslint:disable */
/* eslint-disable */
export function generate_starters(opts: PokeOptions): Spread[];
export function generate_celebi(opts: PokeOptions): Spread[];
export function generate_rng_states(opts: RandOptions): RngState[];
export enum Filter {
  Any = 0,
  Shiny = 1,
  MaxDv = 2,
}
export class PokeOptions {
  private constructor();
  free(): void;
  static new(adiv: number, sdiv: number, adiv_index: number, sdiv_index: number, state: number, start_advance: number, end_advance: number, filter: Filter): PokeOptions;
}
export class RandOptions {
  private constructor();
  free(): void;
  static new(adiv: number, sdiv: number, adiv_index: number, sdiv_index: number, state: number, start_advance: number, end_advance: number): RandOptions;
}
export class RngState {
  private constructor();
  free(): void;
  rand: number;
  advance: number;
  add_div: number;
  sub_div: number;
}
export class Spread {
  private constructor();
  free(): void;
  state: number;
  advance: number;
  shiny: boolean;
  max_dv: boolean;
}
