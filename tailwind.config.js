/** @type {import('tailwindcss').Config} */
export default {
    content: [
        "./index.html",
        "./src/**/*.{js,ts,jsx,tsx}",
    ],
    theme: {
        extend: {
            colors: {
                primary: {
                    DEFAULT: '#1976D2',
                    light: '#42A5F5',
                    dark: '#0D47A1',
                },
                secondary: {
                    DEFAULT: '#FF4081',
                    light: '#FF80AB',
                    dark: '#C51162',
                },
                background: {
                    light: '#F5F5F5',
                    dark: '#121212',
                    paper: {
                        light: '#FFFFFF',
                        dark: '#1E1E1E',
                    }
                },
                success: '#4CAF50',
                error: '#F44336',
                warning: '#FFC107',
                info: '#2196F3',
            },
            fontFamily: {
                sans: ['Inter', 'system-ui', 'sans-serif'],
                mono: ['JetBrains Mono', 'monospace'],
            },
        },
    },
    darkMode: 'class',
    plugins: [],
} 