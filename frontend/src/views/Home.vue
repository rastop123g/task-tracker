<script setup lang="ts">
import type { LoginRequest } from '@protocol/LoginRequest'

const req: LoginRequest = {
  email: 'test@test.com',
  password: '123456',
}
</script>

<template>
  <div class="home-wrapper">
    <h1>Home</h1>
  </div>
</template>

<style scoped></style>
