# Build all the necessary workshop files

.PHONE: clean

documentation: eschers/target/doc
	@echo "Creating documentation"

eschers/target/doc:
	cd eschers; cargo doc --release

clean:
	rm -rf eschers/target/doc
