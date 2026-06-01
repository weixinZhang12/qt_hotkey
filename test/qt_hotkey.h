#include <cstdarg>
#include <cstdint>
#include <cstdlib>
#include <ostream>
#include <new>

struct MyHotKeyEvent
{
  unsigned int id;
  unsigned char state;
};

typedef void (*CallbackFn)(struct MyHotKeyEvent event);

extern "C"
{

  /// 用于绑定快捷键,返回值：0 -> 成功 -1 -> 失败
  ///  # Safety
  ///  你必须保证key的范围在0-214之间（包括），modifiers的范围在 0-13之间（包括）,GLM必须初始化
  int key_register(unsigned char modifiers,
                   unsigned char key,
                   unsigned int *id);
  /// 返回值：0 -> 成功 -1 -> 失败
  ///  # Safety
  ///   该方法传入的函数必须是线程安全的函数
  int handle(CallbackFn fun);

  /// 初始化全局GLM，返回值：0->初始化成功 -1初始化失败
  ///  # Safety
  ///  该方法无特殊安全要求,但是在使用GLM前你必须调用该方法，且GLM不是线程安全的。
  int init_glm();
}
