default: auto

auto:
	autocmd -v -t "\.rs" cargo run

url:
	https://doc.rust-jp.rs/book-ja/ch00-00-introduction.html

hi:
	rustc --version

update_rust_environment:
	rustup update

uninstall_rust_environment:
	rustup self uninstall

cargo_new_env:
	cargo new hello_cargo

cargo_build:
	cargo build

cargo_build_release:
	cargo build --release

cargo_build_and_execute:
	cargo run

cargo_force_update_dependencies:
	cargo update

.PHONY: default auto url hi update_rust_environment uninstall_rust_environment cargo_new_env
.PHONY: cargo_build cargo_build_and_execute cargo_build_release cargo_force_update_dependencies
