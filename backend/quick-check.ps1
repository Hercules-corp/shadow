# Quick compilation check - minimal output
# Safe command that won't hang Cursor

$output = cargo check --message-format=short 2>&1
$errors = $output | Where-Object { $_ -match "error\[" }

if ($errors) {
    $errors | Select-Object -First 10
    exit 1
} else {
    Write-Host "✅ No compilation errors" -ForegroundColor Green
    exit 0
}

