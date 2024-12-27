export default {
    root: '.',
    build: {
      outDir: '../dist/static',
      emptyOutDir: true,
    },
    server: {
        proxy: {
            '/api': {
              target: 'http://localhost:5001',
              changeOrigin: true
            }
          }
      }
  }