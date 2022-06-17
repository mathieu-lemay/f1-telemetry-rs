run *args:
	cargo run -- {{ args }}

nc *args:
	#! /bin/sh
	cargo build
	ret=$?
	[[ ${ret} -ne 0 ]] && exit ${ret}
	cargo run -- --ui=ncurses {{ args }}
	ret=$?
	[[ ${ret} -ne 0 ]] && reset
	exit ${ret}

gtk-debug *args:
	GTK_DEBUG=interactive just run {{ args }}

fmt *args:
	cargo fmt -- {{ args }}

play file *args:
	f1udpcap play "{{ file }}" {{ args }}

record file *args:
	f1udpcap record -f "{{ file }}" {{ args }}

# vim: ft=make
