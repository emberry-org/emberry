export const prerender = 'auto';
export const csr = true;
export const ssr = false;

/** @type {import('./$types').PageLoad} */
export function load({ params }: any) {
  return {
    id: params.id
  }
}