# This script takes care of packaging the build artifacts that will go in the
# release zipfile

$SRC_DIR = $PWD.Path
$STAGE = [System.Guid]::NewGuid().ToString()

Set-Location $ENV:Temp
New-Item -Type Directory -Name $STAGE
Set-Location $STAGE

$EXE = "$SRC_DIR\$($Env:CRATE_NAME)-$($Env:APPVEYOR_REPO_TAG_NAME)-$($Env:TARGET).exe"
Copy-Item "$SRC_DIR\target\$($Env:TARGET)\release\emplace.exe" "$EXE"

Push-AppveyorArtifact "$EXE"

Remove-Item *.* -Force
Set-Location ..
Remove-Item $STAGE
Set-Location $SRC_DIR
