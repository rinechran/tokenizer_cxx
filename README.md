
# 🦀 tokenizer_cxx

**`tokenizer_cxx`** is a Rust-powered tokenizer bridge for C++ built using [Hugging Face Tokenizers](https://github.com/huggingface/tokenizers) and the [`cxx`](https://cxx.rs) crate.

It allows you to load a tokenizer in Rust (from a `tokenizer.json` file) and use it directly from C++ with a simple and safe API.

---

## ✨ Features

- ✅ Powered by Hugging Face `tokenizers` (Rust)
- ✅ C++ access to `make_tokenizer()` and `.encode()`
- ✅ Returns token IDs, token strings, and attention masks

---

## 🛠 Requirements

- Rust ≥ 1.65
- CMake ≥ 3.15
- `tokenizer.json` (pre-trained tokenizer from HuggingFace)

---

## 🧪 Example Usage (C++)
```
    #include "tokenizer_cxx.h"
    #include <iostream>
    #include <string>

    int main() {
        auto tokenizer = make_tokenizer("tokenizer.json");
        if (!tokenizer) {
            std::cerr << "❌ Failed to load tokenizer\n";
            return 1;
        }

        std::string input = "안녕하세요!!! ㅁㄴㅇㅁㄴㅇㅁㄴㅇ asdas hell world";
        auto result = tokenizer->encode(input);

        std::cout << "✅ Token IDs: ";
        for (auto id : result.ids)
            std::cout << id << " ";
        std::cout << "\n";

        std::cout << "✅ Tokens: ";
        for (const auto& token : result.tokens)
            std::cout << token.c_str() << " ";
        std::cout << "\n";

        std::cout << "🎯 Attention Mask: ";
        for (auto mask : result.attention_mask)
            std::cout << mask << " ";
        std::cout << "\n";

        return 0;
    }
```
---

## 🧱 Build Instructions

```bash
# Clone this project
git clone https://github.com/yourname/tokenizer_cxx.git
cd tokenizer_cxx

# Make sure tokenizer.json is in the root directory

# Create build directory
mkdir build && cd build

# Configure with CMake (LTO optional)
cmake -DENABLE_LTO=ON ..

# Build the project
cmake --build .
````

---

## 📁 Project Structure

```
tokenizer_cxx/
├── tokenizer_cxx/         # Rust crate with cxx bridge
│   ├── src/lib.rs
│   └── Cargo.toml
├── main.cpp               # Example C++ usage
├── CMakeLists.txt         # Top-level CMake build
└── tokenizer.json         # Tokenizer model file (not included)
```

---

## 📦 Output

After building, you'll get an executable `main` that prints:

* Token IDs
* Token strings
* Attention masks

---

## 📝 License

MIT