objects = jpegger.o

jpegger: $(objects)
	cc -o jpegger $(objects)

.PHONY: clean

clean:
	rm -f $(objects)
