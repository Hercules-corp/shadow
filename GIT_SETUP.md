# Git Setup Instructions for Shadow Repository

## Step 1: Configure Git (if not already done)

```powershell
git config --global user.name "Hercules-corp"
git config --global user.email "hercules-corp@users.noreply.github.com"
```

## Step 2: Initialize Git Repository

```powershell
cd "C:\Users\1\Documents\milla projects\shadow"
git init
```

## Step 3: Add Remote Repository

```powershell
git remote add origin https://github.com/Hercules-corp/shadow.git
```

## Step 4: Add All Files

```powershell
git add .
```

## Step 5: Create Initial Commit

```powershell
git commit -m "starting shadow"
```

## Step 6: Set Default Branch to main (if needed)

```powershell
git branch -M main
```

## Step 7: Push to GitHub

```powershell
git push -u origin main
```

**Note:** If this is your first push to GitHub, you may be prompted for authentication. You can either:
- Use a Personal Access Token (recommended)
- Use GitHub CLI (`gh auth login`)
- Or use SSH instead of HTTPS

### Alternative: Using SSH (if you have SSH keys set up)

If you prefer SSH, change the remote URL:
```powershell
git remote set-url origin git@github.com:Hercules-corp/shadow.git
git push -u origin main
```

---

## Quick Copy-Paste (All at Once)

```powershell
cd "C:\Users\1\Documents\milla projects\shadow"
git init
git remote add origin https://github.com/Hercules-corp/shadow.git
git add .
git commit -m "starting shadow"
git branch -M main
git push -u origin main
```

