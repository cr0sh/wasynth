(module
 (type $none_=>_i32 (func (result i32)))
 (memory $0 0)
 (export "three" (func $module/three))
 (export "memory" (memory $0))
 (func $module/three (result i32)
  i32.const 3
 )
)
