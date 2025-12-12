<template>
  <div class="home">
    <header class="header">
      <h1>Shadow Browser</h1>
      <nav>
        <router-link v-if="wallet.isConnected" to="/profile" class="nav-link">
          Profile
        </router-link>
      </nav>
    </header>

    <main class="main-content">
      <div v-if="wallet.isLoading" class="loading">
        <p>Loading...</p>
      </div>

      <div v-else-if="wallet.isConnected && wallet.walletAddress" class="wallet-connected">
        <div class="wallet-icon">👛</div>
        <h2>Wallet Connected</h2>
        <p class="wallet-address">{{ wallet.walletAddress }}</p>
        <button @click="handleLogout" class="btn btn-secondary">Logout</button>
      </div>

      <div v-else class="wallet-prompt">
        <button @click="showCreateDialog" class="btn btn-primary">Create Wallet</button>
      </div>
    </main>

    <!-- Password Dialog -->
    <div v-if="wallet.showPasswordDialog" class="dialog-overlay" @click="closeDialog">
      <div class="dialog" @click.stop>
        <h3>{{ wallet.passwordDialogMode === 'create' ? 'Create Wallet' : 'Unlock Wallet' }}</h3>
        <form @submit.prevent="handlePasswordSubmit">
          <div v-if="wallet.passwordError" class="error">
            {{ wallet.passwordError }}
          </div>
          <div class="form-group">
            <label for="password">Password</label>
            <input
              id="password"
              v-model="password"
              type="password"
              required
              :minlength="wallet.passwordDialogMode === 'create' ? 8 : 0"
            />
          </div>
          <div v-if="wallet.passwordDialogMode === 'create'" class="form-group">
            <label for="confirmPassword">Confirm Password</label>
            <input
              id="confirmPassword"
              v-model="confirmPassword"
              type="password"
              required
            />
          </div>
          <div class="dialog-actions">
            <button type="button" @click="closeDialog" class="btn btn-secondary">Cancel</button>
            <button type="submit" class="btn btn-primary">
              {{ wallet.passwordDialogMode === 'create' ? 'Create' : 'Unlock' }}
            </button>
          </div>
        </form>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useWalletStore } from '@/stores/wallet'

const wallet = useWalletStore()
const password = ref('')
const confirmPassword = ref('')

onMounted(async () => {
  await wallet.checkWallet()
  if (wallet.walletExists() && !wallet.isConnected) {
    showUnlockDialog()
  } else if (!wallet.walletExists()) {
    showCreateDialog()
  }
})

function showCreateDialog() {
  wallet.passwordDialogMode = 'create'
  wallet.showPasswordDialog = true
  wallet.passwordError = null
  password.value = ''
  confirmPassword.value = ''
}

function showUnlockDialog() {
  wallet.passwordDialogMode = 'unlock'
  wallet.showPasswordDialog = true
  wallet.passwordError = null
  password.value = ''
  confirmPassword.value = ''
}

function closeDialog() {
  wallet.showPasswordDialog = false
  wallet.passwordError = null
  password.value = ''
  confirmPassword.value = ''
}

async function handlePasswordSubmit() {
  if (wallet.passwordDialogMode === 'create') {
    if (password.value.length < 8) {
      wallet.passwordError = 'Password must be at least 8 characters'
      return
    }
    if (password.value !== confirmPassword.value) {
      wallet.passwordError = 'Passwords do not match'
      return
    }
    try {
      await wallet.createNewWallet(password.value)
    } catch (error) {
      // Error already set in store
    }
  } else {
    try {
      await wallet.unlockWallet(password.value)
    } catch (error) {
      // Error already set in store
    }
  }
}

async function handleLogout() {
  await wallet.logout()
  showCreateDialog()
}
</script>

<style scoped>
.home {
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
  display: flex;
  align-items: center;
  justify-content: center;
  padding: 2rem;
}

.wallet-connected,
.wallet-prompt {
  text-align: center;
}

.wallet-icon {
  font-size: 4rem;
  margin-bottom: 1rem;
}

.wallet-address {
  font-family: monospace;
  word-break: break-all;
  margin: 1rem 0;
  padding: 1rem;
  background: var(--bg-secondary);
  border-radius: 0.5rem;
}

.dialog-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
}

.dialog {
  background: var(--bg-primary);
  padding: 2rem;
  border-radius: 0.5rem;
  min-width: 400px;
  max-width: 90vw;
}

.form-group {
  margin-bottom: 1rem;
}

.form-group label {
  display: block;
  margin-bottom: 0.5rem;
}

.form-group input {
  width: 100%;
  padding: 0.5rem;
  border: 1px solid var(--border-color);
  border-radius: 0.25rem;
}

.error {
  color: var(--error-color);
  margin-bottom: 1rem;
}

.dialog-actions {
  display: flex;
  gap: 1rem;
  justify-content: flex-end;
  margin-top: 1.5rem;
}

.btn {
  padding: 0.5rem 1rem;
  border: none;
  border-radius: 0.25rem;
  cursor: pointer;
  font-weight: 500;
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

