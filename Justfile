lib_name := "librailpack"
go_output_dir := "bins"

# Build Go library for current platform
build-lib:
	mkdir -p {{go_output_dir}}
	cd src && go build -buildmode=c-archive -o ../{{go_output_dir}}/{{lib_name}}.a railpack.go

build: build-lib

clean:
	rm -rf {{go_output_dir}}
	cargo clean
