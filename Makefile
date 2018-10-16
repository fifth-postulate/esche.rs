# Build all the necessary workshop files

.PHONE: clean

eschers.tar.gz: cargo_home docs starter workshop.md
	 tar xcvf eschers.tar.gz cargo_home docs starter workshop.md

cargo_home: eschers
	./fetch-deps

docs: manuscript eschers/target/doc
	cd manuscript; mdbook build
	cp -r eschers/target/doc docs/

documentation: eschers/target/doc
	@echo "Creating documentation"

eschers/target/doc:
	cd eschers; cargo doc --release

clean:
	rm eschers.tar.gz
	rm -rf cargo_home
	rm -rf eschers/target/doc
	rm -rf docs
