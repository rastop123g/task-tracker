<script setup lang="ts">
import { Check, ChevronsUpDown, GalleryVerticalEnd } from 'lucide-vue-next'
import { ref } from 'vue'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { SidebarMenu, SidebarMenuButton, SidebarMenuItem } from '@/components/ui/sidebar'

const props = defineProps<{
  workspaces: string[]
  defaultWorkspace: string
}>()

const selectedWorkspace = ref(props.defaultWorkspace)
</script>

<template>
  <SidebarMenu>
    <SidebarMenuItem>
      <DropdownMenu>
        <DropdownMenuTrigger as-child>
          <SidebarMenuButton
            size="lg"
            class="data-[state=open]:bg-sidebar-accent data-[state=open]:text-sidebar-accent-foreground"
          >
            <div
              class="bg-sidebar-primary text-sidebar-primary-foreground flex aspect-square size-8 items-center justify-center rounded-lg"
            >
              <GalleryVerticalEnd class="size-4" />
            </div>
            <span class="font-medium leading-none">{{ selectedWorkspace }}</span>
            <ChevronsUpDown class="ml-auto" />
          </SidebarMenuButton>
        </DropdownMenuTrigger>
        <DropdownMenuContent class="w-(--reka-dropdown-menu-trigger-width)" align="start">
          <DropdownMenuItem
            v-for="workspace in workspaces"
            :key="workspace"
            @select="selectedWorkspace = workspace"
          >
            {{ workspace }}
            <Check v-if="workspace === selectedWorkspace" class="ml-auto" />
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </SidebarMenuItem>
  </SidebarMenu>
</template>
