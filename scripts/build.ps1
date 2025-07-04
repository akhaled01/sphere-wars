#!/usr/bin/env pwsh

$env:RUST_LOG = "info"

Remove-Item -Path "./bin" -Recurse -Force -ErrorAction SilentlyContinue

function Get-Font {
    if (-not (Test-Path "./bin/assets/SpaceMono-Regular.ttf")) {
        New-Item -ItemType Directory -Path "./bin/assets" -Force | Out-Null
        Write-Host "Downloading font..."
        Invoke-WebRequest -Uri "https://raw.githubusercontent.com/google/fonts/refs/heads/main/ofl/spacemono/SpaceMono-Regular.ttf" -OutFile "./bin/assets/SpaceMono-Regular.ttf" -UseBasicParsing
    }
}

Get-Font

cargo fmt

Write-Host "Building server and client in parallel..."

# Build both binaries in parallel using PowerShell jobs
$serverJob = Start-Job -ScriptBlock { cargo build --bin server --release }
$clientJob = Start-Job -ScriptBlock { cargo build --bin client --release }

# Wait for both builds to complete
Wait-Job $serverJob, $clientJob

# Get job results and remove jobs
Receive-Job $serverJob
Receive-Job $clientJob
Remove-Job $serverJob, $clientJob

New-Item -ItemType Directory -Path "bin" -Force | Out-Null

# Copy binaries (handle Windows .exe extension)
if ($IsWindows) {
    Copy-Item "target/release/server.exe" "bin/server.exe"
    Copy-Item "target/release/client.exe" "bin/client.exe"
    
    # Create batch file for Windows aliases
    @"
@echo off
doskey war-server=.\bin\server.exe `$*
doskey war-client=.\bin\client.exe `$*
"@ | Out-File -FilePath ".aliases.bat" -Encoding ASCII
    
    Write-Host "Build complete!"
    Write-Host "To use aliases on Windows, run: .aliases.bat"
    Write-Host "Then you can use 'war-server' and 'war-client' commands"
} else {
    Copy-Item "target/release/server" "bin/server"
    Copy-Item "target/release/client" "bin/client"
    
    # Create aliases file for Unix-like systems
    @"
alias war-server="./bin/server"
alias war-client="./bin/client"
"@ | Out-File -FilePath ".aliases" -Encoding UTF8
    
    Write-Host "Build complete!"
    Write-Host "To use aliases, run: source .aliases"
    Write-Host "Then you can use 'war-server' and 'war-client' commands"
}
