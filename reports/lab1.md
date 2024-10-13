# 功能实现

+ 在sys_task_info中使用unsafe代码解引用指针，填充TaskInfo
+ 返回值设为0

## status如何实现
  依据提示设置成TaskStatus::Running即可
## syscall_times如何实现
+ 在TaskControlBlock结构体中定义字段
+ TaskManager静态实例化中初始化数组值为0
+ 在TaskControlBlock自身或者TaskManager中实现统计功能
+ 在task的mod中对外暴露获取和更新的fn
+ 由user中调用的syscall会在os的syscall中被捕获，在os的syscall中调用syscall_times的更新方法
## time如何实现
> 程序运行总时间，也就是说需要记录开始时间，然后调用的时间减去开始的时间即可

+ 在TaskControlBlock结构体中定义字段
+ TaskManager静态实例化中初始化值为0
+ 在TaskManager中的task执行和切换的fn中正确记录到开始值
+ 在task的mod中对外暴露获取的fn


# 问答题
## 1. 使用的rustsdi版本为RustSBI-QEMU Version 0.2.0-alpha.2
+ ch2b_bad_address 
    ```
    [kernel] PageFault in application, bad addr = 0x0, bad instruction = 0x804003a4, kernel killed it.
    ```

    使用了错误地址进行进行操作
    
+ ch2b_bad_instructions
    ```
    [kernel] IllegalInstruction in application, kernel killed it.
    ```

+ ch2b_bad_register 
    ```
    [kernel] IllegalInstruction in application, kernel killed it.
    ```

## 2
### 2.1

### 2.2

### 2.3

### 2.4

### 2.5

### 2.6

### 2.7


# 荣誉准则

在完成本次实验的过程（含此前学习的过程）中，我曾分别与 以下各位 就（与本次实验相关的）以下方面做过交流，还在代码中对应的位置以注释形式记录了具体的交流对象及内容：

《你交流的对象说明》

此外，我也参考了 以下资料 ，还在代码中对应的位置以注释形式记录了具体的参考来源及内容：

《你参考的资料说明》

3. 我独立完成了本次实验除以上方面之外的所有工作，包括代码与文档。 我清楚地知道，从以上方面获得的信息在一定程度上降低了实验难度，可能会影响起评分。

4. 我从未使用过他人的代码，不管是原封不动地复制，还是经过了某些等价转换。 我未曾也不会向他人（含此后各届同学）复制或公开我的实验代码，我有义务妥善保管好它们。 我提交至本实验的评测系统的代码，均无意于破坏或妨碍任何计算机系统的正常运转。 我清楚地知道，以上情况均为本课程纪律所禁止，若违反，对应的实验成绩将按“-100”分计。


# 我的看法

问答题难度有点大，比较难理解