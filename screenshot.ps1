Add-Type -AssemblyName System.Drawing

Add-Type -TypeDefinition @"
using System;
using System.Collections.Generic;
using System.Runtime.InteropServices;
using System.Text;
public class W {
    public delegate bool EWP(IntPtr h, IntPtr l);
    [DllImport("user32.dll")] public static extern bool EnumWindows(EWP fn, IntPtr l);
    [DllImport("user32.dll")] public static extern bool IsWindowVisible(IntPtr h);
    [DllImport("user32.dll")] public static extern bool IsIconic(IntPtr h);
    [DllImport("user32.dll")] public static extern bool GetWindowRect(IntPtr h, out R r);
    [DllImport("user32.dll")] public static extern bool ShowWindow(IntPtr h, int n);
    [DllImport("user32.dll")] public static extern bool SetForegroundWindow(IntPtr h);
    [DllImport("user32.dll", CharSet=CharSet.Unicode)] public static extern int GetClassName(IntPtr h, StringBuilder s, int n);
    [DllImport("user32.dll", CharSet=CharSet.Unicode)] public static extern int GetWindowTextW(IntPtr h, StringBuilder s, int n);
    [DllImport("dwmapi.dll")] public static extern int DwmGetWindowAttribute(IntPtr h, int a, out R r, int sz);
    [StructLayout(LayoutKind.Sequential)] public struct R { public int L,T,Ri,B; }
}
"@

$found = [IntPtr]::Zero
$cb = [W+EWP]{
    param($h, $l)
    if (-not [W]::IsWindowVisible($h)) { return $true }
    $cs = New-Object System.Text.StringBuilder 256
    [W]::GetClassName($h, $cs, 256) | Out-Null
    if ($cs.ToString() -eq 'Tauri Window') {
        $ts = New-Object System.Text.StringBuilder 256
        [W]::GetWindowTextW($h, $ts, 256) | Out-Null
        if ($ts.ToString() -match 'Video Lab' -or $script:found -eq [IntPtr]::Zero) {
            $script:found = $h
        }
    }
    return $true
}
[W]::EnumWindows($cb, [IntPtr]::Zero) | Out-Null

if ($found -eq [IntPtr]::Zero) {
    Write-Error "No 'Tauri Window' found"
    exit 1
}

# Restore if minimized
if ([W]::IsIconic($found)) {
    Write-Host "Window minimized, restoring..."
    [W]::ShowWindow($found, 9) | Out-Null  # SW_RESTORE
    Start-Sleep -Milliseconds 600
}
[W]::ShowWindow($found, 1) | Out-Null      # SW_SHOWNORMAL
[W]::SetForegroundWindow($found) | Out-Null
Start-Sleep -Milliseconds 800

$r = New-Object W+R
$sz = [System.Runtime.InteropServices.Marshal]::SizeOf([type][W+R])
$hr = [W]::DwmGetWindowAttribute($found, 9, [ref]$r, $sz)
if ($hr -ne 0) { [W]::GetWindowRect($found, [ref]$r) | Out-Null }

$w = $r.Ri - $r.L
$h = $r.B - $r.T
Write-Host "Bounds: $w by $h"

if ($w -lt 200 -or $h -lt 200) {
    Write-Error "Window still too small after restore: $w by $h"
    exit 1
}

$bmp = New-Object System.Drawing.Bitmap $w, $h
$g = [System.Drawing.Graphics]::FromImage($bmp)
$g.CopyFromScreen($r.L, $r.T, 0, 0, $bmp.Size)
$out = Join-Path $PSScriptRoot "ui-screenshot.png"
$bmp.Save($out, [System.Drawing.Imaging.ImageFormat]::Png)
$g.Dispose(); $bmp.Dispose()
Write-Host "Saved to $out"
