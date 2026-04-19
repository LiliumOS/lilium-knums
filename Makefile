
OUTPUTS := base thread

all: stamp

.PHONY: all clean

stamp: $(OUTPUTS:%=%.imtb)
	touch stamp

clean:
	rm -f stamp *.imtb *.imtb.d


include $(OUTPUTS:%=%.imtb.d)

$(OUTPUTS:%=%.imtb.d): %.imtb.d: src/%/
	cargo run --manifest-path knums-tool-new/Cargo.toml $(CARGOFLAGS) -- --bundle --deps --no-output -- $< $*.imtb

$(OUTPUTS:%=%.imtb): %.imtb: src/%/
	cargo run --manifest-path knums-tool-new/Cargo.toml $(CARGOFLAGS) -- --bundle --deps -- $< $@
