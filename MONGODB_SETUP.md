# MongoDB Atlas Setup Guide

## Getting Your MongoDB Atlas Password

**Important:** MongoDB Atlas does NOT store passwords in plain text for security reasons. You **cannot retrieve** an existing password - you must **reset it**.

### Steps to Reset Password:

1. **Log in to MongoDB Atlas**
   - Go to https://www.mongodb.com/cloud/atlas
   - Sign in with your account

2. **Navigate to Database Access**
   - Click "Security" in the left sidebar
   - Click "Database Access"

3. **Edit Your User**
   - Find your user `kikilapipss_db_user` in the list
   - Click the "Edit" button (pencil icon)

4. **Reset Password**
   - Click "Edit Password" button
   - Choose one:
     - **Autogenerate Secure Password** (recommended) - click to generate
     - **Enter new password** - type your own password
   - Click "Update User"
   - **COPY THE PASSWORD** - you won't be able to see it again!

5. **Update Your Connection String**
   - Replace `<db_password>` in your connection string with the new password
   - Make sure to include the database name: `/shadow` at the end

## Connection String Format

Your connection string should look like this:

```
mongodb+srv://kikilapipss_db_user:YOUR_PASSWORD_HERE@cluster0.ilov0gu.mongodb.net/shadow?retryWrites=true&w=majority
```

**Important parts:**
- `kikilapipss_db_user` - your username
- `YOUR_PASSWORD_HERE` - your password (no `<` or `>` brackets)
- `cluster0.ilov0gu.mongodb.net` - your cluster address
- `/shadow` - the database name
- `?retryWrites=true&w=majority` - connection options

## Add to .env File

1. Copy `env.example` to `.env`:
   ```bash
   cp env.example .env
   ```

2. Edit `.env` and replace `<db_password>` with your actual password:
   ```env
   DATABASE_URL=mongodb+srv://kikilapipss_db_user:your_actual_password@cluster0.ilov0gu.mongodb.net/shadow?retryWrites=true&w=majority
   ```

3. **Keep your password secret!** Never commit `.env` to git.

## Network Access (Important!)

Make sure your IP address is whitelisted in MongoDB Atlas:

1. Go to **Security** → **Network Access** in MongoDB Atlas
2. Click "Add IP Address"
3. Choose one:
   - **Add Current IP Address** (for local development)
   - **Allow Access from Anywhere** (0.0.0.0/0) - for production (less secure)

## Test Connection

Test your connection by running the backend:

```bash
cd backend
cargo run
```

You should see:
```
🚀 Shadow backend starting on port 8080
```

If you see connection errors, check:
- Password is correct (no extra spaces)
- IP address is whitelisted
- Database name `/shadow` is included in the connection string
- Username is correct

## Security Tips

- ✅ Use a strong, unique password
- ✅ Store password in `.env` file (not in code)
- ✅ Add `.env` to `.gitignore` (already done)
- ✅ Restrict IP access to only what you need
- ✅ Use MongoDB Atlas built-in authentication
- ❌ Never commit passwords to git
- ❌ Never share passwords publicly










