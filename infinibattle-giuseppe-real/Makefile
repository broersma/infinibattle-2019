.PHONY: publish upload

URI := https://infinibattle.infi.nl
APIKEY := XXXXXXXXXXXXXXXXXXXXXXXXXXXX

all: help

help:
	@echo "Infinibattle Starterbot"
	@echo ""
	@echo "make build - build the bot"
	@echo "make upload - upload the bot that has been build"
	@echo "make publish - build & upload"

publish: build upload

upload:
	@echo Publishing your bot...
	curl --insecure -X POST -Ffile=@./publish/build.zip $(URI)/api/uploadBot/$(APIKEY)

build:
	mkdir -p publish
	zip -r publish/build.zip StarterBot.py -x *__pycache__*
