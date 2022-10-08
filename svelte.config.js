import staticAdapter from "@sveltejs/adapter-static";
import preprocess from "svelte-preprocess";

/** @type {import('@sveltejs/kit').Config} */
const config = {
  // Consult https://github.com/sveltejs/svelte-preprocess
  // for more information about preprocessors
  preprocess: preprocess(),

  kit: {
    adapter: staticAdapter({
      // Fallback page has to be named `index.html` otherwise build won't work.
      // The fallback page will be loaded if svelte cannot work out a dynamic route.
      fallback: 'index.html'
    }),
    trailingSlash: 'never',
  },
};

export default config;
