#include "tokenizer_cxx.h"
#include <iostream>
#include <string>

int main() {
    auto tokenizer = make_tokenizer("tokenizer.json");

    std::string input = "안녕하세요!!! ㅁㄴㅇㅁㄴㅇㅁㄴㅇ asdas hell world";
    auto result = tokenizer->encode(input);

    std::cout << "✅ Token IDs: ";
    for (auto id : result.ids)
        std::cout << id << " ";
    std::cout << "\n";

    std::cout << "✅ Token : ";
    for (auto tokens : result.tokens)
        std::cout << tokens.c_str() << " ";

    std::cout << "🎯 Attention Mask: ";
    for (auto mask : result.attention_mask)
        std::cout << mask << " ";
    std::cout << "\n";

    return 0;
}
