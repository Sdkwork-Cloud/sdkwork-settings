# SDKWork Settings 验证脚本索引
# 运行所有标准验证命令,确认项目对齐 sdkwork-specs

Write-Host "=== SDKWork Settings 标准验证 ===" -ForegroundColor Cyan

# Rust 验证
Write-Host "`n[1/5] cargo check --workspace" -ForegroundColor Yellow
cargo check --workspace
if ($LASTEXITCODE -ne 0) { Write-Host "FAILED" -ForegroundColor Red; exit 1 }

Write-Host "`n[2/5] cargo clippy --workspace -- -D warnings" -ForegroundColor Yellow
cargo clippy --workspace -- -D warnings
if ($LASTEXITCODE -ne 0) { Write-Host "FAILED" -ForegroundColor Red; exit 1 }

Write-Host "`n[3/5] cargo test --workspace" -ForegroundColor Yellow
cargo test --workspace
if ($LASTEXITCODE -ne 0) { Write-Host "FAILED" -ForegroundColor Red; exit 1 }

Write-Host "`n[4/5] cargo fmt --all -- --check" -ForegroundColor Yellow
cargo fmt --all -- --check
if ($LASTEXITCODE -ne 0) { Write-Host "FAILED" -ForegroundColor Red; exit 1 }

# pnpm 验证(如果 node_modules 存在)
if (Test-Path "node_modules") {
    Write-Host "`n[5/5] pnpm check:pnpm-script-standard" -ForegroundColor Yellow
    pnpm check:pnpm-script-standard
    if ($LASTEXITCODE -ne 0) { Write-Host "FAILED" -ForegroundColor Red; exit 1 }
} else {
    Write-Host "`n[5/5] pnpm 验证跳过(node_modules 不存在)" -ForegroundColor Gray
}

Write-Host "`n=== 所有验证通过 ===" -ForegroundColor Green
