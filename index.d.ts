declare module "buffer-layout" {
  type Layout = any;

  export function seq(element: any, count: number): Layout;
  export function struct(arr: any[]);
  export function u8(property?: string): Layout;
  export function u16(property?: string): Layout;
  export function u32(property?: string): Layout;
}
