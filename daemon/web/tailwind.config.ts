import type { Config } from 'tailwindcss';
import { breakpoints } from './src/theme';

export default {
    content: ['./src/**/*.{html,js,svelte,ts}'],

    theme: {
        extend: {
            colors: {
                'rayhunter-blue': '#4e4eb1',
                'rayhunter-dark-blue': '#3f3da0',
                'rayhunter-green': '#94ea18',
            },
            screens: breakpoints,
        },
    },

    plugins: [],
} as Config;
