/*
** Baremetal stub for the Lua I/O library.
** File I/O is not available in baremetal/wasm environments, so luaopen_io
** registers an empty table instead of crashing or requiring a host import.
*/

#define liolib_c
#define LUA_LIB

#include "lprefix.h"

#include "lua.h"
#include "lauxlib.h"
#include "lualib.h"

#ifdef __cplusplus
extern "C" {
#endif

static const luaL_Reg iolib[] = {
  {NULL, NULL}
};

LUAMOD_API int luaopen_io(lua_State *L) {
  luaL_newlib(L, iolib);
  return 1;
}

#ifdef __cplusplus
}
#endif
