#include "tokenizer_cxx.h"
#include <iostream>
#include <string>

int main() {
    auto tokenizer = make_tokenizer("tokenizer.json");

    std::string input = "안녕하세요!!! ㅁㄴㅇㅁㄴㅇㅁㄴㅇ asdas hell world";
    auto result = tokenizer->encode(input, true);

    std::cout << "✅ Token IDs: ";
    for (auto id : result.ids)
        std::cout << id << " ";
    std::cout << "\n";

    std::cout << "✅ Tokens: ";
    for (auto token : result.tokens)
        std::cout << token.c_str() << " ";
    std::cout << "\n";

    std::cout << "🎯 Attention Mask: ";
    for (auto mask : result.attention_mask)
        std::cout << mask << " ";
    std::cout << "\n";

    std::string decoded = tokenizer->decode(result.ids, true)
        .c_str();
    std::cout << "🔁 Decoded Text: " << decoded << "\n";

    return 0;
}
