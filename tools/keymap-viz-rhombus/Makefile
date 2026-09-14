export PATH := /workspace/racket-install/racket/bin:$(PATH)

.PHONY: test artifacts viz png all clean

test:
	./run.sh test

artifacts:
	./run.sh artifacts

# make viz LAYOUT=48key-basic
viz:
	./run.sh viz $(LAYOUT)

png:
	./run.sh png

all: test artifacts

clean:
	rm -rf out/*.svg out/*.png out/points-*.json out/.cache
