$ErrorActionPreference = "Stop"

$Root = Split-Path -Parent $PSScriptRoot
$ShaderDir = Join-Path $Root "shaders/src"
$OutDir = Join-Path $Root "shaders/compiled"

New-Item -ItemType Directory -Force -Path $OutDir | Out-Null

glslc `
    (Join-Path $ShaderDir "shader.vert") `
    -o (Join-Path $OutDir "vert.spv")

glslc `
    (Join-Path $ShaderDir "shader.frag") `
    -o (Join-Path $OutDir "frag.spv")

glslc `
    (Join-Path $ShaderDir "debug_line.vert") `
    -o (Join-Path $OutDir "debug_line_vert.spv")

glslc `
    (Join-Path $ShaderDir "debug_line.frag") `
    -o (Join-Path $OutDir "debug_line_frag.spv")

Write-Host "Shaders compiled to $OutDir"
