# scripts/fetch-binaries.ps1
#
# Downloads yt-dlp and ffmpeg into src-tauri/binaries/ with the exact
# filenames Tauri expects (see src-tauri/tauri.conf.json `externalBin`).
#
# Why this script exists:
#   ffmpeg.exe is ~184MB, which exceeds GitHub's 100MB per-file limit.
#   yt-dlp + ffmpeg together are ~200MB, which would bloat every clone.
#   So we gitignore them and fetch on demand.
#
# Usage (from repo root, PowerShell):
#   .\scripts\fetch-binaries.ps1
#
# Then build normally:
#   npm install
#   npm run tauri build

$ErrorActionPreference = 'Stop'

$binDir = Join-Path $PSScriptRoot '..\src-tauri\binaries'
$binDir = (Resolve-Path -LiteralPath $binDir).Path

$ytDlpTarget = Join-Path $binDir 'yt-dlp-x86_64-pc-windows-msvc.exe'
$ffmpegTarget = Join-Path $binDir 'ffmpeg-x86_64-pc-windows-msvc.exe'

Write-Host "Target dir: $binDir" -ForegroundColor Cyan

# --- yt-dlp -----------------------------------------------------------------
# Always pull the latest stable yt-dlp.exe from the official GitHub Releases.
# It's a single ~18MB file, no archive.

if (Test-Path $ytDlpTarget) {
    Write-Host "[skip] yt-dlp already present at $ytDlpTarget" -ForegroundColor Yellow
} else {
    $ytDlpUrl = 'https://github.com/yt-dlp/yt-dlp/releases/latest/download/yt-dlp.exe'
    Write-Host "[get ] yt-dlp -> $ytDlpUrl" -ForegroundColor Green
    Invoke-WebRequest -Uri $ytDlpUrl -OutFile $ytDlpTarget -UseBasicParsing
    Write-Host "[ok  ] $ytDlpTarget" -ForegroundColor Green
}

# --- ffmpeg -----------------------------------------------------------------
# ffmpeg-release-essentials from gyan.dev (most-trusted Windows ffmpeg build).
# Comes as a .zip — we extract just bin/ffmpeg.exe.

if (Test-Path $ffmpegTarget) {
    Write-Host "[skip] ffmpeg already present at $ffmpegTarget" -ForegroundColor Yellow
} else {
    $ffmpegZipUrl = 'https://www.gyan.dev/ffmpeg/builds/ffmpeg-release-essentials.zip'
    $tmpZip = Join-Path $env:TEMP "ffmpeg-essentials-$(Get-Random).zip"
    $tmpExtract = Join-Path $env:TEMP "ffmpeg-essentials-$(Get-Random)"

    Write-Host "[get ] ffmpeg -> $ffmpegZipUrl" -ForegroundColor Green
    Write-Host "       (this is ~80MB compressed, takes a minute)" -ForegroundColor DarkGray
    Invoke-WebRequest -Uri $ffmpegZipUrl -OutFile $tmpZip -UseBasicParsing

    Write-Host "[zip ] extracting ffmpeg.exe..." -ForegroundColor Green
    Expand-Archive -LiteralPath $tmpZip -DestinationPath $tmpExtract -Force

    $ffmpegSrc = Get-ChildItem -Path $tmpExtract -Recurse -Filter 'ffmpeg.exe' | Select-Object -First 1
    if (-not $ffmpegSrc) {
        throw "Could not find ffmpeg.exe inside the downloaded archive."
    }
    Copy-Item -LiteralPath $ffmpegSrc.FullName -Destination $ffmpegTarget -Force
    Write-Host "[ok  ] $ffmpegTarget" -ForegroundColor Green

    Remove-Item -LiteralPath $tmpZip -Force -ErrorAction SilentlyContinue
    Remove-Item -LiteralPath $tmpExtract -Recurse -Force -ErrorAction SilentlyContinue
}

Write-Host ""
Write-Host "Done. Binaries staged. You can now build:" -ForegroundColor Cyan
Write-Host "  npm install"
Write-Host "  npm run tauri build"
