<script setup lang="ts">
import type { SidebarProps } from '@/components/ui/sidebar'
import WorkspaceSwitcher from '@/components/WorkspaceSwitcher.vue'
import UserMenuDropdown from '@/components/UserMenuDropdown.vue'
import { Sidebar, SidebarContent, SidebarFooter, SidebarHeader } from '@/components/ui/sidebar'

const props = defineProps<SidebarProps>()

const workspaces = ['Мое рабочее пространство']
</script>

<template>
  <Sidebar v-bind="props">
    <SidebarHeader>
      <WorkspaceSwitcher :workspaces="workspaces" :default-workspace="workspaces[0]!" />
    </SidebarHeader>
    <SidebarContent />
    <SidebarFooter>
      <UserMenuDropdown />
    </SidebarFooter>
  </Sidebar>
</template>
