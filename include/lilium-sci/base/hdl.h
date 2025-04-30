#ifndef __LILIUM__BASE_HDL_2329504F55315C45

#define __LILIUM__BASE_HDL_2329504F55315C45

#ifdef __cplusplus
extern "C"{
#endif /* __cplusplus */
#include <lilium-sci/types/hdl.h>
#include <lilium-sci/types/int.h>
#define HANDLE_TYPE_PROC (1ULL)
#define HANDLE_TYPE_THREAD (2ULL)
#define HANDLE_TYPE_IO (3ULL)
#define HANDLE_SUBTYPE_IO_FILE (268435459ULL)
#define HANDLE_SUBTYPE_IO_DEV (536870915ULL)
#define HANDLE_SUBTYPE_IO_PIPE_READ (805306371ULL)
#define HANDLE_SUBTYPE_IO_PIPE_WRITE (1073741827ULL)
#define HANDLE_SUBTYPE_IO_SOCKET (1342177283ULL)
#define HANDLE_SUBTYPE_IO_SERVER (1610612739ULL)
#define HANDLE_SUBTYPE_IO_MEMBUF (1879048195ULL)
#define HANDLE_SUBTYPE_IO_IPCCON (2147483651ULL)
#define HANDLE_SUBTYPE_IO_IPCSERVER (2415919107ULL)
#define HANDLE_TYPE_DEBUG (4ULL)
#define HANDLE_TYPE_SECURITY (5ULL)
#define HANDLE_TYPE_NAMESPACE (6ULL)
#define HANDLE_TYPE_ENVMAP (7ULL)
#if __LILIUM_WANT_SYSNO
#define __SYS_ShareHandle (0ULL)
#endif /* __LILIUM_WANT_SYSNO */
#if __LILIUM_WANT_SYSPROTO
extern  SysResult ShareHandle( Handle * __shared_handle ( *shdl),  Handle * __handle hdl,  __u32 flags);
#endif /* __LILIUM_WANT_SYSPROTO */
#if __LILIUM_WANT_SYSNO
#define __SYS_UnshareHandle (1ULL)
#endif /* __LILIUM_WANT_SYSNO */
#if __LILIUM_WANT_SYSPROTO
extern  SysResult UnshareHandle( Handle * __handle hdl);
#endif /* __LILIUM_WANT_SYSPROTO */
#if __LILIUM_WANT_SYSNO
#define __SYS_UpgradeSharedHandle (2ULL)
#endif /* __LILIUM_WANT_SYSNO */
#if __LILIUM_WANT_SYSPROTO
extern  SysResult UpgradeSharedHandle( Handle * __handle ( *hdlout),  Handle * __shared_handle shdl);
#endif /* __LILIUM_WANT_SYSPROTO */
#if __LILIUM_WANT_SYSNO
#define __SYS_IdentHandle (3ULL)
#endif /* __LILIUM_WANT_SYSNO */
#if __LILIUM_WANT_SYSPROTO
extern  SysResult IdentHandle( Handle * __handle hdl);
#endif /* __LILIUM_WANT_SYSPROTO */
#if __LILIUM_WANT_SYSNO
#define __SYS_CheckHandleRight (4ULL)
#endif /* __LILIUM_WANT_SYSNO */
#if __LILIUM_WANT_SYSPROTO
extern  SysResult CheckHandleRight( Handle * __handle hdl,  KStrCPtr right);
#endif /* __LILIUM_WANT_SYSPROTO */
#if __LILIUM_WANT_SYSNO
#define __SYS_DropHandleRight (5ULL)
#endif /* __LILIUM_WANT_SYSNO */
#if __LILIUM_WANT_SYSPROTO
extern  SysResult DropHandleRight( Handle * __handle hdl,  KStrCPtr right);
#endif /* __LILIUM_WANT_SYSPROTO */
#if __LILIUM_WANT_SYSNO
#define __SYS_DropAllHandleRights (6ULL)
#endif /* __LILIUM_WANT_SYSNO */
#if __LILIUM_WANT_SYSPROTO
extern  SysResult DropAllHandleRights( Handle * __handle hdl);
#endif /* __LILIUM_WANT_SYSPROTO */
#if __LILIUM_WANT_SYSNO
#define __SYS_GrantHandleRight (7ULL)
#endif /* __LILIUM_WANT_SYSNO */
#if __LILIUM_WANT_SYSPROTO
extern  SysResult GrantHandleRight( Handle * __handle hdl,  KStrCPtr right);
#endif /* __LILIUM_WANT_SYSPROTO */
#ifdef __cplusplus
}
#endif /* __cplusplus */
#endif /* __LILIUM__BASE_HDL_2329504F55315C45
 */
