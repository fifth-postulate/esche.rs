# Build all the necessary workshop files

.PHONE: clean

docs: manuscript eschers/target/doc
	cd manuscript; mdbook build
	cp -r eschers/target/doc docs/

documentation: eschers/target/doc
	@echo "Creating documentation"

eschers/target/doc:
	cd eschers; cargo doc --release

clean:
	rm -rf eschers/target/doc
	rm -rf docs
