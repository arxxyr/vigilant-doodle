# UI 渲染问题诊断报告

## 问题描述

**症状**：
- 主菜单 UI 完全不显示（黑屏）
- 日志显示所有 UI 元素已创建
- 按钮可以点击（有交互响应）但不可见

## 根本原因

**Cargo.toml 配置错误**：`default-features = false` 禁用了 Bevy 的所有默认功能，导致 UI 渲染后端未启用。

```toml
# ❌ 错误配置
bevy = { version = "0.17.2", default-features = false, features = [
    "bevy_ui",           # UI 系统（仅定义组件）
    "bevy_text",         # 文本系统（仅定义组件）
    # 缺少渲染后端！
] }
```

## 技术分析

### Bevy 渲染架构

Bevy 采用分层架构：
1. **组件定义层**：`bevy_ui`, `bevy_text` 定义 UI 组件和 ECS 结构
2. **渲染后端层**：`bevy_ui_render`, `bevy_sprite_render` 负责实际渲染
3. **核心渲染**：`bevy_render`, `bevy_core_pipeline` 提供基础渲染能力

**关键发现**：
- `bevy_ui` 只提供组件定义（`Node`, `Button` 等）
- `bevy_ui_render` 才是真正的 UI 渲染系统
- `default-features = false` 会禁用所有渲染后端

### 为什么按钮可以点击但不可见？

- **交互系统**（`bevy_picking`）在 `bevy_render` 中，未被禁用
- **渲染系统**（`bevy_ui_render`）被禁用
- 结果：按钮碰撞检测正常，但渲染管线不工作

## 解决方案

### 1. 启用完整的 UI 渲染堆栈

```toml
bevy = { version = "0.17.2", default-features = false, features = [
    # 核心系统
    "std",
    "async_executor",
    "bevy_state",
    "bevy_log",

    # 渲染核心
    "bevy_render",
    "bevy_core_pipeline",
    "bevy_camera",
    "bevy_light",
    "bevy_mesh",
    "bevy_shader",
    "bevy_image",
    "bevy_color",

    # 2D/3D 渲染
    "bevy_pbr",
    "bevy_sprite",
    "bevy_sprite_render",      # ← 关键：2D 渲染后端

    # UI 系统
    "bevy_ui",
    "bevy_ui_render",          # ← 关键：UI 渲染后端
    "bevy_text",
    "default_font",

    # 输入与窗口
    "bevy_window",
    "bevy_winit",
    "bevy_input_focus",

    # 资源与场景
    "bevy_asset",
    "bevy_scene",

    # 动画
    "animation",

    # 平台支持
    "x11",
    "wayland",
    "png",
    "zstd_rust",               # ← 关键：图像压缩后端

    # 其他
    "multi_threaded",
    "tonemapping_luts",
    "smaa_luts",
] }
```

### 2. 修复字体乱码

**原因**：FiraSans 字体不支持 CJK（中日韩）字符

**临时解决方案**：改为英文菜单
```rust
Text::new("Play")      // 而非 "开始游戏"
Text::new("Settings")  // 而非 "设置"
Text::new("Quit")      // 而非 "退出"
```

**永久解决方案**（可选）：
1. 下载支持中文的字体（如 Noto Sans SC）
2. 放入 `assets/fonts/` 目录
3. 在 `GameAssets` 中加载

## 诊断步骤回顾

1. **初步排查**：
   - 添加 `IsDefaultUiCamera` 组件 ✗
   - 更改 API 调用方式 ✗
   - 添加 bevy-inspector-egui ✗

2. **架构测试**：
   - 创建独立测试（无 dylib）✗
   - 排除架构问题

3. **搜索已知问题**：
   - 发现 Bevy 0.17 中 UI 渲染需要明确启用后端
   - 对比 default features 列表

4. **根本原因**：
   - 发现 `default-features = false`
   - 添加缺失的渲染后端 features ✓

## 验证结果

✅ **成功指标**：
- UI 完全可见（标题 + 3 个按钮）
- 按钮交互正常（hover 高亮，点击响应）
- 文本清晰显示（英文）
- FPS 稳定在 ~160 帧
- Workspace 架构正常工作（launcher + game dylib）

## 经验总结

### ⚠️ 注意事项

1. **永远不要盲目使用 `default-features = false`**
   - Bevy 的默认 features 是精心设计的完整功能集
   - 如需排除某些功能，应明确列出所有需要的 features

2. **渲染系统是分层的**
   - 组件定义 ≠ 渲染能力
   - 必须同时启用 `bevy_ui` 和 `bevy_ui_render`

3. **字体支持有限制**
   - 西文字体（FiraSans）不支持 CJK
   - 需要专门的 CJK 字体（Noto Sans CJK, Source Han Sans 等）

### 📋 检查清单

修复 Bevy UI 不显示问题：
- [ ] 确认 `bevy_ui_render` 已启用
- [ ] 确认 `bevy_sprite_render` 已启用（如果使用 2D sprite）
- [ ] 确认 `bevy_core_pipeline` 已启用
- [ ] 确认 `zstd_rust` 或 `zstd_c` 已启用（图像加载）
- [ ] 确认至少有一个 `Camera2d` 或 `Camera3d` 实体
- [ ] 确认相机有 `IsDefaultUiCamera` 组件（对于 UI）
- [ ] 确认字体文件存在且支持目标字符集

## 相关链接

- [Bevy 0.17.2 Default Features](https://github.com/bevyengine/bevy/blob/v0.17.2/Cargo.toml)
- [Bevy UI Rendering Architecture](https://bevyengine.org/learn/book/getting-started/ui/)
- [Bevy Migration Guide 0.16 to 0.17](https://bevyengine.org/learn/migration-guides/0-16-to-0-17/)

---

**日期**：2025-10-24
**Bevy 版本**：0.17.2
**解决状态**：✅ 已完全解决
