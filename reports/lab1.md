# 实验总结

1. 通过对 TaskControlBlock 结构的扩展，实现对任务中系统调用次数的记录; (os/src/task/task.rs 和 os/src/task/mod.rs)
2. 在每次调用 syscall 时，进行次数的更新; (os/src/syscall/mod.rs)
3. sys_trace 根据匹配到的参数，执行不同的操作。 (os/src/syscall/process.rs)

# 简答题

1. 正确进入 U 态后，程序的特征还应有：使用 S 态特权指令，访问 S 态寄存器后会报错。 请同学们可以自行测试这些内容（运行 三个 bad 测例 (ch2b_bad_*.rs) ）， 描述程序出错行为，同时注意注明你使用的 sbi 及其版本。
    - ch2b_bad_address: [kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x804003a4, kernel killed it.
        在 U 态直接访问保留地址，触发 PageFault 异常
    - ch2b_bad_instructions: [kernel] IllegalInstruction in application, kernel killed it.
        在 U 态执行特权指令 sret，触发 IllegalInstruction 异常
    - ch2b_bad_register: [kernel] IllegalInstruction in application, kernel killed it.
        在 U 态访问 S 态寄存器，触发 IllegalInstruction 异常
2. 深入理解 trap.S 中两个函数 __alltraps 和 __restore 的作用，并回答如下问题:
    1. L40：刚进入 __restore 时，sp 代表了什么值。请指出 __restore 的两种使用情景。
        sp 指向内核栈的栈顶。使用场景：
        - trap_handler 返回用户态
        - __switch 切换任务时
    2. L43-L48：这几行汇编代码特殊处理了哪些寄存器？这些寄存器的的值对于进入用户态有何意义？请分别解释。
        ```asm
        ld t0, 32*8(sp)
        ld t1, 33*8(sp)
        ld t2, 2*8(sp)
        csrw sstatus, t0
        csrw sepc, t1
        csrw sscratch, t2
        ```
        特殊处理了 sstatus、sepc 和 sscratch 寄存器，可以保证进入用户态的正确上下文
        - sstatus：记录当前特权级和中断使能状态
        - sepc：记录异常发生时的下一条指令地址
        - sscratch：记录用户栈指针
    3. L50-L56：为何跳过了 x2 和 x4？

        ```asm
        ld x1, 1*8(sp)
        ld x3, 3*8(sp)
        .set n, 5
        .rept 27
            LOAD_GP %n
            .set n, n+1
        .endr
        ```
        - x2: 它在第 9 行 后指向的是内核栈，用户栈的栈指针保存在 sscratch 中
        - x4: 一般不会被用到
    4. L60：该指令之后，sp 和 sscratch 中的值分别有什么意义？
        ```asm
        csrrw sp, sscratch, sp
        ```
        - sp: 指向用户栈的栈顶
        - sscratch: 指向内核栈的栈顶
    5. __restore：中发生状态切换在哪一条指令？为何该指令执行之后会进入用户态？
        发生在 sret 指令，会根据 sstatus 恢复用户态，并跳转到 sepc 指向的地址执行
    6. L13：该指令之后，sp 和 sscratch 中的值分别有什么意义？
        ```asm
        csrrw sp, sscratch, sp
        ```
        - sp: 指向内核栈的栈顶
        - sscratch: 指向用户栈的栈顶
    7. 从 U 态进入 S 态是哪一条指令发生的？
        __alltraps 之前的指令

# 说明

1. 在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

    无

2. 此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

    无

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。