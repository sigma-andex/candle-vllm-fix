

#include "candle-vllm-fix/src/cache_kernel.h"
#include "candle-vllm-fix/src/main.rs.h"
#include <algorithm>
#include <functional>
#include <set>
#include <string>
#include <vector>
#include <unordered_map>
#include <iostream>
#include "candle-vllm-fix/src/main.rs.h"

void copy_block(rust::Vec<uint64_t> vec)
{
    for (auto i : vec)
    {
        std::cout << i << std::endl;
    }
}

void copy_block2(rust::Vec<Shared> vec)
{

    for (auto s : vec)
    {
        std::cout << s.k << "->" << s.v << std::endl;
    }
}
