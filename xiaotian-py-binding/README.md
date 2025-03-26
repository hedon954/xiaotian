# xiaotian-py-binding

This project serves as the Python binding for `xiaotian`, with the aim of leveraging `Gradio` for frontend development.

```bash
cd xiaotian-py-binding
rye sync
rye install maturin
rye add --dev pip
rye add --dev ipython
maturin develop
rye run ipython
```
