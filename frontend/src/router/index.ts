import 'vue-router'
import { createRouter, createWebHistory } from 'vue-router'
import type { RouteRecordRaw } from 'vue-router'
import MainLayout from '@/layouts/MainLayout.vue'
import BlankLayout from '@/layouts/BlankLayout.vue'
import SettingsLayout from '@/layouts/SettingsLayout.vue'
import { useAuth } from '@/composables/useAuth'

declare module 'vue-router' {
  interface RouteMeta {
    title?: string
    requiresAuth?: boolean
    guestOnly?: boolean
    transition?: string
    keepAlive?: boolean
  }
}

const routes: RouteRecordRaw[] = [
  {
    path: '/',
    component: MainLayout,
    children: [
      {
        name: 'home',
        path: '',
        component: () => import('@/views/Home.vue'),
        meta: { title: 'Home', requiresAuth: true },
      },
    ],
  },
  {
    path: '/settings',
    component: SettingsLayout,
    children: [
      {
        name: 'settings-profile',
        path: 'profile',
        component: () => import('@/views/ProfileView.vue'),
        meta: { title: 'Profile', requiresAuth: true },
      },
    ],
  },
  {
    path: '/auth',
    component: BlankLayout,
    children: [
      {
        path: '',
        redirect: { name: 'login' },
      },
      {
        name: 'login',
        path: 'login',
        component: () => import('@/views/Login.vue'),
        meta: { title: 'Login', guestOnly: true },
      },
      {
        name: 'register',
        path: 'register',
        component: () => import('@/views/Register.vue'),
        meta: { title: 'Register', guestOnly: true },
      },
    ],
  },
  {
    path: '/welcome',
    component: BlankLayout,
    children: [
      {
        name: 'welcome',
        path: '',
        component: () => import('@/views/Welcome.vue'),
        meta: { title: 'Welcome', requiresAuth: true },
      },
    ],
  },
]

const router = createRouter({
  history: createWebHistory(import.meta.env.BASE_URL),
  routes,
})

router.beforeEach((to) => {
  const { isAuthenticated } = useAuth()
  if (to.meta.requiresAuth && !isAuthenticated.value) {
    return { name: 'login' }
  }
  if (to.meta.guestOnly && isAuthenticated.value) {
    return { name: 'home' }
  }
})

export default router
