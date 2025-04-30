#ifndef __LILIUM__SYSCALL_A19D72E4D613EA5A

#define __LILIUM__SYSCALL_A19D72E4D613EA5A

#ifdef __cplusplus
extern "C"{
#endif /* __cplusplus */
#define __LILIUM_SYSNO(__subsys, __sysno) ((__subsys << 12) | __sysno)
#define __LILIUM_ERRNO(__subsys, __errno) (-((__subsys << 8) | (-__errno)))
#include <lilium-sci/base/syscall.h>
#ifdef __cplusplus
}
#endif /* __cplusplus */
#endif /* __LILIUM__SYSCALL_A19D72E4D613EA5A
 */
