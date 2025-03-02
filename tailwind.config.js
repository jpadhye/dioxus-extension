/** @type {import('tailwindcss').Config} */
export default {
    mode: "all",
    content: [
        "./src/**/*.{js,rs,html,css}",
        "./pkg/**/*.html"
    ],
    theme: {
        extend: {},
    },
    plugins: [require("@tailwindcss/forms"), require("@tailwindcss/typography")],
}