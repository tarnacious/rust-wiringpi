unexport CFLAGS

wiringpi:
	@echo >&2 $(CFLAGS)
	$(MAKE) -C wiringPi/wiringPi clean
	$(MAKE) -C wiringPi/wiringPi static CC=arm-linux-gnueabihf-gcc DEBUG=-O2
	rm -f $(OUT_DIR)/libwiringpi.a
	cp wiringPi/wiringPi/libwiringPi.a $(OUT_DIR)/libwiringpi.a

devlib:
	$(MAKE) -C wiringPi/devLib clean
	$(MAKE) -C wiringPi/devLib static CC=arm-linux-gnueabihf-gcc DEBUG=-O2
	rm -f $(OUT_DIR)/libwiringPiDev.a
	cp wiringPi/devLib/libwiringPiDev.a $(OUT_DIR)/libwiringPiDev.a

all: wiringpi devlib

.PHONY: wiringpi devlib
