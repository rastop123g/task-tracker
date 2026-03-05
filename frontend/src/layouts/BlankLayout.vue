<script setup lang="ts"></script>

<template>
  <div class="min-h-screen flex items-start sm:items-center justify-center bg-muted p-4">
    <router-view v-slot="{ Component }">
      <transition mode="out-in">
        <component :is="Component" />
      </transition>
    </router-view>
  </div>
</template>
