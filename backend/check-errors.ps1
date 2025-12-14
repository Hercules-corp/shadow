# Safe command to check for compilation errors
# Run this manually in your terminal instead of through Cursor

$ErrorActionPreference = "Continue"
$output = & cargo check --message-format=short 2>&1
$errors = $output | Select-String -Pattern "error\[" | Select-Object -First 20
if ($errors) {
    Write-Host "=== COMPILATION ERRORS ===" -ForegroundColor Red
    $errors
} else {
    Write-Host "No compilation errors found!" -ForegroundColor Green
    $warnings = $output | Select-String -Pattern "warning:" | Select-Object -First 10
    if ($warnings) {
        Write-Host "`n=== WARNINGS ===" -ForegroundColor Yellow
        $warnings
    }
}

