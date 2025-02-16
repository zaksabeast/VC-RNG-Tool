/* tslint:disable */
/* eslint-disable */
export function generate_starters(opts: Options): Starter[];
export enum Filter {
  Any = 0,
  Shiny = 1,
  MaxDv = 2,
}
export class Options {
  private constructor();
  free(): void;
  static new(adiv: number, sdiv: number, adiv_index: number, sdiv_index: number, state: number, start_advance: number, end_advance: number, filter: Filter): Options;
}
export class Starter {
  private constructor();
  free(): void;
  state: number;
  advance: number;
  shiny: boolean;
  max_dv: boolean;
}
