$global:adb = "./platform-tools-latest-windows/platform-tools/adb.exe"
$global:serial = "./serial-windows-x86_64/serial.exe"

function _adb_push {
	$proc = start-process -passthru -wait $global:adb -argumentlist "push", $args[0], $args[1]
	if ($proc.exitcode -ne 0) {
		write-host "push exited with exit code $($proc.exitcode)"
	}
	return $proc.exitcode
}

function _adb_shell {
	$proc = start-process -passthru -wait $global:adb -argumentlist "shell", $args[0]
	if ($proc.exitcode -ne 0) {
		write-host "shell exited with exit code $($proc.exitcode)"
	}
	return $proc.exitcode
}

function _wait_for_adb_shell {
	do {
		start-sleep -seconds 1
	} until ((_adb_shell "cat /etc/ver.conf") -eq 0)
}

function _wait_for_atfwd_daemon {
	do {
		start-sleep -seconds 1
	} until ((_adb_shell "pgrep atfwd_daemon") -eq 0)
}

function force_debug_mode {
	write-host "Using adb at $($global:adb)"
	write-host "Forcing a switch into debug mode to enable ADB"
	&$global:serial "--root" | Out-Host
	write-host "adb enabled, waiting for reboot..." -nonewline
	_wait_for_adb_shell
	write-host " it's alive!"
	write-host "waiting for atfwd_daemon to start ..." -nonewline
	_wait_for_atfwd_daemon
	write-host " done!"
}

function setup_rootshell {
	_adb_push "rootshell" "/tmp"
	write-host "cp..."
	&$global:serial "AT+SYSCMD=cp /tmp/rootshell /bin/rootshell" | Out-Host
	start-sleep -seconds 1
	write-host "chown..."
	&$global:serial "AT+SYSCMD=chown root /bin/rootshell" | Out-Host
	start-sleep -seconds 1
	write-host "chmod..."
	&$global:serial "AT+SYSCMD=chmod 4755 /bin/rootshell" | Out-Host
	start-sleep -seconds 1
	_adb_shell '/bin/rootshell -c id'
	write-host "we have root!"
}

function setup_rayhunter {
	&$global:serial "AT+SYSCMD=mkdir -p /data/rayhunter" | Out-Host
	_adb_push "config.toml.example" "/tmp/config.toml"
	&$global:serial "AT+SYSCMD=mv /tmp/config.toml /data/rayhunter" | Out-Host
	_adb_push "rayhunter-daemon-orbic/rayhunter-daemon" "/tmp/rayhunter-daemon"
	&$global:serial "AT+SYSCMD=mv /tmp/rayhunter-daemon /data/rayhunter" | Out-Host
	_adb_push "scripts/rayhunter_daemon" "/tmp/rayhunter_daemon"
	&$global:serial "AT+SYSCMD=mv /tmp/rayhunter_daemon /etc/init.d/rayhunter_daemon" | Out-Host
	_adb_push "scripts/misc-daemon" "/tmp/misc-daemon"
	&$global:serial "AT+SYSCMD=mv /tmp/misc-daemon /etc/init.d/misc-daemon" | Out-Host

	&$global:serial "AT+SYSCMD=chmod 755 /data/rayhunter/rayhunter-daemon" | Out-Host
	&$global:serial "AT+SYSCMD=chmod 755 /etc/init.d/rayhunter_daemon" | Out-Host
	&$global:serial "AT+SYSCMD=chmod 755 /etc/init.d/misc-daemon" | Out-Host

	write-host "waiting for reboot..."
	&$global:serial "AT+SYSCMD=shutdown -r -t 1 now" | Out-Host
	do {
		start-sleep -seconds 1
	} until ((_adb_shell "true 2> /dev/null") -ne 0)

	_wait_for_adb_shell
	write-host "done!"
}

function test_rayhunter {
	$URL = "http://localhost:8080"
	$fproc = start-process $global:adb -argumentlist "forward", "tcp:8080", "tcp:8080" -wait -passthru
	if ($fproc.exitcode -ne 0) {
		write-host "adb forward tcp:8080 tcp:8080 failed with exit code $($proc.exitcode)"
		return
	}
	write-host "checking for rayhunter server..." -nonewline
	$seconds = 0
	do {
		$resp = invoke-webrequest -uri $URL
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
