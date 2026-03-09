<script setup lang="ts">
import { ref, computed } from 'vue'
import { useRouter, RouterLink } from 'vue-router'
import { ChevronsUpDown, Settings, LogOut } from 'lucide-vue-next'
import {
  DropdownMenu,
  DropdownMenuContent,
  DropdownMenuItem,
  DropdownMenuSeparator,
  DropdownMenuTrigger,
} from '@/components/ui/dropdown-menu'
import { SidebarMenu, SidebarMenuButton, SidebarMenuItem } from '@/components/ui/sidebar'
import { currentUserStore } from '@/app/session/current-user-store'
import { useProfile } from '@/composables/useProfile'
import { useAuth } from '@/composables/useAuth'

const { profile, avatarUrl } = useProfile()
const { clearTokens } = useAuth()
const router = useRouter()

const showInitials = ref(false)

const displayName = computed(() => profile.value?.name || profile.value?.email || '—')
const initials = computed(() => displayName.value.charAt(0).toUpperCase() || '?')
const avatarSrc = computed(() => (avatarUrl.value ? `${avatarUrl.value}?t=0` : null))

function onSignOut() {
  clearTokens()
  currentUserStore.clear()
  router.push({ name: 'login' })
}
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
            <!-- Avatar -->
            <div class="relative size-8 shrink-0">
              <img
                v-if="avatarSrc && !showInitials"
                :src="avatarSrc"
                alt="Аватар"
                class="size-8 rounded-full object-cover"
                @error="showInitials = true"
              />
              <div
                v-else
                class="size-8 rounded-full flex items-center justify-center bg-sidebar-primary text-sidebar-primary-foreground text-sm font-semibold"
              >
                {{ initials }}
              </div>
            </div>

            <!-- Name + email -->
            <div class="flex min-w-0 flex-col leading-none">
              <span class="truncate font-medium">{{ displayName }}</span>
              <span class="truncate text-xs text-muted-foreground">{{
                profile?.email || '—'
              }}</span>
            </div>

            <ChevronsUpDown class="ml-auto shrink-0" />
          </SidebarMenuButton>
        </DropdownMenuTrigger>

        <DropdownMenuContent
          class="w-(--reka-dropdown-menu-trigger-width) min-w-56"
          side="top"
          align="start"
        >
          <DropdownMenuItem as-child>
            <RouterLink to="/settings/profile" class="flex items-center gap-2 cursor-pointer">
              <Settings class="size-4" />
              <span>Настройки</span>
            </RouterLink>
          </DropdownMenuItem>
          <DropdownMenuSeparator />
          <DropdownMenuItem
            class="text-destructive focus:text-destructive cursor-pointer"
            @select="onSignOut"
          >
            <LogOut class="size-4" />
            <span>Выйти</span>
          </DropdownMenuItem>
        </DropdownMenuContent>
      </DropdownMenu>
    </SidebarMenuItem>
  </SidebarMenu>
</template>
