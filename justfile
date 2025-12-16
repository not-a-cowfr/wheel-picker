[private]
@default:
  just --list

[linux]
@package:
	cargo release

	mkdir -p dist
	cp target/release/wheel-picker dist/

	cd dist; \
	upx --best --lzma wheel-picker; \
	zip wheel-picker.zip wheel-picker;

[windows]
@package:
	cargo release

	mkdir -p dist
	Copy-Item "target/release/wheel-picker.exe" dist/

	cd dist; \
	Compress-Archive -Path "wheel-picker.exe" -DestinationPath "wheel-picker.zip";

[macos]
@package:
	cargo release

	mkdir -p dist
	cp target/release/wheel-picker} dist/

	cd dist; \
	upx --best --lzma --force-macos wheel-picker; \
	zip wheel-picker.zip wheel-picker;

[group('wrapper')]
@clean *args:
	cargo clean {{args}}
	rm -rf dist/

[group('wrapper')]
@run *args:
	cargo run {{args}}

[group('wrapper')]
@build *args:
	cargo build {{args}}
