import { defineConfig } from 'vite'

export default defineConfig({
    server: {
        watch: {},
        fs: {
            allow: [
                '../salesman-web',
                '../salesman',
            ]
        }
    },
});