$global:adb = ".\platform-tools-latest-windows\platform-tools\adb.exe"
$global:serial = ".\installer-windows-x86_64\installer.exe"

function _adb_push {
    & $global:adb -d push @args *> $null
    $exitCode = $LASTEXITCODE	
	return $exitCode
}

function _adb_shell {
    & $global:adb -d shell @args *> $null
    $exitCode = $LASTEXITCODE	
	return $exitCode
}

function _wait_for_adb_shell {
	do {
		start-sleep -seconds 1
		$success = _adb_shell "uname -a"
	} until ($success -eq 0)
}

function _wait_for_atfwd_daemon {
	do {
		start-sleep -seconds 1
		$success = _adb_shell "pgrep atfwd_daemon"
	} until ($success -eq 0)
}

function force_debug_mode {
	write-host "Using adb at $($global:adb)"
	write-host "Forcing a switch into debug mode to enable ADB"
	_serial "--root" | Out-Host
	write-host "adb enabled, waiting for reboot..." -nonewline
	_wait_for_adb_shell
	write-host " it's alive!"
	write-host "waiting for atfwd_daemon to start ..." -nonewline
	_wait_for_atfwd_daemon
	write-host " done!"
}
function _serial {
    param (
        [Parameter(Mandatory = $false, ValueFromRemainingArguments = $true)]
        [string[]]$Args
    )

    # Build the full argument list
    $allArgs = @("util", "serial") + $Args

    # Call the serial executable
    & $global:serial @allArgs
}

function setup_rootshell {
	write-host "setting up rootshell"
	_adb_push "rootshell" "/tmp" | Out-null
	write-host "cp..."
	_serial "AT+SYSCMD=cp /tmp/rootshell /bin/rootshell" | Out-Host
	start-sleep -seconds 1
	write-host "chown..."
	_serial "AT+SYSCMD=chown root /bin/rootshell" | Out-Host
	start-sleep -seconds 1
	write-host "chmod..."
	_serial "AT+SYSCMD=chmod 4755 /bin/rootshell" | Out-Host
	start-sleep -seconds 1
	_adb_shell '/bin/rootshell -c id' | Out-null
	write-host "we have root!"
}

function setup_rayhunter {
	write-host "installing rayhunter..."
	_serial "AT+SYSCMD=mkdir -p /data/rayhunter" | Out-Host
	_adb_push "config.toml.in" "/tmp/config.toml" | Out-Null
	_serial "AT+SYSCMD=mv /tmp/config.toml /data/rayhunter" | Out-Host
	_adb_push "rayhunter-daemon-orbic/rayhunter-daemon" "/tmp/rayhunter-daemon" | Out-Null
	_serial "AT+SYSCMD=mv /tmp/rayhunter-daemon /data/rayhunter" | Out-Host
	_adb_push "scripts/rayhunter_daemon" "/tmp/rayhunter_daemon" | Out-Null
	_serial "AT+SYSCMD=mv /tmp/rayhunter_daemon /etc/init.d/rayhunter_daemon" | Out-Host
	_adb_push "scripts/misc-daemon" "/tmp/misc-daemon" | Out-Null
	_serial "AT+SYSCMD=mv /tmp/misc-daemon /etc/init.d/misc-daemon" | Out-Host

	_serial "AT+SYSCMD=chmod 755 /data/rayhunter/rayhunter-daemon" | Out-Host
	_serial "AT+SYSCMD=chmod 755 /etc/init.d/rayhunter_daemon" | Out-Host
	_serial "AT+SYSCMD=chmod 755 /etc/init.d/misc-daemon" | Out-Host

	write-host "waiting for reboot..."
	_serial "AT+SYSCMD=shutdown -r -t 1 now" | Out-Host
	do {
		start-sleep -seconds 1
	} until ((_adb_shell "true 2> /dev/null") -ne 0)

	_wait_for_adb_shell
	write-host "done!"
}

function test_rayhunter {
	$URL = "http://localhost:8080/index.html"
	& $global:adb -d forward tcp:8080 tcp:8080
    $exitCode = $LASTEXITCODE
	if ($exitCode -ne 0) {
		write-host "adb forward tcp:8080 tcp:8080 failed with exit code $($exitCode)"
		return
	}
	write-host "checking for rayhunter server..." -nonewline
	$seconds = 0
	do {
		try {
			$resp = invoke-webrequest -uri $URL
		} catch {
			# Fail silently
			$resp = $null			
		}
		if ($resp.statuscode -eq 200) {
			write-host "success!"
			write-host "you can access rayhunter at $($URL)"
			return
		}
		start-sleep 1
		$seconds = $seconds + 1
	} until ($seconds -eq 30)
	write-host "timeout reached! failed to reach rayhunter url $($URL), something went wrong :("
}

function get_android_tools {
	write-host "adb not found, downloading local copy"
	invoke-webrequest "https://dl.google.com/android/repository/platform-tools-latest-windows.zip" -outfile ./platform-tools-latest-windows.zip
	expand-archive -force -path "platform-tools-latest-windows.zip"
}

if (-not (test-path -path $global:serial)) {
	write-error "can't find serial, aborting"
	return
}

if (-not (test-path -path $global:adb)) {
	get_android_tools
}

force_debug_mode
setup_rootshell
setup_rayhunter
test_rayhunter