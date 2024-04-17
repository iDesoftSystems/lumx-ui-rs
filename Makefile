build:
	npx tailwindcss -i ./tailwind-entry.css -o ./assets/lumx-ui.css
	cargo build
