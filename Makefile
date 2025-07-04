# Detect platform and run appropriate build script
build:
	@if command -v pwsh >/dev/null 2>&1; then \
		echo "Using PowerShell build script..."; \
		pwsh -ExecutionPolicy Bypass -File ./scripts/build.ps1; \
	elif [ "$$OS" = "Windows_NT" ] && command -v powershell >/dev/null 2>&1; then \
		echo "Using Windows PowerShell build script..."; \
		powershell -ExecutionPolicy Bypass -File ./scripts/build.ps1; \
	else \
		echo "Using Bash build script..."; \
		chmod +x ./scripts/build.sh; \
		./scripts/build.sh; \
	fi

server:
	./bin/server

client:
	./bin/client

aliases:
	@echo "Run: source .aliases"
	@echo "Then use: war-server or war-client"

.PHONY: build server client aliases
