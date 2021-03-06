# 应该比较常见的问题

## 条件跳转

如果需要实现条件跳转，请使用以下指令的组合（`T` 代表 `u`、`f` 或 `i`；在 **符合条件** 时跳转）：

- 等于：`cmp.T`, `br.false`
- 不等于：`cmp.T`, `br.true`
- 大于：`cmp.T`, `set.gt`, `br.true`
- 小于：`cmp.T`, `set.lt`, `br.true`
- 大于等于：`cmp.T`, `set.lt`, `br.false`
- 小于等于：`cmp.T`, `set.gt`, `br.false`

## 局部变量和参数的存取

在 navm 中，局部变量和参数是分开存储的。其中，参数和返回值（`arg`）存储在一起，从栈底方向开始顺序编号。局部变量（`loc`）存储在另一个位置，也从栈底开始顺序编号。比如：

```
| d            | ↑          loc.1
| c            | 局部变量   loc.0
|==============|
| 1            | ↑          
| %ip          |            
| %bp          | 虚拟机数据 
|==============|
| b            | ↑          arg.2
| a            | 参数       arg.1
| _ret         | 返回值     arg.0
| ...          |
```

此时执行 `loca 1` 获得的就是变量 `d` 的地址，执行 `arga 0` 获得的就是返回值的地址。

获取到地址之后，就可以执行存取操作了。我们用的基本都是 64 位数据类型，所以使用 `load.64` 和 `store.64` 指令就可以了。

```
# 加载局部变量 1
loca 1
load.64

# 存储 0 到参数 0
arga 0
push 0
store.64

# 将局部变量 1 拷贝到局部变量 0
loca 0
loca 1
load.64
store.64
```
