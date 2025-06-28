import adapter from '@sveltejs/adapter-static';

export default {
    kit: {
        adapter: adapter({
            // default options are shown. On some platforms
            // these options are set automatically — see below
            pages: 'build',
            assets: 'build',
            fallback: undefined,
            precompress: false,
            strict: true
        }),
        version: {
            // Use a deterministic version string for reproducible builds.
            // Without this option, SvelteKit will use a timestamp.
            name: process.env.GITHUB_SHA || 'dev'
        }
    }
};
