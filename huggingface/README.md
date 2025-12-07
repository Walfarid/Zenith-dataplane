---
title: Zenith AI
emoji: 🚀
colorFrom: blue
colorTo: purple
sdk: gradio
sdk_version: 4.0.0
app_file: app.py
pinned: true
license: apache-2.0
tags:
  - data-loading
  - pytorch
  - tensorflow
  - rust
  - high-performance
  - machine-learning
  - dataloader
---

# 🚀 Zenith AI

> **"Stop Starving Your GPUs. Feed Them with Zenith."**

**Zenith AI** is a high-performance data loading and preprocessing library designed to accelerate AI/ML training pipelines.

## ⚡ Performance

| Metric | Value |
|--------|-------|
| **Throughput** | 6,000,000+ events/sec |
| **Latency** | < 100µs |
| **Speedup** | 10-120x vs standard loaders |

## 📦 Installation

```bash
pip install zenith-ai
```

## 🔥 Quick Start

### PyTorch
```python
import zenith.torch as zt

loader = zt.DataLoader(
    source="path/to/data",
    batch_size=64,
    preprocessing_plugin="resize.wasm"
)

for batch in loader:
    model.train_step(batch)
```

### TensorFlow
```python
import zenith.tensorflow as ztf

dataset = ztf.ZenithDataset("path/to/data")
dataset = dataset.batch(32).prefetch(tf.data.AUTOTUNE)
model.fit(dataset)
```

## 🛠️ Features

- **Rust Core**: Native performance without Python GIL
- **Zero-Copy**: Apache Arrow integration
- **WASM Plugins**: Secure preprocessing
- **Framework Support**: PyTorch, TensorFlow, JAX

## 🔗 Links

- [GitHub](https://github.com/vibeswithkk/Zenith-dataplane)
- [PyPI](https://pypi.org/project/zenith-ai/)
- [Documentation](https://github.com/vibeswithkk/Zenith-dataplane#documentation)

## 👨‍💻 Author

**Wahyu Ardiansyah** 🇮🇩

Made with ❤️ in Indonesia
