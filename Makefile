build:
	chmod 777 ./scripts/build.sh
	./scripts/build.sh

server:
	./bin/server

client:
	./bin/client

aliases:
	@echo "Run: source .aliases"
	@echo "Then use: war-server or war-client"

.PHONY: build server client aliases
