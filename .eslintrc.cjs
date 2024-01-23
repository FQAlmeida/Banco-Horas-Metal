/* eslint-env node */
module.exports = {
    extends: ['eslint:recommended', 'plugin:@typescript-eslint/recommended-type-checked',
        'plugin:@typescript-eslint/stylistic-type-checked', 'plugin:@typescript-eslint/strict-type-checked'],
    parser: '@typescript-eslint/parser',
    parserOptions: {
        project: true,
        tsconfigRootDir: __dirname,
    },
    plugins: ['@typescript-eslint', '@stylistic/ts', '@stylistic/js'],
    root: true,
    ignorePatterns: [
        "**/dist",
        "**/src-tauri",
        "vite.config.ts",
        "svelte.config.js",
        ".eslintrc.cjs",
        "tailwind.config.cjs",
        "postcss.config.cjs",
    ],
    rules: {
        '@stylistic/js/max-len': ['error', 110],
    }
};