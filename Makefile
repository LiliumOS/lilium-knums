
all: include/stamp



'include/stamp':
	cargo run

include knums.d

knums.d:
	cargo run