<script setup lang="ts">
import { computed } from 'vue'
import { useRoute, RouterLink } from 'vue-router'
import SettingsSidebar from '@/components/SettingsSidebar.vue'
import { SidebarProvider, SidebarInset, SidebarTrigger } from '@/components/ui/sidebar'
import { Separator } from '@/components/ui/separator'
import { Button } from '@/components/ui/button'
import {
  Breadcrumb,
  BreadcrumbItem,
  BreadcrumbLink,
  BreadcrumbList,
  BreadcrumbPage,
  BreadcrumbSeparator,
} from '@/components/ui/breadcrumb'
import { useSettingsActions } from '@/composables/useSettingsActions'

const route = useRoute()
const currentPageTitle = computed(() => route.meta.title ?? '')
const { trigger, isSaving, hasAction } = useSettingsActions()
</script>

<template>
  <SidebarProvider class="h-svh overflow-hidden">
    <SettingsSidebar />
    <SidebarInset>
      <header class="bg-background flex h-16 shrink-0 items-center gap-2 border-b px-4">
        <SidebarTrigger class="-ml-1" />
        <Separator orientation="vertical" class="mr-2 h-4" />
        <Breadcrumb>
          <BreadcrumbList>
            <BreadcrumbItem class="hidden md:block">
              <BreadcrumbLink as-child>
                <RouterLink to="/settings/profile">Настройки</RouterLink>
              </BreadcrumbLink>
            </BreadcrumbItem>
            <BreadcrumbSeparator class="hidden md:block" />
            <BreadcrumbItem>
              <BreadcrumbPage>{{ currentPageTitle }}</BreadcrumbPage>
            </BreadcrumbItem>
          </BreadcrumbList>
        </Breadcrumb>
      </header>
      <div class="flex-1 overflow-y-auto min-h-0">
        <router-view v-slot="{ Component }">
          <transition>
            <keep-alive>
              <component :is="Component" />
            </keep-alive>
          </transition>
        </router-view>
      </div>
      <footer
        v-if="hasAction"
        class="bg-background flex h-16 shrink-0 items-center justify-end border-t px-4"
      >
        <Button :disabled="isSaving" @click="trigger">
          {{ isSaving ? 'Сохранение…' : 'Сохранить' }}
        </Button>
      </footer>
    </SidebarInset>
  </SidebarProvider>
</template>
