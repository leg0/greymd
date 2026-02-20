# Quickstart: Static File Server

## Build

```sh
cargo build --release
```

## Run

```sh
# Serve current directory
./target/release/docsvr

# Serve a specific directory
./target/release/docsvr /path/to/docs

# Show help
./target/release/docsvr --help
```

## Test Scenarios

### 1. Basic file serving

```sh
# Setup: create a test directory with a file
mkdir -p /tmp/docsvr-test
echo "<h1>Hello</h1>" > /tmp/docsvr-test/index.html

# Start server
./target/release/docsvr /tmp/docsvr-test

# In another terminal:
curl -i http://127.0.0.1:8080/index.html
# Expected: HTTP/1.1 200 OK, Content-Type: text/html, body: <h1>Hello</h1>
```

### 2. Subdirectory access

```sh
mkdir -p /tmp/docsvr-test/sub
echo "nested" > /tmp/docsvr-test/sub/file.txt

curl -i http://127.0.0.1:8080/sub/file.txt
# Expected: HTTP/1.1 200 OK, Content-Type: text/plain, body: nested
```

### 3. Missing file (404)

```sh
curl -i http://127.0.0.1:8080/nonexistent.txt
# Expected: HTTP/1.1 404 Not Found
```

### 4. Directory traversal (blocked)

```sh
curl -i http://127.0.0.1:8080/../../../etc/passwd
# Expected: HTTP/1.1 404 Not Found (or 403 Forbidden)
```

### 5. Non-GET method (405)

```sh
curl -i -X POST http://127.0.0.1:8080/index.html
# Expected: HTTP/1.1 405 Method Not Allowed
```

### 6. Port conflict

```sh
# Start first instance
./target/release/docsvr /tmp/docsvr-test &

# Start second instance on same port
./target/release/docsvr /tmp/docsvr-test
# Expected: Error message about port 8080 being in use
```

### 7. Content types

```sh
echo '{"key":"val"}' > /tmp/docsvr-test/data.json
echo 'body { color: red; }' > /tmp/docsvr-test/style.css

curl -sI http://127.0.0.1:8080/data.json | grep Content-Type
# Expected: Content-Type: application/json

curl -sI http://127.0.0.1:8080/style.css | grep Content-Type
# Expected: Content-Type: text/css
```
