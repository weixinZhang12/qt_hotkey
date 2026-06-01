// main.c
#include <stdio.h>
#include <stdint.h>
#include <windows.h>
#include "qt_hotkey.h"

void xxx(MyHotKeyEvent event)
{
    printf("xxx: id=%u, state=%u\n", event.id, event.state);
}

int main()
{
    printf("Test start\n");
    // Initialize
    int ret = init_glm();
    printf("init_glm() = %d\n", ret);
    if (ret != 0)
    {
        printf("Init failed\n");
        return 1;
    }

    // Register hotkey: Ctrl + D (modifiers=3 for CONTROL, key=22 for KeyD)
    uint32_t id;
    ret = key_register(3, 22, &id);
    printf("key_register() = %d, id = %u\n", ret, id);
    if (ret != 0)
    {
        printf("Register hotkey failed\n");
        return 1;
    }
    printf("Hotkey registered, waiting for events...\n");
    printf("Press Ctrl+D to trigger, Ctrl+C to exit\n");

    // Windows message loop - required for global hotkey events
    MSG msg;
    while (GetMessage(&msg, NULL, 0, 0))
    {
        TranslateMessage(&msg);
        DispatchMessage(&msg);
        // Poll for hotkey events after each message
        handle(xxx);
    }

    printf("Test complete\n");
    return 0;
}
