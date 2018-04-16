GIR = gir/target/bin/gir
GIR_SRC = gir/Cargo.toml gir/Cargo.lock gir/build.rs $(shell find gir/src -name '*.rs')
GIR_FILES = gir-files/NM-1.0.gir

# Run `gir` generating the bindings
gir : src/auto/mod.rs nm-sys/src/lib.rs

src/auto/mod.rs : Gir.toml $(GIR) $(GIR_FILES)
	$(GIR) -c Gir.toml

nm-sys/src/lib.rs : Gir.sys.toml $(GIR) $(GIR_FILES)
	$(GIR) -c Gir.sys.toml -o $(abspath nm-sys) -d gir-files

$(GIR) : $(GIR_SRC)
	rm -f gir/target/bin/gir
	cargo install --path gir --root gir/target
	rm -f gir/target/.crates.toml

$(GIR_SRC) $(GIR_FILES) :
	git submodule update --init
