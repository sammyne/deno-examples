OUT_DIR = $(shell pwd)/out

.PHONY: all
all: $(OUT_DIR)/app.js $(OUT_DIR)/app.min.js $(OUT_DIR)/app.sm.min.js

$(OUT_DIR):
	mkdir -p $@

$(OUT_DIR)/app.js: $(OUT_DIR) index.ts utils.ts
	deno bundle -o $@ index.ts

$(OUT_DIR)/app.min.js: $(OUT_DIR) index.ts utils.ts
	deno bundle --minify -o $@ index.ts

$(OUT_DIR)/app.sm.min.js: $(OUT_DIR) index.ts utils.ts
	# 更加适用于生产环境的 source-map 选项值为 external，但是没找到 deno 的 source-map 接口。
	deno bundle --minify --sourcemap=inline -o $@ index.ts

$(OUT_DIR)/app.esb.js: $(OUT_DIR) index.ts utils.ts
	esbuild --bundle --minify --sourcemap=inline --outfile=$@ index.ts

run: $(OUT_DIR)/app.js $(OUT_DIR)/app.min.js $(OUT_DIR)/app.sm.min.js
	deno run $(OUT_DIR)/app.js
	deno run $(OUT_DIR)/app.min.js
	deno run $(OUT_DIR)/app.sm.min.js

run-esb: $(OUT_DIR)/app.esb.js
	deno run $<

.PHONY: clean
clean:
	rm -rf $(OUT_DIR)
