import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react-swc'
import fs from 'node:fs'
import path from 'node:path'

const certDir = path.resolve(__dirname, '../../.certs')
const https =
  fs.existsSync(path.join(certDir, 'local-key.pem')) && fs.existsSync(path.join(certDir, 'local.pem'))
    ? {
        key: fs.readFileSync(path.join(certDir, 'local-key.pem')),
        cert: fs.readFileSync(path.join(certDir, 'local.pem')),
      }
    : undefined

export default defineConfig({
  plugins: [react()],
  server: {
    https,
    proxy: {
      '/api': {
        target: 'https://localhost:8787',
        secure: false,
      },
    },
  },
})
