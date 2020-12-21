# 23. Web 编程

Web 编程实践涵盖提取链接、URL、媒体类型（MIME）、电子邮件、客户端、请求处理、Web API 调用、下载。

提取链接实践：
- 从 HTML 网页中提取所有链接
- 检查网页死链
- 从 MediaWiki 标记页面提取所有唯一性链接
- 提取链接 crates 小结

URL 实践：
- 解析 URL 字符串为 `Url` 类型
- 通过移除路径段创建基本 URL
- 从基本 URL 创建新 URLs
- 提取 URL 源（scheme/ host/ port）
- 从 URL 移除片段标识符和查询对
- URL crates 小结

媒体类型（MIME）实践：
- 从字符串获取 MIME 类型
- 从文件名获取 MIME 类型
- 解析 HTTP 响应的 MIME 类型
- 媒体类型（MIME）crates 小结

电子邮件实践。

客户端实践：
- 请求处理
    - 发出 HTTP GET 请求
    - 为 REST 请求设置自定义消息标头和 URL 参数
- Web API 调用
    - 查询 GitHub API
    - 检查 API 资源是否存在
    - 使用 GitHub API 创建和删除 Gist
    - 使用 RESTful API 分页
    - 处理速率受限 API
- 下载
    - 下载文件到临时目录
    - 使用 HTTP range 请求头进行部分下载
    - POST 文件到 paste-rs
    - 客户端 crates 小结
