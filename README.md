## ⚠️ **WARNING: This Library is UNSAFE BY DESIGN** ⚠️

# 🌺 SubStd 🌺

*High-Performance Core Replacement Library*

Welcome to SubStd - an experimental, performance-focused alternative implementation of Rust's std/core modules. This library prioritizes raw speed over safety guarantees!

It does not meet typical Rust promises of safety, so is a little sub-standard.

## 💫 Overview

SubStd provides blazing-fast alternatives to common std/core functionality by:

-   Removing runtime checks and bounds validation
-   Using unsafe optimizations aggressively
-   Providing zero-cost abstractions
-   Minimizing memory operations
-   Work around additional checks with it's own calls to externs.
-   Performs a valid action or aborts the process

## 🎯 Goals

-   Maximum performance over safety
-   Drop-in replacement for std/core modules
-   Minimal runtime overhead
-   Platform-specific optimizations

## ✨ Key Features

-   Unchecked operations for Vec, String, etc
-   Lock-free concurrent structures
-   SIMD-accelerated algorithms
-   Zero-allocation APIs where possible
-   Platform-specific assembly optimizations
-   Dumb enough to do what you tell it
-   Smart enough to tell you what caused panic in production

## 🗺️ Roadmap

### Future Goals

-   Since I'm still scaffolding, placed the first 2 stream handlers
-   Using kernel32, a console handler on windows
-   Using a boolean reference, and a static table, print to an led in morse code
-   This isn't a mistake
-   Part of the beauty of a rudimentary, fast, simple design is dexterity

### Phase 1

-   Really, I'm still laying all the groundwork
-   Yes, I'm using AI to help with some of the scaffolding
-   No, I don't trust AI to get it right when you're working this unsafely
-   Everything it writes simply gives me keywords to narrow down docs and do it my way quickly

## ⚡️ Performance

Performance is our #1 priority! Expect:

-   Significantly faster than std for common operations
-   Works on any platform with rustc
-   #![no_std] compliant!
-   It's only error contol is to panic in production

## 🚫 Limitations

-   Safety guarantees are minimal, it does as told
-   Not recommended for safety-critical systems
-   Requires careful usage and understanding
-   May cause undefined behavior if misused
-   By design, may panic in production

## 🌸 Contributing

This is a work in progress! Contributors welcome, but please note:

-   Safety is secondary to performance
-   Design requirements always require 'what caused a panic'
-   Breaking changes are expected

## 💝 License

Licensed under the Apache License, Version 2.0 (the "License");
you may not use this file except in compliance with the License.
You may obtain a copy of the License at

    http://www.apache.org/licenses/LICENSE-2.0

Unless required by applicable law or agreed to in writing, software
distributed under the License is distributed on an "AS IS" BASIS,
WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
See the License for the specific language governing permissions and
limitations under the License.
