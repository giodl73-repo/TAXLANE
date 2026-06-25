# Build the accepted TAXLANE research papers from markdown to PDF.
#
# Requires: pandoc + a LaTeX engine (MiKTeX xelatex). Run from the repo root:
#   pwsh docs/papers/build.ps1
#
# The markdown tier (research/publications/<slug>/paper.md) is the source of truth;
# these PDFs are a generated rendering. Unicode symbols (-> x ~ >= != etc.) require
# a Unicode-capable mainfont, so we use Cambria/Consolas via xelatex.

$ErrorActionPreference = "Stop"
$root = Split-Path -Parent (Split-Path -Parent $PSScriptRoot)  # repo root (docs/papers/ -> repo)
$pub  = Join-Path $root "research\publications"
$out  = $PSScriptRoot

# Accepted papers in reading order: synthesis first, then tracks T1-T5.
$papers = @(
    @{ n = 0; slug = "legible-federal-funding" }
    @{ n = 1; slug = "health-funding-premium" }
    @{ n = 2; slug = "old-age-tax-and-the-cap" }
    @{ n = 3; slug = "defense-tax-in-allied-perspective" }
    @{ n = 4; slug = "the-thin-safety-net" }
    @{ n = 5; slug = "low-tax-country-borrowing-habit" }
)

$fmt = "markdown-tex_math_dollars-tex_math_single_backslash-raw_tex"

foreach ($p in $papers) {
    $src = Join-Path $pub "$($p.slug)\paper.md"
    $dst = Join-Path $out "$($p.n)+$($p.slug).pdf"
    Write-Host "Building $($p.n)+$($p.slug).pdf ..."
    pandoc $src -o $dst `
        --pdf-engine=xelatex `
        -f $fmt `
        -V geometry:margin=1in `
        -V fontsize=11pt `
        -V colorlinks=true `
        -V linkcolor=blue `
        -V mainfont="Cambria" `
        -V monofont="Consolas"
    if (-not (Test-Path $dst)) { throw "FAILED to build $dst" }
}

Write-Host "Done. Built $($papers.Count) PDFs into docs/papers/."
