import type { Config } from 'tailwindcss';

export default {
    content: ['./src/**/*.{html,js,svelte,ts}'],

    theme: {
        extend: {
            colors: {
                'rayhunter-blue': '#4e4eb1',
                'rayhunter-dark-blue': '#3f3da0',
                'rayhunter-green': '#94ea18'
            }
        }
    },

    plugins: []
} as Config;
