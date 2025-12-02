"use client"

import { useEffect, useState } from "react"
import { useParams, useRouter } from "next/navigation"
import { motion } from "framer-motion"
import { Button } from "@/components/ui/button"
import { ArrowLeft, Globe } from "lucide-react"

interface Site {
  program_address: string
  owner_pubkey: string
  storage_cid: string
  name: string | null
  description: string | null
}

export default function SitePage() {
  const params = useParams()
  const router = useRouter()
  const program = params.program as string
  const [site, setSite] = useState<Site | null>(null)
  const [content, setContent] = useState<string>("")
  const [loading, setLoading] = useState(true)

  useEffect(() => {
    const fetchSite = async () => {
      try {
        const backendUrl = process.env.NEXT_PUBLIC_BACKEND_URL || "http://localhost:8080"
        const res = await fetch(`${backendUrl}/api/sites/${program}`)
        if (res.ok) {
          const data = await res.json()
          setSite(data)

          // Fetch content
          const contentRes = await fetch(`${backendUrl}/api/sites/${program}/content`)
          if (contentRes.ok) {
            const html = await contentRes.text()
            setContent(html)
          }
        }
      } catch (error) {
        console.error("Error fetching site:", error)
      } finally {
        setLoading(false)
      }
    }

    if (program) {
      fetchSite()
    }
  }, [program])

  if (loading) {
    return (
      <div className="min-h-screen flex items-center justify-center">
        <div className="text-muted-foreground">Loading...</div>
      </div>
    )
  }

  return (
    <div className="min-h-screen bg-background">
      <div className="container mx-auto px-4 py-8">
        <Button
          variant="ghost"
          onClick={() => router.back()}
          className="mb-6"
        >
          <ArrowLeft className="w-4 h-4 mr-2" />
          Back
        </Button>

        <motion.div
          initial={{ opacity: 0, y: 20 }}
          animate={{ opacity: 1, y: 0 }}
          className="max-w-4xl mx-auto"
        >
          {site && (
            <div className="bg-card border border-border rounded-2xl p-8 mb-6">
              <h1 className="text-3xl font-bold mb-2">
                {site.name || site.program_address}
              </h1>
              {site.description && (
                <p className="text-muted-foreground mb-4">{site.description}</p>
              )}
            </div>
          )}

          {content ? (
            <div
              className="bg-card border border-border rounded-2xl p-8 min-h-[400px]"
              dangerouslySetInnerHTML={{ __html: content }}
            />
          ) : (
            <div className="bg-card border border-border rounded-2xl p-8 text-center text-muted-foreground">
              No content available
            </div>
          )}
        </motion.div>
      </div>
    </div>
  )
}

