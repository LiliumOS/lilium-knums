#ifndef __LILIUM__TYPES_HDL_59630789C66998F2

#define __LILIUM__TYPES_HDL_59630789C66998F2

#ifdef __cplusplus
extern "C"{
#endif /* __cplusplus */
#ifndef __HAS_LILIUM_HANDLE_DEF__
#define __handle
#define __shared_handle
#define __HAS_LILIUM_HANDLE_DEF__
#endif /*__HAS_LILIUM_HANDLE_DEF__*/
#include <lilium-sci/types/int.h>
typedef struct Handle  Handle ;
typedef struct WideHandle { Handle * __handle hdl; void const*( __pad)[((16ULL - __LILIUM_SIZEOF_POINTER__) / __LILIUM_SIZEOF_POINTER__)]; } WideHandle __attribute__((__aligned(16ULL)));
#ifdef __cplusplus
}
#endif /* __cplusplus */
#endif /* __LILIUM__TYPES_HDL_59630789C66998F2
 */
