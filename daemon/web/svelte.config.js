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
            strict: true,
        }),
        output: {
            // Force everything into one HTML file. SvelteKit will still generate
            // a lot of JS files but they are deadweight and will not be included
            // in the rust binary.
            bundleStrategy: 'inline',
        },
        version: {
            // Use a deterministic version string for reproducible builds.
            // Without this option, SvelteKit will use a timestamp.
            name: process.env.GITHUB_SHA || 'dev',
        },
    },
};
