# Native Windows build: Go 1.26+, Rust/Cargo 1.94+ MSVC toolchain.
$ErrorActionPreference = 'Stop'
$Root = (Resolve-Path (Join-Path $PSScriptRoot '..')).Path
$Out = Join-Path $Root 'dist\NeoFM-win-x64'
$Target = Join-Path $Root 'target'
New-Item -ItemType Directory -Force $Out, "$Out\samples" | Out-Null
Push-Location (Join-Path $Root 'lf-upstream')
try {
    $env:CGO_ENABLED = '0'
    go build -trimpath -buildvcs=false -ldflags '-s -w' -o "$Out\lf.exe" .
    if ($LASTEXITCODE -ne 0) { throw 'lf build failed' }
} finally { Pop-Location }
$env:CARGO_TARGET_DIR = $Target
cargo build --release --locked --manifest-path "$Root\lf-integration-poc\adapter\Cargo.toml"
if ($LASTEXITCODE -ne 0) { throw 'Renderer build failed' }
Copy-Item "$Target\release\lfpreview.exe" $Out -Force
Copy-Item "$Root\platform\windows\*" $Out -Force
Copy-Item "$Root\lf-integration-poc\fixtures-*" "$Out\samples" -Recurse -Force
Copy-Item "$Root\LICENSE" "$Out\LICENSE-Codex" -Force
Copy-Item "$Root\lf-upstream\LICENSE" "$Out\LICENSE-lf" -Force
Copy-Item "$Root\NOTICE","$Root\VERSION" $Out -Force
Write-Host "NeoFM v$(Get-Content "$Root\VERSION"): $Out"
& "$Out\neofm.cmd" -version
if ($LASTEXITCODE -ne 0) { throw 'Launcher smoke failed' }
