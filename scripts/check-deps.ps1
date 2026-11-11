# pcpulse Phase-0 dependency check. Read-only: inspects only, installs nothing.
# Usage: powershell -NoLogo -File scripts\check-deps.ps1
$script = $0

function Report($name, $found, $detail, $fix) {
    if ($found) {
        Write-Output "[PASS] $name - $detail"
    }
    else {
        Write-Output "[MISS] $name - $detail"
        Write-Output "       fix: $fix"
    }
}

Write-Output "=== pcpulse toolchain check ==="

# 1+2) rustc / cargo
$rustcOut = & rustc --version 2>$null
$rustcVer = if ($rustcOut) { (("$rustcOut" -split ' ')[1]) } else { "not found" }
Report "rustc" ($null -ne $rustcOut) $rustcVer "install Rust via rustup (system-wide toolchain - ask first): https://www.rust-lang.org/tools/install"

$cargoOut = & cargo --version 2>$null
$cargoVer = if ($cargoOut) { (("$cargoOut" -split ' ')[1]) } else { "not found" }
Report "cargo" ($null -ne $cargoOut) $cargoVer "ships with rustc (same rustup install)"

# 3) MSVC linker (default target is x86_64-pc-windows-msvc)
$msvcDirs = @(
    "C:\Program Files\Microsoft Visual Studio",
    "C:\Program Files (x86)\Microsoft Visual Studio"
)
$links = foreach ($d in $msvcDirs) {
    if (Test-Path -LiteralPath $d) {
        Get-ChildItem -Path $d -Recurse -Depth 10 -Filter link.exe -ErrorAction SilentlyContinue |
            Where-Object { $_.FullName -notmatch '\\Shared\\' }
    }
}
$links = @($links | Where-Object { $_ })
$mlink = if ($links.Count -gt 0) { $links[0].FullName } else { "not found" }
Report "MSVC link.exe" ($links.Count -gt 0) $mlink "install VS 2022 Build Tools (free): https://devblogs.microsoft.com/commandline/getting-started-with-the-visual-studio-2022-build-tools/ or via https://learn.microsoft.com/cpp/build/building-on-the-command-line"

# 4) Inno Setup compiler (any recent version)
$inno = Get-Item "C:\Program Files (x86)\Inno Setup *\ISCC.exe", "C:\Program Files\Inno Setup *\ISCC.exe", "$env:LOCALAPPDATA\Programs\Inno Setup *\ISCC.exe" -ErrorAction SilentlyContinue
$inno = @($inno | Where-Object { $_ }) | Select-Object -First 1
$innoWhere = if (@($inno).Count -gt 0) { (@($inno) | Select-Object -First 1).FullName } else { "iscc.exe not found" }
Report "Inno Setup" (@($inno).Count -gt 0) $innoWhere "install Inno Setup from https://jrsoftware.org/isdl.php (needs my approval)"

# 5) git (read-only use only)
$gitCmd = Get-Command git.exe -ErrorAction SilentlyContinue
$gitWhere = if ($gitCmd) { $gitCmd.Source } else { "not found" }
Report "git" ($null -ne $gitCmd) $gitWhere "only needed to read repo history; never required to build"

Write-Output "=== done (nothing installed) ==="
