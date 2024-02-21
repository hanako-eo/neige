# check if the command exists on the computer
function Test-CommandExist {
    param($command)
    if (Get-Command $command -ErrorAction SilentlyContinue) {
        return $true
    } else {
        return $false
    }
}

if (-not (Test-CommandExist "cargo")) {
    Write-Host "The project needs rust (and cargo more specifically), please install it!"
    exit 1
}

# install cargo watch
& cargo install cargo-watch
& npm i
