<script setup lang="ts">
import { computed, ref } from 'vue'
import { ArrowUpRight, Building2, UserRound } from 'lucide-vue-next'
import { useRouter } from 'vue-router'
import { Button } from '@/components/ui/button'

type WelcomeInvitation = {
  id: string
  workspaceName: string
  inviterName: string
}

const router = useRouter()
const invitations = ref<WelcomeInvitation[]>([])

const hasInvitations = computed(() => invitations.value.length > 0)

async function handleCreateArea(): Promise<void> {
  await router.push({ name: 'home' })
}

async function handleInvitationOpen(): Promise<void> {
  await router.push({ name: 'home' })
}
</script>

<template>
  <section class="w-full max-w-3xl rounded-2xl border bg-card px-6 py-10 shadow-sm sm:px-10">
    <div class="mx-auto flex w-full max-w-xl flex-col items-center text-center">
      <h1 class="text-4xl font-semibold tracking-tight sm:text-5xl">Здравствуйте 👋</h1>
      <p class="mt-10 text-lg font-medium sm:text-xl">У вас не настроена ни одна область работы</p>
      <p class="mt-1 text-xs text-muted-foreground sm:text-base">но вы можете ее создать</p>

      <Button
        size="lg"
        class="mt-8 min-w-48 bg-black text-white hover:bg-black/90 focus-visible:ring-black/30"
        @click="handleCreateArea"
      >
        Создать область
      </Button>

      <p v-if="hasInvitations" class="mt-10 text-xs text-muted-foreground sm:text-base">
        или вступить по приглашению
      </p>

      <div class="mt-10 w-full">
        <ul v-if="hasInvitations" class="space-y-3 text-left">
          <li v-for="invitation in invitations" :key="invitation.id">
            <a
              href="#"
              class="group relative flex items-start gap-3 rounded-md border bg-background px-4 py-3 pr-10 transition-colors hover:border-border hover:bg-accent/30 focus-visible:outline-none focus-visible:ring-2 focus-visible:ring-ring/50"
              @click.prevent="handleInvitationOpen"
            >
              <ArrowUpRight
                class="absolute top-3.5 right-4 size-4 text-muted-foreground transition-colors group-hover:text-foreground"
              />
              <span
                class="mt-0.5 flex size-8 shrink-0 items-center justify-center rounded-md bg-muted text-muted-foreground group-hover:text-foreground"
              >
                <Building2 class="size-4" />
              </span>

              <div class="min-w-0 flex-1">
                <div
                  class="flex items-center gap-1 text-sm font-medium text-primary underline-offset-4 group-hover:underline"
                >
                  {{ invitation.workspaceName }}
                </div>
                <div class="mt-1 flex items-center gap-1 text-sm text-muted-foreground">
                  <UserRound class="size-3.5" />
                  Приглашение от {{ invitation.inviterName }}
                </div>
              </div>
            </a>
          </li>
        </ul>

        <div
          v-else
          class="text-right flex items-center justify-center flex-col gap-2 text-muted-foreground bg-muted sm:text-base border border-gray-200 rounded-md p-6"
        >
          <div class="text-4xl">🙂</div>
          <div>Здесь будут ваши приглашения</div>
        </div>
      </div>
    </div>
  </section>
</template>
