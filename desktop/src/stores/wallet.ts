import { defineStore } from 'pinia'
import { ref, computed } from 'vue'
import { Keypair } from '@solana/web3.js'
import {
  generateWallet,
  storeWallet,
  loadWallet,
  getStoredWalletAddress,
  hasStoredWallet,
  deleteWallet,
  verifyPassword,
} from '@/lib/wallet'

export const useWalletStore = defineStore('wallet', () => {
  const wallet = ref<Keypair | null>(null)
  const walletAddress = ref<string | null>(null)
  const isLoading = ref(true)
  const showPasswordDialog = ref(false)
  const passwordDialogMode = ref<'create' | 'unlock'>('create')
  const passwordError = ref<string | null>(null)

  const isConnected = computed(() => wallet.value !== null)

  // Check if wallet exists on mount
  async function checkWallet() {
    isLoading.value = true
    const address = getStoredWalletAddress()
    if (address) {
      walletAddress.value = address
    }
    isLoading.value = false
  }

  // Create new wallet
  async function createNewWallet(password: string): Promise<void> {
    try {
      const keypair = generateWallet()
      await storeWallet(keypair, password)
      wallet.value = keypair
      walletAddress.value = keypair.publicKey.toBase58()
      showPasswordDialog.value = false
      passwordError.value = null
    } catch (error) {
      passwordError.value = error instanceof Error ? error.message : 'Failed to create wallet'
      throw error
    }
  }

  // Unlock existing wallet
  async function unlockWallet(password: string): Promise<void> {
    try {
      const keypair = await loadWallet(password)
      if (keypair) {
        wallet.value = keypair
        walletAddress.value = keypair.publicKey.toBase58()
        showPasswordDialog.value = false
        passwordError.value = null
      } else {
        throw new Error('No wallet found')
      }
    } catch (error) {
      passwordError.value = error instanceof Error ? error.message : 'Failed to unlock wallet'
      throw error
    }
  }

  // Logout
  async function logout(): Promise<void> {
    deleteWallet()
    wallet.value = null
    walletAddress.value = null
  }

  // Check if wallet exists
  function walletExists(): boolean {
    return hasStoredWallet()
  }

  // Verify password
  async function checkPassword(password: string): Promise<boolean> {
    return await verifyPassword(password)
  }

  return {
    wallet,
    walletAddress,
    isLoading,
    isConnected,
    showPasswordDialog,
    passwordDialogMode,
    passwordError,
    checkWallet,
    createNewWallet,
    unlockWallet,
    logout,
    walletExists,
    checkPassword,
  }
})

