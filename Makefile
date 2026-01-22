
OUTPUTS := base

all: stamp

.PHONY: all clean

stamp: $(OUTPUTS:%=%.imtb)
    
