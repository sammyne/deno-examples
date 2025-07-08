OUT_DIR = $(shell pwd)/out

.PHONY: all
all: $(OUT_DIR)/app.js $(OUT_DIR)/app.min.js $(OUT_DIR)/app.sm.min.js

$(OUT_DIR):
	mkdir -p $@

$(OUT_DIR)/app.js: $(OUT_DIR) index.js utils.js
	deno bundle -o $@ index.js

$(OUT_DIR)/app.min.js: $(OUT_DIR) index.js utils.js
	deno bundle --minify -o $@ index.js

$(OUT_DIR)/app.sm.min.js: $(OUT_DIR) index.js utils.js
	# 更加适用于生产环境的 source-map 选项值为 external，但是没找到 deno 的 source-map 接口。
	deno bundle --minify --sourcemap=inline -o $@ index.js

$(OUT_DIR)/app.esb.js: $(OUT_DIR) index.js utils.js
	esbuild --bundle --minify --sourcemap=inline --outfile=$@ index.js

run: $(OUT_DIR)/app.js $(OUT_DIR)/app.min.js $(OUT_DIR)/app.sm.min.js $(OUT_DIR)/app.esb.js
	deno run $(OUT_DIR)/app.js
	deno run $(OUT_DIR)/app.min.js
	deno run $(OUT_DIR)/app.sm.min.js
	deno run $(OUT_DIR)/app.esb.js

.PHONY: clean
clean:
	rm -rf $(OUT_DIR)
