# ZON Ecosystem Launch Script (zon-lib)
# publishes zon-lib, zon-inspector, and builds zon-wasm for NPM

Write-Host "========================================" -ForegroundColor Cyan
Write-Host "    ZON-LIB ECOSYSTEM LAUNCH SEQUENCE" -ForegroundColor Cyan
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""

# stage 1: publish zon-lib
Write-Host "[STAGE 1] Launching ZON-LIB..." -ForegroundColor Yellow
cargo publish -p zon-lib

if ($LASTEXITCODE -ne 0) {
    Write-Host "[ERROR] Failed to publish zon-lib" -ForegroundColor Red
    exit 1
}
Write-Host "[SUCCESS] zon-lib published!" -ForegroundColor Green
Write-Host ""

# the wait: allow crates.io index to update
Write-Host "[WAITING] Allowing 5 minutes for crates.io index to propagate..." -ForegroundColor Magenta
Write-Host "          This ensures zon-inspector can find zon-lib on the registry." -ForegroundColor Gray
for ($i = 300; $i -gt 0; $i -= 30) {
    Write-Host "          $i seconds remaining..." -ForegroundColor Gray
    Start-Sleep -Seconds 30
}
Write-Host "[READY] Index should be updated." -ForegroundColor Green
Write-Host ""

# stage 2: publish zon-inspector
Write-Host "[STAGE 2] Launching INSPECTOR..." -ForegroundColor Yellow
cargo publish -p zon-inspector

if ($LASTEXITCODE -ne 0) {
    Write-Host "[ERROR] Failed to publish zon-inspector" -ForegroundColor Red
    exit 1
}
Write-Host "[SUCCESS] zon-inspector published!" -ForegroundColor Green
Write-Host ""

# stage 3: build zon-wasm for NPM
Write-Host "[STAGE 3] Building WASM Bridge..." -ForegroundColor Yellow
wasm-pack build crates/zon-wasm --target nodejs --scope @zaim-abbasi

if ($LASTEXITCODE -ne 0) {
    Write-Host "[ERROR] Failed to build zon-wasm" -ForegroundColor Red
    exit 1
}
Write-Host "[SUCCESS] zon-wasm built!" -ForegroundColor Green
Write-Host ""

# final instructions
Write-Host "========================================" -ForegroundColor Cyan
Write-Host "    SUCCESS! ECOSYSTEM IS LIVE!" -ForegroundColor Green
Write-Host "========================================" -ForegroundColor Cyan
Write-Host ""
Write-Host "Final step to publish NPM package:" -ForegroundColor Yellow
Write-Host "  cd crates/zon-wasm/pkg" -ForegroundColor White
Write-Host "  npm publish --access public" -ForegroundColor White
Write-Host ""
Write-Host "Your packages are now available:" -ForegroundColor Cyan
Write-Host "  - https://crates.io/crates/zon-lib" -ForegroundColor Gray
Write-Host "  - https://crates.io/crates/zon-inspector" -ForegroundColor Gray
Write-Host "  - https://www.npmjs.com/package/@zaim-abbasi/zon-wasm (after npm publish)" -ForegroundColor Gray
