$ErrorActionPreference = 'Stop'
function SelectedPath([string]$p) {
 if ($p.StartsWith('"') -and $p.EndsWith('"')) { $p=$p.Substring(1,$p.Length-2) }
 if ([string]::IsNullOrWhiteSpace($p) -or $p -match '[\x00-\x1f]' -or -not [IO.Path]::IsPathRooted($p)) { throw 'Invalid selected path' }
 $p
}
try {
 $raw=if($env:fs){$env:fs -split '\r?\n'}else{@($env:f)}
 $paths=@($raw | ForEach-Object { SelectedPath $_ })
 if($paths.Count -eq 0){throw 'No selected items'}
 foreach($p in $paths){
  if($p -notmatch '^[A-Za-z]:\\' -or $p -eq [IO.Path]::GetPathRoot($p)){throw 'Recycle requires a local fixed-drive non-root path'}
  $drive=New-Object IO.DriveInfo([IO.Path]::GetPathRoot($p))
  if($drive.DriveType -ne [IO.DriveType]::Fixed){throw 'Recycle unavailable on non-fixed drive'}
  if(!(Test-Path -LiteralPath $p)){throw "Item not found: $p"}
 }
 Write-Host "Move $($paths.Count) item(s) to Recycle Bin:"
 $paths | ForEach-Object {Write-Host "  $_"}
 if((Read-Host 'Confirm [y/N]') -notmatch '^[yY]$'){exit 0}
 Add-Type -Path (Join-Path $PSScriptRoot 'LFRecycle.cs')
 foreach($p in $paths){[LFRecycle]::Move($p); if(Test-Path -LiteralPath $p){throw "Item still exists: $p"}}
 Write-Host 'Recycle operation finished.'
} catch {
 Write-Error $_
 exit 1
}
