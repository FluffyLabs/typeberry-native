declare const process:
  | {
      versions?: {
        node?: string;
      };
      platform?: string;
      arch?: string;
    }
  | undefined;

declare module 'module' {
  export function createRequire(path: string | URL): (id: string) => any;
}
