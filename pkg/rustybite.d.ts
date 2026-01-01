/* tslint:disable */
/* eslint-disable */

export class Cart {
  free(): void;
  [Symbol.dispose](): void;
  item_count(): number;
  remove_item(index: number): string;
  total_items(): number;
  update_quantity(index: number, new_qty: number): void;
  constructor();
  clear(): void;
  add_item(menu_item_id: number, item_name: string, quantity: number, unit_price: number): void;
  is_empty(): boolean;
  subtotal(): number;
  get_items(): any;
}

export class Menu {
  free(): void;
  [Symbol.dispose](): void;
  get_all_items(): any;
  get_categories(): string[];
  is_item_available(id: number): boolean;
  get_item_by_id_json(id: number): any;
  get_item_by_category(category: string): any;
  constructor();
}

export class Order {
  free(): void;
  [Symbol.dispose](): void;
  get_summary(): string;
  is_cancelled(): boolean;
  is_completed(): boolean;
  advance_status(): string;
  get_items_json(): string;
  get_status_string(): string;
  static estimate_wait_time(item_count: number, is_rush: boolean): number;
  constructor(id: number, items_js: any, total: number, current_time: number, is_rush_hour: boolean);
  cancel(): void;
  id: number;
  total: number;
  created_at: number;
  ready_at: number;
  estimated_wait_minutes: number;
}

export enum OrderStatus {
  Pending = 0,
  Confirmed = 1,
  Cooking = 2,
  Ready = 3,
  Completed = 4,
  Cancelled = 5,
}

export class Pricing {
  free(): void;
  [Symbol.dispose](): void;
  is_rush_hour(hour: number): boolean;
  get_multiplier(hour: number): number;
  calculate_total(cart: Cart, hour: number): number;
  apply_tax_and_fees(subtotal: number): number;
  calculate_subtotal(cart: Cart, hour: number): number;
  calculate_item_price(base_price: number, hour: number, qty: number): number;
  get_rush_hour_message(hour: number): string;
  constructor();
}

export function init(): void;

export type InitInput = RequestInfo | URL | Response | BufferSource | WebAssembly.Module;

export interface InitOutput {
  readonly memory: WebAssembly.Memory;
  readonly __wbg_cart_free: (a: number, b: number) => void;
  readonly cart_add_item: (a: number, b: number, c: number, d: number, e: number, f: number) => [number, number];
  readonly cart_clear: (a: number) => void;
  readonly cart_get_items: (a: number) => any;
  readonly cart_is_empty: (a: number) => number;
  readonly cart_item_count: (a: number) => number;
  readonly cart_new: () => number;
  readonly cart_remove_item: (a: number, b: number) => [number, number, number, number];
  readonly cart_subtotal: (a: number) => number;
  readonly cart_total_items: (a: number) => number;
  readonly cart_update_quantity: (a: number, b: number, c: number) => [number, number];
  readonly __wbg_get_order_created_at: (a: number) => number;
  readonly __wbg_get_order_estimated_wait_minutes: (a: number) => number;
  readonly __wbg_get_order_id: (a: number) => number;
  readonly __wbg_get_order_ready_at: (a: number) => number;
  readonly __wbg_get_order_total: (a: number) => number;
  readonly __wbg_order_free: (a: number, b: number) => void;
  readonly __wbg_set_order_created_at: (a: number, b: number) => void;
  readonly __wbg_set_order_estimated_wait_minutes: (a: number, b: number) => void;
  readonly __wbg_set_order_id: (a: number, b: number) => void;
  readonly __wbg_set_order_ready_at: (a: number, b: number) => void;
  readonly __wbg_set_order_total: (a: number, b: number) => void;
  readonly order_advance_status: (a: number) => [number, number];
  readonly order_cancel: (a: number) => [number, number];
  readonly order_estimate_wait_time: (a: number, b: number) => number;
  readonly order_get_items_json: (a: number) => [number, number];
  readonly order_get_status_string: (a: number) => [number, number];
  readonly order_get_summary: (a: number) => [number, number];
  readonly order_is_cancelled: (a: number) => number;
  readonly order_is_completed: (a: number) => number;
  readonly order_new: (a: number, b: any, c: number, d: number, e: number) => number;
  readonly init: () => void;
  readonly __wbg_pricing_free: (a: number, b: number) => void;
  readonly pricing_apply_tax_and_fees: (a: number, b: number) => number;
  readonly pricing_calculate_item_price: (a: number, b: number, c: number, d: number) => number;
  readonly pricing_calculate_subtotal: (a: number, b: number, c: number) => number;
  readonly pricing_calculate_total: (a: number, b: number, c: number) => [number, number, number];
  readonly pricing_get_multiplier: (a: number, b: number) => number;
  readonly pricing_get_rush_hour_message: (a: number, b: number) => [number, number];
  readonly pricing_is_rush_hour: (a: number, b: number) => number;
  readonly pricing_new: () => number;
  readonly __wbg_menu_free: (a: number, b: number) => void;
  readonly menu_get_all_items: (a: number) => any;
  readonly menu_get_categories: (a: number) => [number, number];
  readonly menu_get_item_by_category: (a: number, b: number, c: number) => any;
  readonly menu_get_item_by_id_json: (a: number, b: number) => any;
  readonly menu_is_item_available: (a: number, b: number) => number;
  readonly menu_new: () => number;
  readonly __wbindgen_malloc: (a: number, b: number) => number;
  readonly __wbindgen_realloc: (a: number, b: number, c: number, d: number) => number;
  readonly __wbindgen_exn_store: (a: number) => void;
  readonly __externref_table_alloc: () => number;
  readonly __wbindgen_externrefs: WebAssembly.Table;
  readonly __externref_table_dealloc: (a: number) => void;
  readonly __wbindgen_free: (a: number, b: number, c: number) => void;
  readonly __externref_drop_slice: (a: number, b: number) => void;
  readonly __wbindgen_start: () => void;
}

export type SyncInitInput = BufferSource | WebAssembly.Module;

/**
* Instantiates the given `module`, which can either be bytes or
* a precompiled `WebAssembly.Module`.
*
* @param {{ module: SyncInitInput }} module - Passing `SyncInitInput` directly is deprecated.
*
* @returns {InitOutput}
*/
export function initSync(module: { module: SyncInitInput } | SyncInitInput): InitOutput;

/**
* If `module_or_path` is {RequestInfo} or {URL}, makes a request and
* for everything else, calls `WebAssembly.instantiate` directly.
*
* @param {{ module_or_path: InitInput | Promise<InitInput> }} module_or_path - Passing `InitInput` directly is deprecated.
*
* @returns {Promise<InitOutput>}
*/
export default function __wbg_init (module_or_path?: { module_or_path: InitInput | Promise<InitInput> } | InitInput | Promise<InitInput>): Promise<InitOutput>;
