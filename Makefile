# Build artifacts folder
ARTIFACTS := "./build_artifacts"

# dim white italic
DIM       := \033[2;3;37m

# bold cyan
INFO      := \033[1;36m

# bold green
PASS      := \033[1;32m

# bold red
WARN      := \033[1;31m

RESET     := \033[0m

ARTIFACTS := "./build_artifacts"

# dim white italic
DIM            := \033[2;3;37m

# bold cyan
INFO           := \033[1;36m

# bold green
PASS           := \033[1;32m

# bold red
WARN           := \033[1;31m

RESET          := \033[0m

.PHONY: artifacts
artifacts:
	@if [ -d "$(ARTIFACTS)/starkgate_4594188" ] || \
		[ -d "$(ARTIFACTS)/starkgate_c08863a" ] || \
		[ -d "$(ARTIFACTS)/starkgate_82e651f" ] || \
		[ -d "$(ARTIFACTS)/cairo_lang"        ] || \
		[ -d "$(ARTIFACTS)/piltover"          ] || \
		[ -d "$(ARTIFACTS)/local_contracts"   ]; \
	then \
		echo -e "$(DIM)Build artifacts have already been generated, do you want to overwrite them?$(RESET) $(PASS)[y/N] $(RESET)" && \
			read ans && \
			case "$$ans" in \
				[yY]*) true;; \
			*) false;; \
		esac \
	fi
	@echo -e "$(WARN)Removing existing artifacts$(RESET)"
	@rm -rf $(ARTIFACTS)/starkgate_4594188
	@rm -rf $(ARTIFACTS)/starkgate_c08863a
	@rm -rf $(ARTIFACTS)/starkgate_82e651f
	@rm -rf $(ARTIFACTS)/cairo_lang
	@rm -rf $(ARTIFACTS)/piltover
	@rm -rf $(ARTIFACTS)/local_contracts
	@docker build -f ./build_artifacts/Dockerfile -t contracts .
	@docker create --name contracts contracts do-nothing > /dev/null
	@docker cp contracts:/artifacts/. ./build_artifacts/
	@docker rm contracts > /dev/null
