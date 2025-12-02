import * as React from "react"
import { createContext, useContext } from "react"
import { Keypair } from "@solana/web3.js"
import {
  generateWallet,
  storeWallet,
  loadWallet,
  getStoredWalletAddress,
  hasStoredWallet,
  deleteWallet,
  verifyPassword,
} from "@/lib/wallet"
import { PasswordDialog } from "./password-dialog"

interface WalletContextType {
  walletAddress: string | null
  wallet: Keypair | null
  isConnected: boolean
  isLoading: boolean
  login: () => Promise<void>
  logout: () => Promise<void>
  createNewWallet: () => Promise<void>
  unlockWallet: () => Promise<void>
}

const WalletContext = createContext<WalletContextType>({
  walletAddress: null,
  wallet: null,
  isConnected: false,
  isLoading: true,
  login: async () => {},
  logout: async () => {},
  createNewWallet: async () => {},
  unlockWallet: async () => {},
})

export function useAuth() {
  return useContext(WalletContext)
}

export function WalletProvider({ children }: { children: React.ReactNode }) {
  const [wallet, setWallet] = React.useState<Keypair | null>(null)
  const [walletAddress, setWalletAddress] = React.useState<string | null>(null)
  const [isLoading, setIsLoading] = React.useState(true)
  const [showPasswordDialog, setShowPasswordDialog] = React.useState(false)
  const [passwordDialogMode, setPasswordDialogMode] = React.useState<"create" | "unlock">("create")
  const [passwordError, setPasswordError] = React.useState<string | null>(null)

  // Check if wallet exists on mount
  React.useEffect(() => {
    const checkWallet = async () => {
      try {
        if (hasStoredWallet()) {
          // Wallet exists, need password to unlock
          const address = getStoredWalletAddress()
          setWalletAddress(address)
          setPasswordDialogMode("unlock")
          setShowPasswordDialog(true)
        } else {
          // No wallet, will create one
          setPasswordDialogMode("create")
          setShowPasswordDialog(true)
        }
      } catch (error) {
        console.error("Failed to check wallet:", error)
      } finally {
        setIsLoading(false)
      }
    }

    checkWallet()
  }, [])

  const handlePasswordConfirm = async (password: string) => {
    setPasswordError(null)
    
    try {
      if (passwordDialogMode === "create") {
        // Create new wallet with password
        const newWallet = generateWallet()
        await storeWallet(newWallet, password)
        setWallet(newWallet)
        setWalletAddress(newWallet.publicKey.toBase58())
        setShowPasswordDialog(false)
      } else {
        // Unlock existing wallet
        const loadedWallet = await loadWallet(password)
        if (loadedWallet) {
          setWallet(loadedWallet)
          setWalletAddress(loadedWallet.publicKey.toBase58())
          setShowPasswordDialog(false)
        } else {
          setPasswordError("Failed to unlock wallet")
        }
      }
    } catch (error: any) {
      setPasswordError(error.message || "Invalid password")
      throw error
    }
  }

  const createNewWallet = async () => {
    setPasswordDialogMode("create")
    setPasswordError(null)
    setShowPasswordDialog(true)
  }

  const unlockWallet = async () => {
    if (!hasStoredWallet()) {
      return
    }
    setPasswordDialogMode("unlock")
    setPasswordError(null)
    setShowPasswordDialog(true)
  }

  const login = async () => {
    // For local wallets, "login" means unlock with password
    await unlockWallet()
  }

  const logout = async () => {
    // Clear wallet from memory (but keep encrypted storage)
    setWallet(null)
    setWalletAddress(null)
    // Optionally delete wallet entirely:
    // deleteWallet()
  }

  const handleDeleteWallet = async () => {
    if (confirm("Are you sure you want to delete your wallet? This cannot be undone!")) {
      deleteWallet()
      setWallet(null)
      setWalletAddress(null)
      setPasswordDialogMode("create")
      setShowPasswordDialog(true)
    }
  }

  return (
    <WalletContext.Provider
      value={{
        walletAddress,
        wallet,
        isConnected: wallet !== null && walletAddress !== null,
        isLoading,
        login,
        logout,
        createNewWallet,
        unlockWallet,
      }}
    >
      {children}
      <PasswordDialog
        open={showPasswordDialog}
        onOpenChange={(open) => {
          setShowPasswordDialog(open)
          if (!open) {
            setPasswordError(null)
          }
        }}
        onConfirm={handlePasswordConfirm}
        title={passwordDialogMode === "create" ? "Create Encrypted Wallet" : "Unlock Wallet"}
        description={
          passwordDialogMode === "create"
            ? "Create a password to encrypt your wallet. You'll need this password to access your wallet."
            : "Enter your password to unlock your encrypted wallet."
        }
        confirmText={passwordDialogMode === "create" ? "Create Wallet" : "Unlock"}
        isNewWallet={passwordDialogMode === "create"}
        error={passwordError}
      />
    </WalletContext.Provider>
  )
}
