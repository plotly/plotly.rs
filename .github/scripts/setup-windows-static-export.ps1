# Windows environment setup for plotly static export tests
# This script sets up Chrome, chromedriver, and environment variables for Windows CI

param(
    [string]$ChromeVersion,
    [string]$ChromePath,
    [string]$ChromeDriverPath
)

Write-Host "=== Setting up Windows environment for static export ==="

# Find chromedriver path
$chromedriverPath = $ChromeDriverPath
if (-not (Test-Path $chromedriverPath)) {
    Write-Host "Action output chromedriver path not found, searching for alternatives..."

    $commonPaths = @(
        "C:\Program Files\Google\Chrome\Application\chromedriver.exe",
        "C:\Program Files (x86)\Google\Chrome\Application\chromedriver.exe",
        "$env:USERPROFILE\AppData\Local\Google\Chrome\Application\chromedriver.exe",
        "$env:PROGRAMFILES\Google\Chrome\Application\chromedriver.exe",
        "$env:PROGRAMFILES(X86)\Google\Chrome\Application\chromedriver.exe"
    )

    foreach ($path in $commonPaths) {
        if (Test-Path $path) {
            Write-Host "Using chromedriver from: $path"
            $chromedriverPath = $path
            break
        }
    }
}

# Find Chrome path
$chromePath = $ChromePath
if (-not (Test-Path $chromePath)) {
    # Try the tool cache path first
    $toolCachePath = "C:\hostedtoolcache\windows\setup-chrome\chromium\$ChromeVersion\x64\chrome.exe"
    if (Test-Path $toolCachePath) {
        $chromePath = $toolCachePath
        Write-Host "Using Chrome from setup-chrome installation: $chromePath"
    } else {
        # Fallback: search for Chrome in the tool cache
        $toolCacheDir = "C:\hostedtoolcache\windows\setup-chrome\chromium"
        if (Test-Path $toolCacheDir) {
            $chromeExe = Get-ChildItem -Path $toolCacheDir -Recurse -Name "chrome.exe" | Select-Object -First 1
            if ($chromeExe) {
                $chromePath = Join-Path $toolCacheDir $chromeExe
                Write-Host "Using Chrome from tool cache search: $chromePath"
            } else {
                $chromePath = "C:\Program Files\Google\Chrome\Application\chrome.exe"
                Write-Host "Using system Chrome: $chromePath"
            }
        } else {
            $chromePath = "C:\Program Files\Google\Chrome\Application\chrome.exe"
            Write-Host "Using system Chrome: $chromePath"
        }
    }
}

# Set environment variables
$env:WEBDRIVER_PATH = $chromedriverPath
$env:BROWSER_PATH = $chromePath
$env:RUST_LOG = "debug"
$env:RUST_BACKTRACE = "1"
$env:ANGLE_DEFAULT_PLATFORM = "swiftshader"

Write-Host "Environment variables set:"
Write-Host "WEBDRIVER_PATH: $env:WEBDRIVER_PATH"
Write-Host "BROWSER_PATH: $env:BROWSER_PATH"
Write-Host "RUST_LOG: $env:RUST_LOG"
Write-Host "RUST_BACKTRACE: $env:RUST_BACKTRACE"

# Verify paths exist
if (-not (Test-Path $env:WEBDRIVER_PATH)) {
    Write-Error "Chromedriver executable not found at: $env:WEBDRIVER_PATH"
    Write-Host "Available chromedriver locations:"
    Get-ChildItem -Path "C:\Program Files\Google\Chrome\Application\" -Name "chromedriver*" -ErrorAction SilentlyContinue
    Get-ChildItem -Path "C:\Program Files (x86)\Google\Chrome\Application\" -Name "chromedriver*" -ErrorAction SilentlyContinue
    exit 1
}

if (-not (Test-Path $env:BROWSER_PATH)) {
    Write-Error "Chrome not found at: $env:BROWSER_PATH"
    exit 1
}

# Test Chrome version
try {
    $chromeVersion = & "$env:BROWSER_PATH" --version 2>&1
    Write-Host "Chrome version: $chromeVersion"
} catch {
    Write-Host "Failed to get Chrome version: $_"
}

# Test chromedriver version
try {
    $chromedriverVersion = & "$env:WEBDRIVER_PATH" --version 2>&1
    Write-Host "Chromedriver version: $chromedriverVersion"
} catch {
    Write-Host "Failed to get chromedriver version: $_"
}

Write-Host "=== Windows environment setup completed ==="