.wrangler/test_bucket_written:
	npx wrangler r2 object put test-bucket/dir1/file1.txt --file=test-bucket/dir1/file1.txt --local
	npx wrangler r2 object put test-bucket/dir1/dir2/file2.txt --file=test-bucket/dir1/dir2/file2.txt --local
	npx wrangler r2 object put test-bucket/dir1/dir2/file3.jpg --file=test-bucket/dir1/dir2/file3.jpg --local
	npx wrangler r2 object put test-bucket/dir1/dir2/dir3/file4.txt --file=test-bucket/dir1/dir2/dir3/file4.txt --local
	touch .wrangler/test_bucket_written

build_frontend:
	cargo leptos build --release

build_backend:
	LEPTOS_WASM_OPT_VERSION=version_125 LEPTOS_OUTPUT_NAME=start-axum worker-build --release --features ssr

all: .wrangler/test_bucket_written build_frontend build_backend
