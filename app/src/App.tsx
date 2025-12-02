import { Routes, Route } from 'react-router-dom'
import { ThemeProvider } from '@/components/theme-provider'
import { WalletProvider } from '@/components/wallet-provider'
import Home from './pages/Home'
import ProfilePage from './pages/ProfilePage'
import SitePage from './pages/SitePage'

function App() {
  return (
    <ThemeProvider
      attribute="class"
      defaultTheme="dark"
      enableSystem
      disableTransitionOnChange
    >
      <WalletProvider>
        <div className="min-h-screen bg-background">
          <Routes>
            <Route path="/" element={<Home />} />
            <Route path="/profile/:wallet" element={<ProfilePage />} />
            <Route path="/site/:program" element={<SitePage />} />
          </Routes>
        </div>
      </WalletProvider>
    </ThemeProvider>
  )
}

export default App
