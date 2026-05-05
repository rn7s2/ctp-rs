# LocalCTP

ctp-rs 附带了 LocalCTP (MIT协议) 来提供 TraderApi 的本地调试能力。

## 当前版本

- 2026.5.3\
  <https://github.com/dearleeyoung/LocalCTP/commit/273988397c4c1cbf6b0b9537175df4df1e96f0b8>

  ctp-rs 对 LocalCTP 源码的修改：
  - **macOS 支持**：`LeeDateTime.h` / `LeeDateTime.cpp` 中所有 `#if defined(__linux__)` 分支扩展为 `#if defined(__linux__) || defined(__APPLE__)`，让 POSIX 时间 API 路径（`timegm`、`gmtime_r`、`strftime`、`strptime`、`timeval` 等）在 macOS 上同样生效
  - **UTC+8 钉住**：`LeeDateTime` 不再读取操作系统时区，始终按 Asia/Shanghai 计算交易日和结算时间，避免在非北京时区主机上出现日期错位
  - **启动期同步**：在 `CSettlementHandler` 中新增 `s_ready` / `s_readyMtx` / `s_readyCv` 与 `WaitUntilReady()`，让用户线程能阻塞到 `m_timerThread` 完成首轮 `checkSettlement` 后再下发 SPI 回调，避免启动窗口内并发触碰同一批静态数据结构导致 mutex 失败。`extern "C" localctp_wait_until_ready()` 作为转发入口，由 [`wrapper/src/TraderApi.cpp`](../wrapper/src/TraderApi.cpp) 在 `CTP_RS_LOCALCTP` 编译期开关下调用
  - **移除 sqlite/shell.c**：上游附带的 SQLite CLI shell 在 LocalCTP 作为库构建时不需要，且会引入对终端 IO 的额外依赖（commit 7ae1880）

## 历史版本

- N/A
