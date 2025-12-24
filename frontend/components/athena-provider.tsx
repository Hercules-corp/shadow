"use client"

import * as React from "react"
import { createContext, useContext, useState, useEffect, useCallback } from "react"
import { Keypair } from "@solana/web3.js"
import {
  generateWallet,
  saveWallet,
  loadWallet,
  getWalletAddress,
  hasWallet,
  deleteWallet,
} from "@/lib/athena-wallet"
import { Dialog, DialogContent, DialogDescription, DialogHeader, DialogTitle } from "@/components/ui/dialog"
import { Input } from "@/components/ui/input"
import { Button } from "@/components/ui/button"

interface AthenaContextType {
  walletAddress: string | null
  isConnected: boolean
  keypair: Keypair | null
  login: (password?: string) => Promise<void>
  logout: () => Promise<void>
  createWallet: (password: string) => Promise<void>
  unlockWallet: (password: string) => Promise<void>
  isLocked: boolean
  isLoading: boolean
}

const AthenaContext = createContext<AthenaContextType>({
  walletAddress: null,
  isConnected: false,
  keypair: null,
  login: async () => {},
  logout: async () => {},
  createWallet: async () => {},
  unlockWallet: async () => {},
  isLocked: true,
  isLoading: true,
})

export function useAuth() {
  return useContext(AthenaContext)
}

export function AthenaProvider({ children }: { children: React.ReactNode }) {
  const [walletAddress, setWalletAddress] = useState<string | null>(null)
  const [keypair, setKeypair] = useState<Keypair | null>(null)
  const [isLocked, setIsLocked] = useState(true)
  const [isLoading, setIsLoading] = useState(true)
  const [showPasswordDialog, setShowPasswordDialog] = useState(false)
  const [password, setPassword] = useState("")
  const [isCreating, setIsCreating] = useState(false)

  // Check for existing wallet on mount
  useEffect(() => {
    const address = getWalletAddress()
    if (address) {
      setWalletAddress(address)
      setIsLocked(true) // Wallet exists but needs password to unlock
    }
    setIsLoading(false)
  }, [])

  const createWallet = useCallback(async (walletPassword: string) => {
    try {
      const newKeypair = generateWallet()
      await saveWallet(newKeypair, walletPassword)
      
      setKeypair(newKeypair)
      setWalletAddress(newKeypair.publicKey.toBase58())
      setIsLocked(false)
      setShowPasswordDialog(false)
      setPassword("")
    } catch (error) {
      console.error("Failed to create wallet:", error)
      throw error
    }
  }, [])

  const unlockWallet = useCallback(async (walletPassword: string) => {
    try {
      const loadedKeypair = await loadWallet(walletPassword)
      if (!loadedKeypair) {
        throw new Error("Failed to decrypt wallet. Incorrect password?")
      }

      setKeypair(loadedKeypair)
      setWalletAddress(loadedKeypair.publicKey.toBase58())
      setIsLocked(false)
      setShowPasswordDialog(false)
      setPassword("")
    } catch (error) {
      console.error("Failed to unlock wallet:", error)
      throw error
    }
  }, [])

  const login = useCallback(async (walletPassword?: string) => {
    if (hasWallet()) {
      // Wallet exists, show password dialog
      if (walletPassword) {
        await unlockWallet(walletPassword)
      } else {
        setShowPasswordDialog(true)
      }
    } else {
      // No wallet, show create dialog
      setShowPasswordDialog(true)
      setIsCreating(true)
    }
  }, [unlockWallet])

  const logout = useCallback(async () => {
    setKeypair(null)
    setWalletAddress(null)
    setIsLocked(true)
    // Don't delete wallet, just lock it
  }, [])

  const handlePasswordSubmit = async () => {
    if (!password.trim()) {
      return
    }

    try {
      if (isCreating) {
        await createWallet(password)
        setIsCreating(false)
      } else {
        await unlockWallet(password)
      }
    } catch (error) {
      alert(error instanceof Error ? error.message : "Failed to authenticate")
    }
  }

  return (
    <AthenaContext.Provider
      value={{
        walletAddress,
        isConnected: !isLocked && keypair !== null,
        keypair,
        login,
        logout,
        createWallet,
        unlockWallet,
        isLocked,
        isLoading,
      }}
    >
      {children}
      
      <Dialog open={showPasswordDialog} onOpenChange={setShowPasswordDialog}>
        <DialogContent>
          <DialogHeader>
            <DialogTitle>
              {isCreating ? "Create Athena Wallet" : "Unlock Athena Wallet"}
            </DialogTitle>
            <DialogDescription>
              {isCreating
                ? "Create a new local wallet. Your private key will be encrypted with your password."
                : "Enter your password to unlock your wallet."}
            </DialogDescription>
          </DialogHeader>
          <div className="space-y-4">
            <Input
              type="password"
              placeholder="Password"
              value={password}
              onChange={(e) => setPassword(e.target.value)}
              onKeyDown={(e) => {
                if (e.key === "Enter") {
                  handlePasswordSubmit()
                }
              }}
              autoFocus
            />
            <div className="flex gap-2 justify-end">
              <Button
                variant="outline"
                onClick={() => {
                  setShowPasswordDialog(false)
                  setPassword("")
                  setIsCreating(false)
                }}
              >
                Cancel
              </Button>
              <Button onClick={handlePasswordSubmit}>
                {isCreating ? "Create" : "Unlock"}
              </Button>
            </div>
          </div>
        </DialogContent>
      </Dialog>
    </AthenaContext.Provider>
  )
}

