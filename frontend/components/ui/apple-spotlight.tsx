"use client"

import * as React from "react"
import { Search, X, User, Globe, Upload, Settings, AlertCircle } from "lucide-react"
import { motion, AnimatePresence } from "framer-motion"
import { cn } from "@/lib/utils"
import { useRouter } from "next/navigation"
import { validateTokenAddress, parseTokenAddress } from "@/lib/token-validation"

interface Shortcut {
  label: string
  icon: React.ReactNode
  link: string
  description?: string
}

interface SearchResult {
  id: string
  label: string
  icon: React.ReactNode
  link: string
  description: string
  type: "profile" | "site" | "shortcut"
}

interface AppleSpotlightProps {
  shortcuts?: Shortcut[]
  onSearch?: (query: string) => Promise<SearchResult[]>
  defaultShortcuts?: boolean
}

export function AppleSpotlight({
  shortcuts = [],
  onSearch,
  defaultShortcuts = true,
}: AppleSpotlightProps) {
  const [isOpen, setIsOpen] = React.useState(false)
  const [query, setQuery] = React.useState("")
  const [results, setResults] = React.useState<SearchResult[]>([])
  const [isSearching, setIsSearching] = React.useState(false)
  const [selectedIndex, setSelectedIndex] = React.useState(0)
  const [validationError, setValidationError] = React.useState<string | null>(null)
  const inputRef = React.useRef<HTMLInputElement>(null)
  const router = useRouter()

  const defaultShortcutsList: Shortcut[] = [
    { label: "Profiles", icon: <User className="w-4 h-4" />, link: "/profiles" },
    { label: "Sites", icon: <Globe className="w-4 h-4" />, link: "/sites" },
    { label: "Deploy", icon: <Upload className="w-4 h-4" />, link: "/deploy" },
    { label: "Settings", icon: <Settings className="w-4 h-4" />, link: "/settings" },
  ]

  const allShortcuts = defaultShortcuts
    ? [...defaultShortcutsList, ...shortcuts]
    : shortcuts

  React.useEffect(() => {
    const handleKeyDown = (e: KeyboardEvent) => {
      if ((e.metaKey || e.ctrlKey) && e.key === "k") {
        e.preventDefault()
        setIsOpen(true)
      }
      if (e.key === "Escape" && isOpen) {
        setIsOpen(false)
        setQuery("")
        setResults([])
      }
    }

    window.addEventListener("keydown", handleKeyDown)
    return () => window.removeEventListener("keydown", handleKeyDown)
  }, [isOpen])

  React.useEffect(() => {
    if (isOpen) {
      inputRef.current?.focus()
    }
  }, [isOpen])

  React.useEffect(() => {
    const search = async () => {
      if (!query.trim()) {
        setResults([])
        setValidationError(null)
        return
      }

      // Validate token address for token-only browser
      const parsed = parseTokenAddress(query.trim())
      if (!parsed) {
        const validation = validateTokenAddress(query.trim())
        if (!validation.valid) {
          setValidationError(validation.error || "Invalid token address")
          setResults([])
          return
        }
      }

      setValidationError(null)
      setIsSearching(true)
      try {
        if (onSearch) {
          const searchResults = await onSearch(query)
          setResults(searchResults)
        } else {
          // Default search
          const filtered = allShortcuts.filter((s) =>
            s.label.toLowerCase().includes(query.toLowerCase())
          )
          setResults(
            filtered.map((s) => ({
              id: s.link,
              label: s.label,
              icon: s.icon,
              link: s.link,
              description: s.description || "",
              type: "shortcut" as const,
            }))
          )
        }
      } catch (error) {
        console.error("Search error:", error)
        setResults([])
      } finally {
        setIsSearching(false)
      }
    }

    const debounce = setTimeout(search, 300)
    return () => clearTimeout(debounce)
  }, [query, onSearch, allShortcuts])

  React.useEffect(() => {
    setSelectedIndex(0)
  }, [results])

  const handleSelect = (result: SearchResult) => {
    router.push(result.link)
    setIsOpen(false)
    setQuery("")
    setResults([])
  }

  const handleKeyDown = (e: React.KeyboardEvent) => {
    if (e.key === "ArrowDown") {
      e.preventDefault()
      setSelectedIndex((prev) =>
        prev < results.length - 1 ? prev + 1 : prev
      )
    } else if (e.key === "ArrowUp") {
      e.preventDefault()
      setSelectedIndex((prev) => (prev > 0 ? prev - 1 : 0))
    } else if (e.key === "Enter" && results[selectedIndex]) {
      e.preventDefault()
      handleSelect(results[selectedIndex])
    }
  }

  const displayResults = query.trim() ? results : []

  return (
    <>
      <AnimatePresence>
        {isOpen && (
          <>
            <motion.div
              initial={{ opacity: 0 }}
              animate={{ opacity: 1 }}
              exit={{ opacity: 0 }}
              className="fixed inset-0 bg-black/50 backdrop-blur-sm z-40"
              onClick={() => setIsOpen(false)}
            />
            <motion.div
              initial={{ opacity: 0, scale: 0.95, y: -20 }}
              animate={{ opacity: 1, scale: 1, y: 0 }}
              exit={{ opacity: 0, scale: 0.95, y: -20 }}
              transition={{ type: "spring", damping: 25, stiffness: 300 }}
              className="fixed left-1/2 top-20 -translate-x-1/2 w-full max-w-2xl z-50"
            >
              <div className="bg-background/95 backdrop-blur-xl border border-border rounded-2xl shadow-2xl overflow-hidden">
                <div className="flex items-center px-4 py-3 border-b border-border">
                  <Search className="w-5 h-5 text-muted-foreground mr-3" />
                  <input
                    ref={inputRef}
                    type="text"
                    value={query}
                    onChange={(e) => setQuery(e.target.value)}
                    onKeyDown={handleKeyDown}
                    placeholder="Enter token address (e.g., 9WzDXwBbmkg8ZTbNMqUxvQRAyrZzDsGYdLVL9zYtAWWM)..."
                    className="flex-1 bg-transparent outline-none text-foreground placeholder:text-muted-foreground"
                  />
                  {query && (
                    <button
                      onClick={() => setQuery("")}
                      className="ml-2 p-1 hover:bg-accent rounded"
                    >
                      <X className="w-4 h-4" />
                    </button>
                  )}
                </div>

                <div className="max-h-96 overflow-y-auto">
                  {validationError ? (
                    <motion.div
                      initial={{ opacity: 0, y: -10 }}
                      animate={{ opacity: 1, y: 0 }}
                      className="px-4 py-4 bg-destructive/10 border-l-4 border-destructive"
                    >
                      <div className="flex items-center gap-2 text-destructive">
                        <AlertCircle className="w-4 h-4" />
                        <span className="text-sm font-medium">{validationError}</span>
                      </div>
                      <p className="text-xs text-muted-foreground mt-1 ml-6">
                        Shadow browser only accepts SPL token addresses as domains.
                      </p>
                    </motion.div>
                  ) : isSearching ? (
                    <div className="px-4 py-8 text-center text-muted-foreground">
                      Searching...
                    </div>
                  ) : displayResults.length > 0 ? (
                    <div className="py-2">
                      {displayResults.map((result, index) => (
                        <motion.div
                          key={result.id}
                          initial={{ opacity: 0, x: -10 }}
                          animate={{ opacity: 1, x: 0 }}
                          transition={{ delay: index * 0.05 }}
                        >
                          <button
                            onClick={() => handleSelect(result)}
                            onMouseEnter={() => setSelectedIndex(index)}
                            className={cn(
                              "w-full px-4 py-3 flex items-center gap-3 text-left hover:bg-accent transition-colors",
                              selectedIndex === index && "bg-accent"
                            )}
                          >
                            <div className="text-muted-foreground">
                              {result.icon}
                            </div>
                            <div className="flex-1">
                              <div className="font-medium text-foreground">
                                {result.label}
                              </div>
                              {result.description && (
                                <div className="text-sm text-muted-foreground">
                                  {result.description}
                                </div>
                              )}
                            </div>
                          </button>
                        </motion.div>
                      ))}
                    </div>
                  ) : query.trim() ? (
                    <div className="px-4 py-8 text-center text-muted-foreground">
                      No results found
                    </div>
                  ) : (
                    <div className="px-4 py-4">
                      <div className="text-xs font-medium text-muted-foreground mb-2 px-2">
                        Shortcuts
                      </div>
                      {allShortcuts.map((shortcut, index) => (
                        <button
                          key={shortcut.link}
                          onClick={() => {
                            router.push(shortcut.link)
                            setIsOpen(false)
                          }}
                          className="w-full px-4 py-2 flex items-center gap-3 text-left hover:bg-accent rounded transition-colors"
                        >
                          <div className="text-muted-foreground">
                            {shortcut.icon}
                          </div>
                          <div className="font-medium">{shortcut.label}</div>
                        </button>
                      ))}
                    </div>
                  )}
                </div>
              </div>
            </motion.div>
          </>
        )}
      </AnimatePresence>
    </>
  )
}

