import { useState, useEffect } from "react"
import { motion, AnimatePresence } from "framer-motion"
import { AppleSpotlight } from "@/components/ui/apple-spotlight"
import { useAuth } from "@/components/wallet-provider"
import { Button } from "@/components/ui/button"
import { ThemeToggle } from "@/components/theme-toggle"
import { shortenAddress } from "@/lib/utils"
import { hasStoredWallet } from "@/lib/wallet"
import { User, Globe, Lock } from "lucide-react"
import { useNavigate } from "react-router-dom"

interface SearchResult {
  id: string
  label: string
  icon: React.ReactNode
  link: string
  description: string
  type: "profile" | "site" | "shortcut"
}

async function search(query: string): Promise<SearchResult[]> {
  const backendUrl = import.meta.env.VITE_BACKEND_URL || "http://localhost:8080"
  const results: SearchResult[] = []

  try {
    // Search profiles
    const profilesRes = await fetch(`${backendUrl}/api/profiles/search?q=${encodeURIComponent(query)}&limit=5`)
    if (profilesRes.ok) {
      const profiles = await profilesRes.json()
      results.push(...profiles.map((p: any) => ({
        id: `profile-${p.wallet_pubkey}`,
        label: shortenAddress(p.wallet_pubkey),
        icon: <User className="w-4 h-4" />,
        link: `/profile/${p.wallet_pubkey}`,
        description: p.is_public ? "Public profile" : "Anonymous profile",
        type: "profile" as const,
      })))
    }

    // Search sites
    const sitesRes = await fetch(`${backendUrl}/api/sites/search?q=${encodeURIComponent(query)}&limit=5`)
    if (sitesRes.ok) {
      const sites = await sitesRes.json()
      results.push(...sites.map((s: any) => ({
        id: `site-${s.program_address}`,
        label: s.name || shortenAddress(s.program_address),
        icon: <Globe className="w-4 h-4" />,
        link: `/site/${s.program_address}`,
        description: s.description || "Site on Shadow",
        type: "site" as const,
      })))
    }

    // Search Solana
    const solanaRes = await fetch(`${backendUrl}/api/solana/search?q=${encodeURIComponent(query)}`)
    if (solanaRes.ok) {
      const solana = await solanaRes.json()
      if (solana.type === "program" || solana.type === "account") {
        results.push({
          id: `solana-${solana.data.address}`,
          label: shortenAddress(solana.data.address),
          icon: solana.type === "program" ? <Globe className="w-4 h-4" /> : <User className="w-4 h-4" />,
          link: solana.type === "program" ? `/site/${solana.data.address}` : `/profile/${solana.data.address}`,
          description: solana.type === "program" ? "Solana program" : "Solana account",
          type: solana.type === "program" ? "site" : "profile",
        })
      }
    }
  } catch (error) {
    console.error("Search error:", error)
  }

  return results
}

export default function Home() {
  const { isConnected, walletAddress, isLoading, createNewWallet, unlockWallet, logout } = useAuth()
  const [showWelcome, setShowWelcome] = useState(false)
  const navigate = useNavigate()

  useEffect(() => {
    if (isConnected && walletAddress && !isLoading) {
      setShowWelcome(true)
      const timer = setTimeout(() => setShowWelcome(false), 5000)
      return () => clearTimeout(timer)
    }
  }, [isConnected, walletAddress, isLoading])

  return (
    <div className="min-h-screen bg-gradient-to-br from-neutral-50 via-neutral-100 to-neutral-200 dark:from-neutral-950 dark:via-neutral-900 dark:to-neutral-800 safe-area-inset">
      <div className="absolute inset-0 bg-[radial-gradient(circle_at_50%_50%,rgba(0,0,0,0.03),transparent)] dark:bg-[radial-gradient(circle_at_50%_50%,rgba(255,255,255,0.03),transparent)]" />
      
      <div className="fixed top-4 right-4 z-30 safe-area-top">
        <ThemeToggle />
      </div>
      
      <AnimatePresence>
        {showWelcome && walletAddress && (
          <motion.div
            initial={{ opacity: 0, y: -20 }}
            animate={{ opacity: 1, y: 0 }}
            exit={{ opacity: 0, y: -20 }}
            className="fixed top-4 left-1/2 -translate-x-1/2 z-50 bg-background/95 backdrop-blur-xl border border-border rounded-2xl shadow-xl p-6 max-w-md mx-4"
          >
            <motion.div
              initial={{ scale: 0.95 }}
              animate={{ scale: 1 }}
              transition={{ type: "spring", stiffness: 300 }}
            >
              <h2 className="text-2xl font-bold mb-2">Welcome, {shortenAddress(walletAddress)}</h2>
              <p className="text-muted-foreground mb-2">
                You're whatever your wallet is — forget who you are online.
              </p>
              <p className="text-sm text-muted-foreground">
                Your wallet is encrypted with AES-256-GCM. No accounts, no emails, no tracking.
              </p>
            </motion.div>
          </motion.div>
        )}
      </AnimatePresence>

      <div className="relative z-10 container mx-auto px-4 py-16 safe-area-inset">
        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ duration: 0.5 }}
          className="text-center mb-12"
        >
          <motion.h1
            initial={{ opacity: 0, scale: 0.95 }}
            animate={{ opacity: 1, scale: 1 }}
            transition={{ delay: 0.2, type: "spring", stiffness: 200 }}
            className="text-5xl sm:text-6xl md:text-8xl font-bold mb-4 bg-clip-text text-transparent bg-gradient-to-r from-foreground to-muted-foreground"
          >
            Shadow
          </motion.h1>
          <motion.p
            initial={{ opacity: 0 }}
            animate={{ opacity: 1 }}
            transition={{ delay: 0.4 }}
            className="text-lg sm:text-xl md:text-2xl text-muted-foreground mb-8 px-4"
          >
            You are whatever your wallet is. Forget who you are online.
          </motion.p>

          {isLoading ? (
            <motion.div
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: 0.6 }}
              className="text-muted-foreground"
            >
              {hasStoredWallet() ? "Unlocking wallet..." : "Creating wallet..."}
            </motion.div>
          ) : !isConnected ? (
            <motion.div
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: 0.6 }}
              className="text-muted-foreground"
            >
              Enter your password to continue
            </motion.div>
          ) : (
            <motion.div
              initial={{ opacity: 0, y: 10 }}
              animate={{ opacity: 1, y: 0 }}
              transition={{ delay: 0.6 }}
              className="flex flex-col sm:flex-row items-center justify-center gap-4 px-4"
            >
              <Button
                onClick={() => navigate("/profile/" + walletAddress)}
                variant="outline"
                size="lg"
                className="w-full sm:w-auto touch-manipulation min-h-[48px]"
              >
                My Profile
              </Button>
              <Button
                onClick={createNewWallet}
                variant="outline"
                size="lg"
                className="w-full sm:w-auto touch-manipulation min-h-[48px]"
              >
                <Lock className="w-4 h-4 mr-2" />
                New Wallet
              </Button>
              {hasStoredWallet() && (
                <Button
                  onClick={unlockWallet}
                  variant="outline"
                  size="lg"
                  className="w-full sm:w-auto touch-manipulation min-h-[48px]"
                >
                  <Lock className="w-4 h-4 mr-2" />
                  Unlock Wallet
                </Button>
              )}
              <Button
                onClick={logout}
                variant="ghost"
                size="lg"
                className="w-full sm:w-auto touch-manipulation min-h-[48px]"
              >
                Delete Wallet
              </Button>
            </motion.div>
          )}
        </motion.div>

        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          transition={{ delay: 0.8 }}
          className="max-w-2xl mx-auto px-4"
        >
          <AppleSpotlight onSearch={search} />
        </motion.div>
      </div>
    </div>
  )
}

