.PHONY :test
.PHONY :bind
bind:
	cbindgen --output ./test/qt_hotkey.h
test:
	clear
	-taskkill /f /im test.exe 2
	g++ test/test.cpp  -Ltarget/release/ -lqt_hotkey -o test/test.exe
	cp target/release/qt_hotkey.dll test/
	./test/test.exe
