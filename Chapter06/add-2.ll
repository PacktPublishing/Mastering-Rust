; ModuleID = 'add_2.cgu-0.rs'
source_filename = "add_2.cgu-0.rs"
target datalayout = "e-m:e-i64:64-f80:128-n8:16:32:64-S128"
target triple = "x86_64-unknown-linux-gnu"

%str_slice = type { i8*, i64 }

@str.0 = internal constant [8 x i8] c"add-2.rs"
@str.1 = internal constant [28 x i8] c"attempt to add with overflow"
@panic_loc.2 = internal unnamed_addr constant { %str_slice, %str_slice, i32 } { %str_slice { i8* getelementptr inbounds ([28 x i8], [28 x i8]* @str.1, i32 0, i32 0), i64 28 }, %str_slice { i8* getelementptr inbounds ([8 x i8], [8 x i8]* @str.0, i32 0, i32 0), i64 8 }, i32 2 }, align 8

; Function Attrs: uwtable
define internal i8 @_ZN5add_27add_one17h78adf02a741f4a27E(i8) unnamed_addr #0 {
entry-block:
  br label %start

start:                                            ; preds = %entry-block
  %1 = call { i8, i1 } @llvm.uadd.with.overflow.i8(i8 %0, i8 -1)
  %2 = extractvalue { i8, i1 } %1, 0
  %3 = extractvalue { i8, i1 } %1, 1
  %4 = call i1 @llvm.expect.i1(i1 %3, i1 false)
  br i1 %4, label %panic, label %bb1

bb1:                                              ; preds = %start
  ret i8 %2

panic:                                            ; preds = %start
  call void @_ZN4core9panicking5panic17h7abeb5b818ec354eE({ %str_slice, %str_slice, i32 }* @panic_loc.2)
  unreachable
}

; Function Attrs: uwtable
define internal void @_ZN5add_24main17h55a5483c3ba0c0f8E() unnamed_addr #0 {
entry-block:
  %_0 = alloca {}
  br label %start

start:                                            ; preds = %entry-block
  %0 = call i8 @_ZN5add_27add_one17h78adf02a741f4a27E(i8 1)
  br label %bb1

bb1:                                              ; preds = %start
  ret void
}

; Function Attrs: nounwind readnone
declare { i8, i1 } @llvm.uadd.with.overflow.i8(i8, i8) #1

; Function Attrs: nounwind readnone
declare i1 @llvm.expect.i1(i1, i1) #1

; Function Attrs: cold noinline noreturn
declare void @_ZN4core9panicking5panic17h7abeb5b818ec354eE({ %str_slice, %str_slice, i32 }* noalias readonly dereferenceable(40)) unnamed_addr #2

define i64 @main(i64, i8**) unnamed_addr {
top:
  %2 = call i64 @_ZN3std2rt10lang_start17hd7c880a37a646e81E(i8* bitcast (void ()* @_ZN5add_24main17h55a5483c3ba0c0f8E to i8*), i64 %0, i8** %1)
  ret i64 %2
}

declare i64 @_ZN3std2rt10lang_start17hd7c880a37a646e81E(i8*, i64, i8**) unnamed_addr

attributes #0 = { uwtable }
attributes #1 = { nounwind readnone }
attributes #2 = { cold noinline noreturn }

!llvm.module.flags = !{!0}

!0 = !{i32 1, !"PIE Level", i32 2}
