param(
    [string]$OutputDirectory = "dist/rev-level-7-submission"
)

$ErrorActionPreference = "Stop"
$repoRoot = [System.IO.Path]::GetFullPath((Join-Path $PSScriptRoot ".."))
$controlRelative = "data/derived/breadth_benchmark_matrix/rev_level_7_external_submission_control.v1.draft.json"
$controlPath = [System.IO.Path]::GetFullPath((Join-Path $repoRoot $controlRelative))
$outputRoot = [System.IO.Path]::GetFullPath((Join-Path $repoRoot $OutputDirectory))
$distRoot = [System.IO.Path]::GetFullPath((Join-Path $repoRoot "dist"))

if (-not $outputRoot.StartsWith($distRoot + [System.IO.Path]::DirectorySeparatorChar, [System.StringComparison]::OrdinalIgnoreCase)) {
    throw "OutputDirectory must resolve beneath the repository dist directory"
}

$control = Get-Content -Raw -LiteralPath $controlPath | ConvertFrom-Json
if ($control.authorization.exact_outbound_action_authorized -or $control.preflight.send_allowed) {
    throw "Bundle builder refuses a control record that enables outbound action"
}

$seenBundlePaths = @{}
$orderedDigestLines = [System.Collections.Generic.List[string]]::new()
foreach ($item in $control.payload) {
    if ($item.bundle_path -match '(^|[\\/])\.\.([\\/]|$)' -or [System.IO.Path]::IsPathRooted($item.bundle_path)) {
        throw "Unsafe bundle path: $($item.bundle_path)"
    }
    if ($seenBundlePaths.ContainsKey($item.bundle_path)) {
        throw "Duplicate bundle path: $($item.bundle_path)"
    }
    $seenBundlePaths[$item.bundle_path] = $true

    $sourcePath = [System.IO.Path]::GetFullPath((Join-Path $repoRoot $item.path))
    if (-not $sourcePath.StartsWith($repoRoot + [System.IO.Path]::DirectorySeparatorChar, [System.StringComparison]::OrdinalIgnoreCase)) {
        throw "Payload path escapes repository: $($item.path)"
    }
    if (-not (Test-Path -LiteralPath $sourcePath -PathType Leaf)) {
        throw "Missing payload file: $($item.path)"
    }
    $observedHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $sourcePath).Hash.ToLowerInvariant()
    if ($observedHash -ne $item.sha256) {
        throw "Payload hash mismatch for $($item.path): expected $($item.sha256), observed $observedHash"
    }
    $orderedDigestLines.Add("$($item.bundle_path):$observedHash")
}

$bundleIdentityBytes = [System.Text.Encoding]::UTF8.GetBytes(($orderedDigestLines -join "`n") + "`n")
$sha256 = [System.Security.Cryptography.SHA256]::Create()
try {
    $observedBundleId = [System.Convert]::ToHexString($sha256.ComputeHash($bundleIdentityBytes)).ToLowerInvariant()
} finally {
    $sha256.Dispose()
}
if ($observedBundleId -ne $control.bundle_id) {
    throw "Bundle ID mismatch: expected $($control.bundle_id), observed $observedBundleId"
}

[System.IO.Directory]::CreateDirectory($outputRoot) | Out-Null
$zipPath = Join-Path $outputRoot "$($control.submission_id).zip"
$hashPath = "$zipPath.sha256"
foreach ($target in @($zipPath, $hashPath)) {
    $resolvedTarget = [System.IO.Path]::GetFullPath($target)
    if (-not $resolvedTarget.StartsWith($outputRoot + [System.IO.Path]::DirectorySeparatorChar, [System.StringComparison]::OrdinalIgnoreCase)) {
        throw "Refusing to replace output outside resolved output directory: $resolvedTarget"
    }
    if (Test-Path -LiteralPath $resolvedTarget) {
        Remove-Item -LiteralPath $resolvedTarget -Force
    }
}

Add-Type -AssemblyName System.IO.Compression
$stream = [System.IO.File]::Open($zipPath, [System.IO.FileMode]::CreateNew)
try {
    $archive = [System.IO.Compression.ZipArchive]::new($stream, [System.IO.Compression.ZipArchiveMode]::Create, $false)
    try {
        foreach ($item in $control.payload) {
            $sourcePath = [System.IO.Path]::GetFullPath((Join-Path $repoRoot $item.path))
            $entry = $archive.CreateEntry($item.bundle_path, [System.IO.Compression.CompressionLevel]::NoCompression)
            $entry.LastWriteTime = [System.DateTimeOffset]::Parse("1980-01-01T00:00:00Z")
            $entryStream = $entry.Open()
            try {
                $bytes = [System.IO.File]::ReadAllBytes($sourcePath)
                $entryStream.Write($bytes, 0, $bytes.Length)
            } finally {
                $entryStream.Dispose()
            }
        }
        $manifestEntry = $archive.CreateEntry("manifest/submission-control.json", [System.IO.Compression.CompressionLevel]::NoCompression)
        $manifestEntry.LastWriteTime = [System.DateTimeOffset]::Parse("1980-01-01T00:00:00Z")
        $manifestStream = $manifestEntry.Open()
        try {
            $manifestBytes = [System.IO.File]::ReadAllBytes($controlPath)
            $manifestStream.Write($manifestBytes, 0, $manifestBytes.Length)
        } finally {
            $manifestStream.Dispose()
        }
    } finally {
        $archive.Dispose()
    }
} finally {
    $stream.Dispose()
}

$zipHash = (Get-FileHash -Algorithm SHA256 -LiteralPath $zipPath).Hash.ToLowerInvariant()
[System.IO.File]::WriteAllText($hashPath, "$zipHash  $([System.IO.Path]::GetFileName($zipPath))`n", [System.Text.UTF8Encoding]::new($false))
[pscustomobject]@{
    submission_id = $control.submission_id
    bundle_id = $control.bundle_id
    zip_path = $zipPath
    zip_sha256 = $zipHash
    payload_files = $control.payload.Count
    outbound_action_performed = $false
} | ConvertTo-Json
