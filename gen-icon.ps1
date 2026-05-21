Add-Type -AssemblyName System.Drawing

$size = 1024
$bitmap = New-Object System.Drawing.Bitmap $size, $size
$g = [System.Drawing.Graphics]::FromImage($bitmap)
$g.SmoothingMode = [System.Drawing.Drawing2D.SmoothingMode]::AntiAlias
$g.TextRenderingHint = [System.Drawing.Text.TextRenderingHint]::AntiAliasGridFit
$g.Clear([System.Drawing.Color]::Transparent)

# Rounded square background
$radius = 200
$margin = 24
$x = $margin
$y = $margin
$w = $size - 2 * $margin
$h = $size - 2 * $margin

$path = New-Object System.Drawing.Drawing2D.GraphicsPath
$path.AddArc($x, $y, $radius, $radius, 180, 90)
$path.AddArc($x + $w - $radius, $y, $radius, $radius, 270, 90)
$path.AddArc($x + $w - $radius, $y + $h - $radius, $radius, $radius, 0, 90)
$path.AddArc($x, $y + $h - $radius, $radius, $radius, 90, 90)
$path.CloseFigure()

# Dark fill #0A0B0D
$bgBrush = New-Object System.Drawing.SolidBrush ([System.Drawing.Color]::FromArgb(255, 10, 11, 13))
$g.FillPath($bgBrush, $path)

# Neon green stroke around the rounded square for a glow-edge feel
$strokeColor = [System.Drawing.Color]::FromArgb(70, 64, 255, 120)
$pen = New-Object System.Drawing.Pen $strokeColor, 4
$g.DrawPath($pen, $path)

# DA text — neon green
$fontFamily = $null
foreach ($candidate in @('Segoe UI Black', 'Arial Black', 'Impact')) {
    try {
        $fontFamily = New-Object System.Drawing.FontFamily $candidate
        break
    } catch {}
}
if ($null -eq $fontFamily) { $fontFamily = New-Object System.Drawing.FontFamily 'Arial' }

$font = New-Object System.Drawing.Font $fontFamily, 440, ([System.Drawing.FontStyle]::Bold), ([System.Drawing.GraphicsUnit]::Pixel)
$textBrush = New-Object System.Drawing.SolidBrush ([System.Drawing.Color]::FromArgb(255, 64, 255, 120))

$sf = New-Object System.Drawing.StringFormat
$sf.Alignment = [System.Drawing.StringAlignment]::Center
$sf.LineAlignment = [System.Drawing.StringAlignment]::Center
$rect = New-Object System.Drawing.RectangleF 0, -30, $size, $size
$g.DrawString('DA', $font, $textBrush, $rect, $sf)

# Small gold accent dot under the DA text
$accentBrush = New-Object System.Drawing.SolidBrush ([System.Drawing.Color]::FromArgb(255, 255, 219, 64))
$dotSize = 36
$g.FillEllipse($accentBrush, ($size - $dotSize) / 2, 760, $dotSize, $dotSize)

$outFile = Join-Path $PSScriptRoot 'app-icon.png'
$bitmap.Save($outFile, [System.Drawing.Imaging.ImageFormat]::Png)
$g.Dispose()
$bitmap.Dispose()

Write-Host "Icon saved to $outFile"
