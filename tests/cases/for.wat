(module
 (type $none_=>_i32 (func (result i32)))
 (memory $0 0)
 (export "three" (func $module/three))
 (export "memory" (memory $0))
 (func $module/three (result i32)
  (local $0 i32)
  (local $1 i32)
  loop $for-loop|0
   local.get $0
   i32.const 3
   i32.lt_s
   if
    local.get $0
    local.get $1
    i32.add
    local.set $1
    local.get $0
    i32.const 1
    i32.add
    local.set $0
    br $for-loop|0
   end
  end
  local.get $1
 )
)

