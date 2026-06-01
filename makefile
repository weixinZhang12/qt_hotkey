tt:
	clear
	-taskkill /f /im test.exe 2>nul
	g++ test/test.cpp  -Ltarget/release/ -lqt_hotkey -o test/test.exe
	cp target/release/qt_hotkey.dll test/
	./test/test.exe
