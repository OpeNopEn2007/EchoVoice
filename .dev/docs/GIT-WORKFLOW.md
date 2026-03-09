# Git 工作流规范

> EchoVoice 项目开发流程

---

## 分支策略

### 主分支

| 分支 | 用途 | 保护 |
|------|------|------|
| `main` | 稳定版本，可发布 | ✅ 禁止直接推送 |
| `develop` | 开发集成（可选）| ✅ 需PR审查 |

### 功能分支

命名：`feature/<模块名>`

- `feature/audio` - 音频录制模块
- `feature/asr` - ASR识别模块
- `feature/llm` - LLM润色模块
- `feature/hotkey` - 全局快捷键模块
- `feature/tray` - 系统托盘模块

### 修复分支

命名：`fix/<问题描述>`

---

## 开发流程

```
┌─────────┐    ┌──────────┐    ┌──────────┐    ┌──────────┐    ┌─────────┐
│  需求   │ →  │ 设计文档 │ →  │ feature  │ →  │   PR     │ →  │  main   │
│ 分析    │    │ 评审     │    │ 分支开发 │    │ 代码审查 │    │ 合并    │
└─────────┘    └──────────┘    └──────────┘    └──────────┘    └─────────┘
     │               │                │               │              │
     ▼               ▼                ▼               ▼              ▼
  飞书记录      文档沉淀          单元测试        CI检查通过      自动部署
```

---

## 详细步骤

### 1. 需求分析

- 在飞书文档记录需求
- 明确接口契约
- 定义验收标准

### 2. 设计文档

- 编写 `.dev/modules/<module>/design.md`
- 编写 `.dev/modules/<module>/interface.md`
- 提交到main分支（文档先行）

### 3. 创建功能分支

```bash
# 从main分支最新代码开始
git checkout main
git pull origin main

# 创建功能分支
git checkout -b feature/audio

# 推送到远程
git push -u origin feature/audio
```

### 4. 开发实现

- 按照设计文档编码
- 编写单元测试
- 本地验证通过

### 5. 提交代码

```bash
# 频繁小提交
git add .
git commit -m "feat(audio): implement recording start/stop

- Add AudioRecorder struct
- Implement start() with cpal
- Implement stop() returning Vec<f32>
- Add error handling for device errors

Refs: #123"
```

**提交信息规范**：

```
<type>(<scope>): <subject>

<body>

<footer>
```

**类型**：
- `feat`: 新功能
- `fix`: Bug修复
- `docs`: 文档更新
- `refactor`: 重构
- `test`: 测试
- `chore`: 构建/工具

### 6. 保持同步

```bash
# 定期同步main分支变更
git fetch origin
git rebase origin/main

# 如有冲突，解决后继续
git rebase --continue
```

### 7. 发起PR

```bash
# 推送最终代码
git push origin feature/audio

# 创建PR（使用gh CLI或Web界面）
gh pr create \
  --title "feat(audio): implement audio recording module" \
  --body "## 变更\n- 实现录音功能\n- 添加错误处理\n\n## 测试\n- [x] 单元测试通过\n- [x] 本地手动测试\n\n## 关联文档\n- .dev/modules/audio/design.md\n- .dev/modules/audio/interface.md"
```

### 8. 代码审查

**审查清单**：
- [ ] 符合设计文档
- [ ] 代码风格一致
- [ ] 单元测试覆盖
- [ ] 无安全漏洞
- [ ] 性能达标

**审查人**：至少1人批准

### 9. 合并到main

```bash
# Squash合并，保持历史整洁
git checkout main
git merge --squash feature/audio
git commit -m "feat(audio): add audio recording module (#456)"

# 删除功能分支
git branch -d feature/audio
git push origin --delete feature/audio
```

---

## 提交信息模板

```
<type>(<module>): <简短描述>

<详细说明>

<变更原因>
<技术细节>
<注意事项>

Refs: <issue编号>
Closes: <PR编号>
```

**示例**：

```
feat(asr): integrate whisper.cpp for speech recognition

- Add WhisperASR struct wrapping whisper.cpp
- Implement lazy model loading
- Support Chinese and English transcription
- Add model download helper

The integration uses the Rust bindings from
whisper-rs crate. Model is loaded on first use
to reduce startup time.

Performance: ~0.5x real-time on M2 MacBook

Refs: #78
```

---

## 版本标签

```bash
# 发布版本
git tag -a v0.1.0 -m "First alpha release"
git push origin v0.1.0
```

**版本号规则**：SemVer (MAJOR.MINOR.PATCH)

---

## CI/CD集成

### GitHub Actions

```yaml
# .github/workflows/ci.yml
name: CI
on: [push, pull_request]
jobs:
  test:
    runs-on: ubuntu-latest
    steps:
      - uses: actions/checkout@v3
      - name: Test
        run: cargo test
      - name: Lint
        run: cargo clippy -- -D warnings
      - name: Format check
        run: cargo fmt -- --check
```

---

## 回退策略

**发现问题**：
1. 在feature分支：直接修复，重新PR
2. 在main分支：
   - 紧急修复：`git revert <commit>`
   - 创建hotfix分支

---

*文档版本: 0.1.0*
*最后更新: 2026-03-09*
