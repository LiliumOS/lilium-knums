


all: 'include/stamp'

.PHONY: all clean

clean:
	rm -rf include

'include/stamp':
	cargo run

include knums.d

knums.d:
	cargo run