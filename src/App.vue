<script setup lang="ts">
import { useUsageStore } from './stores/usage'
import Dashboard from './views/Dashboard.vue'
import SettingsModal from './components/SettingsModal.vue'

const store = useUsageStore()
</script>

<template>
  <Dashboard />
  <SettingsModal
    v-if="store.showSettings"
    @close="store.showSettings = false"
  />
</template>
