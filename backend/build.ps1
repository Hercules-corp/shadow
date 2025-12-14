# Shadow Backend Build Script
# Safe build script that won't hang Cursor
# 
# This script uses cargo check (faster) and avoids complex pipes
# that can cause Cursor to hang.

$ErrorActionPreference = "Continue"

Write-Host "🔍 Checking Rust backend..." -ForegroundColor Cyan
Write-Host "Using cargo check (faster, no artifacts)..." -ForegroundColor Gray

# Capture output to avoid hanging
$output = cargo check 2>&1

# Filter errors/warnings without blocking pipes
$errors = $output | Where-Object { $_ -match "error\[" }
$warnings = $output | Where-Object { $_ -match "warning:" }

if ($errors) {
    Write-Host "`n❌ Compilation errors found:" -ForegroundColor Red
    $errors | Select-Object -First 10 | ForEach-Object { Write-Host $_ }
    exit 1
}

if ($output -match "Finished.*target") {
    Write-Host "✅ Backend compiles successfully!" -ForegroundColor Green
    if ($warnings) {
        $warningCount = ($warnings | Measure-Object).Count
        Write-Host "⚠️  $warningCount warnings (non-critical)" -ForegroundColor Yellow
    }
    exit 0
} else {
    Write-Host "❌ Unexpected output from cargo check" -ForegroundColor Red
    exit 1
}

