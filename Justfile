run *args:
	cargo run -- {{ args }}

gtk-debug *args:
	GTK_DEBUG=interactive just run {{ args }}

fmt *args:
	cargo fmt -- {{ args }}

play file *args:
	python -m f1_2019_telemetry.cli.player "{{ file }}" {{ args }}

record file *args:
	python -m f1_2019_telemetry.cli.recorder -f "{{ file }}" {{ args }}

# vim: ft=make
