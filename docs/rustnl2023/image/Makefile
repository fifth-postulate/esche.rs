.PHONY: all clean

SVGS=$(wildcard *.svg)
PNGS=$(SVGS:svg=png)

all: ${PNGS}
	@echo "Finished creating pngs"

%.png: %.svg
	inkscape -w 200 -D -f $< -e $@

clean:
	rm -rf ${PNGS}