<template>
  <div class="profile">
    <header class="header">
      <h1>Profile</h1>
      <router-link to="/" class="nav-link">Home</router-link>
    </header>

    <main class="main-content">
      <div v-if="wallet.walletAddress" class="profile-content">
        <h2>Wallet Address</h2>
        <p class="wallet-address">{{ wallet.walletAddress }}</p>
        <button @click="handleLogout" class="btn btn-secondary">Logout</button>
      </div>
      <div v-else>
        <p>No wallet connected</p>
        <router-link to="/" class="btn btn-primary">Go Home</router-link>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { useWalletStore } from '@/stores/wallet'
import { useRouter } from 'vue-router'

const wallet = useWalletStore()
const router = useRouter()

async function handleLogout() {
  await wallet.logout()
  router.push('/')
}
</script>

<style scoped>
.profile {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

.header {
  display: flex;
  justify-content: space-between;
  align-items: center;
  padding: 1rem 2rem;
  border-bottom: 1px solid var(--border-color);
}

.main-content {
  flex: 1;
  padding: 2rem;
}

.profile-content {
  max-width: 600px;
}

.wallet-address {
  font-family: monospace;
  word-break: break-all;
  margin: 1rem 0;
  padding: 1rem;
  background: var(--bg-secondary);
  border-radius: 0.5rem;
}

.btn {
  padding: 0.5rem 1rem;
  border: none;
  border-radius: 0.25rem;
  cursor: pointer;
  font-weight: 500;
  margin-top: 1rem;
}

.btn-primary {
  background: var(--primary-color);
  color: white;
}

.btn-secondary {
  background: var(--bg-secondary);
  color: var(--text-color);
}
</style>

