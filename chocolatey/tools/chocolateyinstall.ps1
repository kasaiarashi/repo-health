$ErrorActionPreference = 'Stop'

$packageName = 'repo-health'
$toolsDir = "$(Split-Path -parent $MyInvocation.MyCommand.Definition)"
$url64 = 'https://github.com/kasaiarashi/repo-health/releases/download/v0.1.0/repo-health-windows-x64.exe'

$packageArgs = @{
  packageName   = $packageName
  fileType      = 'exe'
  url64bit      = $url64
  softwareName  = 'repo-health*'
  checksum64    = ''
  checksumType64= 'sha256'
  silentArgs    = ''
  validExitCodes= @(0)
  destination   = $toolsDir
}

Get-ChocolateyWebFile @packageArgs

# Rename to repo-health.exe
Rename-Item -Path "$toolsDir\repo-health-windows-x64.exe" -NewName "repo-health.exe" -Force
