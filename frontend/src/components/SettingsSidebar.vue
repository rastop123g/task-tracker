<script setup lang="ts">
import type { SidebarProps } from '@/components/ui/sidebar'
import WorkspaceSwitcher from '@/components/WorkspaceSwitcher.vue'
import UserMenuDropdown from '@/components/UserMenuDropdown.vue'
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarHeader,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem,
} from '@/components/ui/sidebar'
import { RouterLink, useRoute } from 'vue-router'
import { ArrowLeft, UserRound } from 'lucide-vue-next'

const props = defineProps<SidebarProps>()

const route = useRoute()

const workspaces = ['Мое рабочее пространство']
</script>

<template>
  <Sidebar v-bind="props">
    <SidebarHeader>
      <WorkspaceSwitcher :workspaces="workspaces" :default-workspace="workspaces[0]!" />
    </SidebarHeader>
    <SidebarContent>
      <SidebarGroup>
        <SidebarGroupContent>
          <SidebarMenu>
            <SidebarMenuItem>
              <SidebarMenuButton as-child>
                <RouterLink to="/">
                  <ArrowLeft />
                  <span>Вернуться в приложение</span>
                </RouterLink>
              </SidebarMenuButton>
            </SidebarMenuItem>
          </SidebarMenu>
        </SidebarGroupContent>
      </SidebarGroup>
      <SidebarGroup>
        <SidebarGroupLabel>Настройки</SidebarGroupLabel>
        <SidebarGroupContent>
          <SidebarMenu>
            <SidebarMenuItem>
              <SidebarMenuButton as-child :is-active="route.path === '/settings/profile'">
                <RouterLink to="/settings/profile">
                  <UserRound />
                  <span>Профиль</span>
                </RouterLink>
              </SidebarMenuButton>
            </SidebarMenuItem>
          </SidebarMenu>
        </SidebarGroupContent>
      </SidebarGroup>
    </SidebarContent>
    <SidebarFooter>
      <UserMenuDropdown />
    </SidebarFooter>
  </Sidebar>
</template>
