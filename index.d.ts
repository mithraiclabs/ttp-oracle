/* eslint-disable @typescript-eslint/no-empty-interface */
declare module 'buffer-layout' {
  interface Layout<T> {
    decode: (buf: Buffer, offset?: number) => T;
    encode: (src: any, buf?: Buffer, offset?: number) => Buffer;
    getSpan: (buf?: Buffer, offset?: number) => number;
  }

  interface Sequence extends Layout {}

  interface Union<E> extends Layout {
    addVariant: (variant: number, layout: Layout, property: string) => void;
    configGetSourceVariant: (func: <T>() => T) => void;
    getSourceVariant: (src: any) => E;
  }

  interface UTF8 extends Layout {}

  interface VariantLayout extends Layout {}

  export function seq(
    element: Layout,
    count: number,
    property?: string,
  ): Sequence;
  export function struct(arr: Layout[]);
  export function u8(property?: string): Layout;
  export function u16(property?: string): Layout;
  export function u32(property?: string): Layout;
  export function union(
    discriminator: Layout,
    defaultLayout?: Layout,
    property?: string,
  ): Union;
  export function utf8(maxSpan?: number, property?: string): UTF8;
}
