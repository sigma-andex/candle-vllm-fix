
#pragma once
#include "rust/cxx.h"
#include "candle-vllm-fix/src/main.rs.h"

void copy_block(rust::Vec<uint64_t> vec);
void copy_block2(rust::Vec<Shared> vec);
