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

const workspaces = ['My Workspace']
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
                  <span>Back to app</span>
                </RouterLink>
              </SidebarMenuButton>
            </SidebarMenuItem>
          </SidebarMenu>
        </SidebarGroupContent>
      </SidebarGroup>
      <SidebarGroup>
        <SidebarGroupLabel>Settings</SidebarGroupLabel>
        <SidebarGroupContent>
          <SidebarMenu>
            <SidebarMenuItem>
              <SidebarMenuButton as-child :is-active="route.path === '/settings/profile'">
                <RouterLink to="/settings/profile">
                  <UserRound />
                  <span>Profile</span>
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
