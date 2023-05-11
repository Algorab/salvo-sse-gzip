# salvo-sse-gzip whith slow stream
## Description
Show's the sse behavior on a slow stream a message 0-1000 milliseconds when used the compression gzip.
For roundabout 10 minutes now output (sse, keep-alive) is written. After that period all messages are
written at once. And the next 10 minutes wait start.

## Command for testing
curl compressed
```shell
curl --compressed -i http://localhost:9090/sse -H "accept-encoding: gzip"
```

