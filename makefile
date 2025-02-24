git-push:
	git add .
	git commit -m "$(msg) - $(shell date '+%Y-%m-%d %H:%M:%S')"
	git push
main: main.py
	python3 main.py