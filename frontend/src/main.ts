import { createApp } from 'vue'
import './style.css'
import App from './App.vue'
import router from './router'
import { initializeAuthFlow } from '@/app/session/initialize-auth-flow'

const app = createApp(App)

app.use(router)

await initializeAuthFlow()
await router.isReady()

app.mount('#app')
