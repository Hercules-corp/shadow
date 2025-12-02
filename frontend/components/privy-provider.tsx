"use client"

import * as React from "react"
import { createContext, useContext } from "react"
import { PrivyProvider as PrivyProviderBase, usePrivy } from "@privy-io/react-auth"

interface PrivyContextType {
  walletAddress: string | null
  isConnected: boolean
  login: () => Promise<void>
  logout: () => Promise<void>
}

const PrivyContext = createContext<PrivyContextType>({
  walletAddress: null,
  isConnected: false,
  login: async () => {},
  logout: async () => {},
})

export function useAuth() {
  return useContext(PrivyContext)
}

function PrivyContextProvider({ children }: { children: React.ReactNode }) {
  const { ready, authenticated, login, logout, user } = usePrivy()
  const [walletAddress, setWalletAddress] = React.useState<string | null>(null)

  React.useEffect(() => {
    if (authenticated && user) {
      // Get Solana wallet address from Privy
      const wallets = user.linkedAccounts.filter((account) => 
        account.type === "wallet" && account.walletClientType === "privy"
      )
      
      if (wallets.length > 0) {
        // Privy embedded wallet address
        const address = wallets[0].address
        setWalletAddress(address)
      } else if (user.wallet?.address) {
        // Fallback to wallet address
        setWalletAddress(user.wallet.address)
      } else {
        setWalletAddress(null)
      }
    } else {
      setWalletAddress(null)
    }
  }, [authenticated, user])

  const handleLogin = async () => {
    try {
      await login()
    } catch (error) {
      console.error("Login error:", error)
    }
  }

  const handleLogout = async () => {
    try {
      await logout()
      setWalletAddress(null)
    } catch (error) {
      console.error("Logout error:", error)
    }
  }

  return (
    <PrivyContext.Provider
      value={{
        walletAddress,
        isConnected: authenticated && ready,
        login: handleLogin,
        logout: handleLogout,
      }}
    >
      {children}
    </PrivyContext.Provider>
  )
}

export function PrivyProvider({ children }: { children: React.ReactNode }) {
  const appId = process.env.NEXT_PUBLIC_PRIVY_APP_ID || ""
  
  if (!appId) {
    console.warn("PRIVY_APP_ID is not set. Authentication will not work.")
  }

  return (
    <PrivyProviderBase
      appId={appId}
      config={{
        loginMethods: ["google", "wallet"],
        embeddedWallets: {
          createOnLogin: "users-without-wallets",
          requireUserPasswordOnCreate: false,
          noPromptOnSignature: false,
        },
        appearance: {
          theme: "dark",
          accentColor: "#000000",
        },
        // Solana configuration
        supportedChains: [],
        // Note: Privy's Solana support may require additional configuration
        // Check Privy docs for latest Solana integration
      }}
    >
      <PrivyContextProvider>{children}</PrivyContextProvider>
    </PrivyProviderBase>
  )
}
