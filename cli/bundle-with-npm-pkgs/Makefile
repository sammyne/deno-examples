BUILD_DIR = build

.PHONY: all
all: $(BUILD_DIR)/app.js $(BUILD_DIR)/app.min.js

$(BUILD_DIR):
	mkdir -p $(BUILD_DIR)	

$(BUILD_DIR)/app.js: main.js $(BUILD_DIR)
	deno bundle -o $@ $<

$(BUILD_DIR)/app.min.js: main.js $(BUILD_DIR)
	deno bundle --minify -o $@ $<

brun: $(BUILD_DIR)/app.js
	deno run --allow-read $<

run: main.js
	deno run --allow-read $<

.PHONY: clean
clean:
	rm -rf $(BUILD_DIR)