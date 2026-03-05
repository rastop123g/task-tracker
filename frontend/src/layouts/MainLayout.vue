<script setup lang="ts"></script>

<template>
  <div class="main-layout flex min-h-screen">
    <aside
      class="flex-none bg-sidebar border-r border-sidebar-border"
      style="width: var(--sidebar-width)"
    >
      <nav></nav>
    </aside>
    <main class="flex-1 overflow-y-auto p-6">
      <router-view v-slot="{ Component }">
        <transition>
          <keep-alive>
            <component :is="Component" />
          </keep-alive>
        </transition>
      </router-view>
    </main>
  </div>
</template>

<style scoped>
.main-layout {
  --sidebar-width: 16rem;
}
</style>
