param(
	[switch]$clear
)

function new-tempdir{
    $parent = [System.IO.Path]::GetTempPath()
    [string] $name = [System.Guid]::NewGuid()
    New-Item -ItemType Directory -Path (Join-Path $parent $name)
}

echo "building everything in release mode"
$dir=new-tempdir
cargo build --release --bins --target-dir $dir

if($lastExitCode) {
	return "could not compile the project"
}

$bins= get-childitem "$dir\release\day*.exe"

$t= measure-command {
	$bins | % { & $_ }
}

echo "note: benchmarks on wsl perform many times better, this may have to do with the way powershell's measure-command measures time" 1>&2

if($clear) {
	remove-item -recurse -force $dir 2>&1 | out-null
}

$t
