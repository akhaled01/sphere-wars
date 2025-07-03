build:
	chmod 777 ./scripts/build.sh
	./scripts/build.sh
	
server:
	./bin/server

.PHONY: build server client
