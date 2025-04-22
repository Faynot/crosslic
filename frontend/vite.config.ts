import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vite.dev/config/
export default defineConfig({
  plugins: [react()],
  resolve: {
    alias: {
      // Сопоставляем 'react-native' с 'react-native-web' для web
      'react-native$': 'react-native-web',
      // Можно добавить другие алиасы для путей, если нужно
    },
  },
})

