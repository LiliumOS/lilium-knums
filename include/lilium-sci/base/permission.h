#ifndef __LILIUM__BASE_PERMISSION_7BBF955E9EAF680D

#define __LILIUM__BASE_PERMISSION_7BBF955E9EAF680D

#ifdef __cplusplus
extern "C"{
#endif /* __cplusplus */
#include <lilium-sci/types/hdl.h>
#include <lilium-sci/types/int.h>
#include <lilium-sci/types/sysresult.h>
#include <lilium-sci/types/uuid.h>
#include <lilium-sci/types/str.h>
#include <lilium-sci/types/slice.h>
#include <lilium-sci/thread/hdl.h>
#include <lilium-sci/process/hdl.h>
typedef struct SecurityContextHandle  SecurityContextHandle ;
typedef union ThreadOwner { WideHandle hdl; Uuid principal; } ThreadOwner ;
#if __LILIUM_WANT_SYSNO
#define __SYS_CreateSecurityContext (16ULL)
#endif /* __LILIUM_WANT_SYSNO */
#if __LILIUM_WANT_SYSPROTO
extern  SysResult CreateSecurityContext( SecurityContext * __handle ( *nctx));
#endif /* __LILIUM_WANT_SYSPROTO */
#if __LILIUM_WANT_SYSNO
#define __SYS_CopySecurityContext (17ULL)
#endif /* __LILIUM_WANT_SYSNO */
#if __LILIUM_WANT_SYSPROTO
extern  SysResult CopySecurityContext( SecurityContext * __handle ( *nctx),  SecurityContext * __handle ctx);
#endif /* __LILIUM_WANT_SYSPROTO */
#if __LILIUM_WANT_SYSNO
#define __SYS_DestroySecurityContext (18ULL)
#endif /* __LILIUM_WANT_SYSNO */
#if __LILIUM_WANT_SYSPROTO
extern  SysResult DestroySecurityContext( SecurityContext * __handle ctx);
#endif /* __LILIUM_WANT_SYSPROTO */
#if __LILIUM_WANT_SYSNO
#define __SYS_GetCurrentSecurityContext (19ULL)
#endif /* __LILIUM_WANT_SYSNO */
#if __LILIUM_WANT_SYSPROTO
extern  SysResult GetCurrentSecurityContext( SecurityContext * __handle ( *nctx));
#endif /* __LILIUM_WANT_SYSPROTO */
#if __LILIUM_WANT_SYSNO
#define __SYS_HasKernelPermission (20ULL)
#endif /* __LILIUM_WANT_SYSNO */
#if __LILIUM_WANT_SYSPROTO
extern  SysResult HasKernelPermission( SecurityContext * __handle ctx,  KStrCPtr perm);
#endif /* __LILIUM_WANT_SYSPROTO */
#if __LILIUM_WANT_SYSNO
#define __SYS_HasThreadPermission (21ULL)
#endif /* __LILIUM_WANT_SYSNO */
#if __LILIUM_WANT_SYSPROTO
extern  SysResult HasThreadPermission( SecurityContext * __handle ctx,  ThreadHandle * __handle th,  KStrCPtr perm);
#endif /* __LILIUM_WANT_SYSPROTO */
#if __LILIUM_WANT_SYSNO
#define __SYS_HasProcessPermission (22ULL)
#endif /* __LILIUM_WANT_SYSNO */
#if __LILIUM_WANT_SYSPROTO
extern  SysResult HasProcessPermission( SecurityContext * __handle ctx,  ProcessHandle * __handle ph,  KStrCPtr perm);
#endif /* __LILIUM_WANT_SYSPROTO */
#if __LILIUM_WANT_SYSNO
#define __SYS_SetPrimaryPrincipal (23ULL)
#endif /* __LILIUM_WANT_SYSNO */
#if __LILIUM_WANT_SYSPROTO
extern  SysResult SetPrimaryPrincipal( SecurityContext * __handle ctx,  Uuid const*principal);
#endif /* __LILIUM_WANT_SYSPROTO */
#if __LILIUM_WANT_SYSNO
#define __SYS_AddSecondaryPrincipal (24ULL)
#endif /* __LILIUM_WANT_SYSNO */
#if __LILIUM_WANT_SYSPROTO
extern  SysResult AddSecondaryPrincipal( SecurityContext * __handle ctx,  Uuid const*principal);
#endif /* __LILIUM_WANT_SYSPROTO */
#if __LILIUM_WANT_SYSNO
#define __SYS_GrantKernelPermission (25ULL)
#endif /* __LILIUM_WANT_SYSNO */
#if __LILIUM_WANT_SYSPROTO
extern  SysResult GrantKernelPermission( SecurityContext * __handle ctx,  KStrCPtr perm,  c_long status);
#endif /* __LILIUM_WANT_SYSPROTO */
#if __LILIUM_WANT_SYSNO
#define __SYS_GrantThreadPermission (26ULL)
#endif /* __LILIUM_WANT_SYSNO */
#if __LILIUM_WANT_SYSPROTO
extern  SysResult GrantThreadPermission( SecurityContext * __handle ctx,  ThreadHandle * __handle th,  KStrCPtr perm,  c_long status);
#endif /* __LILIUM_WANT_SYSPROTO */
#if __LILIUM_WANT_SYSNO
#define __SYS_GrantProcessPermission (27ULL)
#endif /* __LILIUM_WANT_SYSNO */
#if __LILIUM_WANT_SYSPROTO
extern  SysResult GrantProcessPermission( SecurityContext * __handle ctx,  ProcessHandle * __handle ph,  KStrCPtr perm,  c_long status);
#endif /* __LILIUM_WANT_SYSPROTO */
#if __LILIUM_WANT_SYSNO
#define __SYS_DropKernelPermission (28ULL)
#endif /* __LILIUM_WANT_SYSNO */
#if __LILIUM_WANT_SYSPROTO
extern  SysResult DropKernelPermission( SecurityContext * __handle ctx,  KStrCPtr perm,  c_long status);
#endif /* __LILIUM_WANT_SYSPROTO */
#if __LILIUM_WANT_SYSNO
#define __SYS_DropThreadPermission (29ULL)
#endif /* __LILIUM_WANT_SYSNO */
#if __LILIUM_WANT_SYSPROTO
extern  SysResult DropThreadPermission( SecurityContext * __handle ctx,  ThreadHandle * __handle th,  KStrCPtr perm,  c_long status);
#endif /* __LILIUM_WANT_SYSPROTO */
#if __LILIUM_WANT_SYSNO
#define __SYS_DropProcessPermission (30ULL)
#endif /* __LILIUM_WANT_SYSNO */
#if __LILIUM_WANT_SYSPROTO
extern  SysResult DropProcessPermission( SecurityContext * __handle ctx,  ProcessHandle * __handle ph,  KStrCPtr perm,  c_long status);
#endif /* __LILIUM_WANT_SYSPROTO */
#if __LILIUM_WANT_SYSNO
#define __SYS_RevokeKernelPermission (31ULL)
#endif /* __LILIUM_WANT_SYSNO */
#if __LILIUM_WANT_SYSPROTO
extern  SysResult RevokeKernelPermission( SecurityContext * __handle ctx,  KStrCPtr perm);
#endif /* __LILIUM_WANT_SYSPROTO */
#if __LILIUM_WANT_SYSNO
#define __SYS_RevokeThreadPermission (32ULL)
#endif /* __LILIUM_WANT_SYSNO */
#if __LILIUM_WANT_SYSPROTO
extern  SysResult RevokeThreadPermission( SecurityContext * __handle ctx,  ThreadHandle * __handle th,  KStrCPtr perm);
#endif /* __LILIUM_WANT_SYSPROTO */
#if __LILIUM_WANT_SYSNO
#define __SYS_RevokeProcessPermission (33ULL)
#endif /* __LILIUM_WANT_SYSNO */
#if __LILIUM_WANT_SYSPROTO
extern  SysResult RevokeProcessPermission( SecurityContext * __handle ctx,  ProcessHandle * __handle ph,  KStrCPtr perm);
#endif /* __LILIUM_WANT_SYSPROTO */
#if __LILIUM_WANT_SYSNO
#define __SYS_SetKernelResourceLimit (34ULL)
#endif /* __LILIUM_WANT_SYSNO */
#if __LILIUM_WANT_SYSPROTO
extern  SysResult SetKernelResourceLimit( SecurityContext * __handle ctx,  KStrCPtr limit_name,  __u64 value);
#endif /* __LILIUM_WANT_SYSPROTO */
#if __LILIUM_WANT_SYSNO
#define __SYS_GetKernelResourceLimit (35ULL)
#endif /* __LILIUM_WANT_SYSNO */
#if __LILIUM_WANT_SYSPROTO
extern  SysResult GetKernelResourceLimit( SecurityContext * __handle ctx,  KStrCPtr limit_name,  __u64 *value);
#endif /* __LILIUM_WANT_SYSPROTO */
#if __LILIUM_WANT_SYSNO
#define __SYS_EncodeSecurityContext (36ULL)
#endif /* __LILIUM_WANT_SYSNO */
#if __LILIUM_WANT_SYSPROTO
extern  SysResult EncodeSecurityContext( SecurityContext * __handle ctx,  __u8 *buffer,  __ulong *len);
#endif /* __LILIUM_WANT_SYSPROTO */
#if __LILIUM_WANT_SYSNO
#define __SYS_GetPrimaryPrincipal (37ULL)
#endif /* __LILIUM_WANT_SYSNO */
#if __LILIUM_WANT_SYSPROTO
extern  SysResult GetPrimaryPrincipal( SecurityContext * __handle ctx,  Uuid *principal);
#endif /* __LILIUM_WANT_SYSPROTO */
#if __LILIUM_WANT_SYSNO
#define __SYS_GetSecondaryPrincipals (38ULL)
#endif /* __LILIUM_WANT_SYSNO */
#if __LILIUM_WANT_SYSPROTO
extern  SysResult GetSecondaryPrincipals( SecurityContext * __handle ctx,  KSlice *principals);
#endif /* __LILIUM_WANT_SYSPROTO */
#ifdef __cplusplus
}
#endif /* __cplusplus */
#endif /* __LILIUM__BASE_PERMISSION_7BBF955E9EAF680D
 */
