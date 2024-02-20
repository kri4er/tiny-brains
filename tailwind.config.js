/** @type {import('tailwindcss').Config} */
module.exports = {
    darkMode: ['class', '[data-mode="dark"]'],
    content: ["./templates/**/*.{html,js}", './templates/*{html,js}'],
    theme: {
        extend: {},
    },
    plugins: [],
}
